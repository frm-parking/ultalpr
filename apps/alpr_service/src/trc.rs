use anyhow::Result;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn setup() -> Result<()> {
	use tracing_subscriber as trsb;

	let tracing_fmt = trsb::fmt::layer().with_thread_ids(true);
	let tracing_env = {
		trsb::filter::EnvFilter::builder()
			.with_default_directive(LevelFilter::TRACE.into())
			.from_env()?
			.add_directive("tokio_util::codec=info".parse()?)
			.add_directive("retina=info".parse()?)
	};

	let tracing_registry = trsb::registry().with(tracing_fmt).with(tracing_env);

	tracing_registry.init();

	Ok(())
}
