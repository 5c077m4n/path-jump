use std::{error, fmt, io};

#[derive(Debug)]
pub enum ErrorType {
	Io(io::Error),
	Rusqlite(rusqlite::Error),
}
impl error::Error for ErrorType {}

impl From<io::Error> for ErrorType {
	fn from(err: io::Error) -> Self {
		ErrorType::Io(err)
	}
}
impl From<rusqlite::Error> for ErrorType {
	fn from(err: rusqlite::Error) -> Self {
		ErrorType::Rusqlite(err)
	}
}

impl fmt::Display for ErrorType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			Self::Io(io_error) => write!(f, "{}", io_error),
			Self::Rusqlite(rusqlite_error) => write!(f, "{}", rusqlite_error),
		}
	}
}
