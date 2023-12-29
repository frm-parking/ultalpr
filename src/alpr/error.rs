use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigCastError {
	#[error(transparent)]
	Json(#[from] serde_json::Error),

	#[error(transparent)]
	Ccast(#[from] std::ffi::NulError),
}

#[derive(Debug, Error)]
pub enum ProcessError {
	#[error(transparent)]
	Utf8(#[from] std::str::Utf8Error),

	#[error(transparent)]
	De(#[from] serde_json::Error),
}
