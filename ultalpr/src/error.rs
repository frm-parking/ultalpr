use thiserror::Error;

#[derive(Debug, Error)]
pub enum InitError {
	#[error(transparent)]
	Json(#[from] serde_json::Error),

	#[error(transparent)]
	Ccast(#[from] std::ffi::NulError),

	#[error(transparent)]
	Join(#[from] tokio::task::JoinError),
}

#[derive(Debug, Error)]
pub enum ProcessError {
	#[error(transparent)]
	Utf8(#[from] std::str::Utf8Error),

	#[error(transparent)]
	De(#[from] serde_json::Error),

	#[error(transparent)]
	Join(#[from] tokio::task::JoinError),
}

#[derive(Debug, Error)]
pub enum UltalprError {
	#[error(transparent)]
	InitError(#[from] InitError),

	#[error(transparent)]
	ProcessError(#[from] ProcessError),
}
