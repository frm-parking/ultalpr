use std::thread;

use async_channel::{bounded, Receiver, Sender};
use image::DynamicImage;

use crate::{engine, Config, Plate, ProcessResult, Result, UltalprError};

#[derive(Debug)]
pub struct AlprService {
	tx: Sender<DynamicImage>,
	rx: Receiver<ProcessResult>,
}

impl AlprService {
	pub fn start(config: Config) -> Result<Self> {
		let thread = build_thread();

		let (req_tx, req_rx) = bounded(1);
		let (res_tx, res_rx) = bounded(1);

		let this = Self { tx: req_tx, rx: res_rx };

		thread.spawn(move || -> Result<()> {
			let _defer = engine::init(config)?;

			while let Ok(image) = req_rx.recv_blocking() {
				let result = engine::process_image(image)?;
				let _ = res_tx.send_blocking(result);
			}

			Ok(())
		})?;

		Ok(this)
	}

	pub async fn recognize(&self, image: DynamicImage) -> Result<Option<Plate>> {
		self.tx.send(image).await.map_err(|_| UltalprError::SendError)?;
		let plate = self.rx.recv().await.map(|res| res.plates.into_iter().next())?;

		Ok(plate)
	}
}

fn build_thread() -> thread::Builder {
	thread::Builder::new().name("alpr".into())
}
