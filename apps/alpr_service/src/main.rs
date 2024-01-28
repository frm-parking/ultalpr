// (FRM SCAM WARNING)
// Если вы видите это сообщение, значит проект находится
// в активной разработке или код этого проекта
// распространяется и используется без согласия автора
//
// (c) Danil Karpenko, 2024
// limpix31@gmail.com, @LIMPIX31

use std::sync::Arc;
use anyhow::Result;
use restream_core::{RestreamCapture, RestreamOpts, SessionGroup};
use tracing::{error, info};

pub use alpr::alpr_server;
use crate::config::{Config, ConfigPath};
use crate::recognizer::Recognizer;

mod alpr;
mod config;
mod service;
mod trc;
mod recognizer;

#[tokio::main]
async fn main() -> Result<()> {
	trc::setup()?;

	let config = Config::read_from_file(ConfigPath::default()).await?;

	let mut recog = Recognizer::init(&config.alpr)?;
	let group = Arc::new(SessionGroup::default());

	let cap = RestreamCapture::describe(RestreamOpts {
		group,
		url: "rtsp://in1.miackovodisp.keenetic.pro/h265_2".parse()?,
		creds: Some(("admin", "SysAdmin").into()),
	}).await?;

	let mut player = cap.play().await?;

	loop {
		if let Ok(Some(frame)) = player.next().await {
			if let Some(vrm) = recog.process(frame).await? {
				info!("Recognized: {}", vrm.vrm);
			} else {
				info!("No VRMs");
			}
		}
	}

	Ok(())
}
