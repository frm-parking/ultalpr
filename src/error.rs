use thiserror::Error;

pub type Result<T, E = UltalprError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum UltalprError {
	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	Json(#[from] serde_json::Error),

	#[error(transparent)]
	Ccast(#[from] std::ffi::NulError),

	#[error(transparent)]
	Utf8(#[from] std::str::Utf8Error),

	#[cfg(feature = "async-service")]
	#[error(transparent)]
	RecvError(#[from] async_channel::RecvError),

	#[cfg(feature = "async-service")]
	#[error("Send error")]
	SendError,
}
