use crate::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub enum Error {
	Io,
	SendError,
	Redis,
}

impl From<std::io::Error> for Error {
	#[inline(always)]
	fn from(_: std::io::Error) -> Self {
		Self::Io
	}
}

impl From<redis::RedisError> for Error {
	#[inline(always)]
	fn from(_: redis::RedisError) -> Self {
		Self::Redis
	}
}

impl From<tokio::sync::mpsc::error::SendError<Message>> for Error {
	#[inline(always)]
	fn from(_: tokio::sync::mpsc::error::SendError<Message>) -> Self {
		Self::SendError
	}
}
