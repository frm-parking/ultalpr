use thiserror::Error;

#[derive(Debug, Error)]
pub enum UltalprError {
	#[error(transparent)]
	Json(#[from] serde_json::Error),

	#[error(transparent)]
	Ccast(#[from] std::ffi::NulError),

	#[error(transparent)]
	Join(#[from] tokio::task::JoinError),

	#[error(transparent)]
	Utf8(#[from] std::str::Utf8Error),
}
