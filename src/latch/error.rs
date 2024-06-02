use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum LatchError {
	InvalidInput
}

impl fmt::Display for LatchError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			LatchError::InvalidInput => write!(f, "Invalid latch input combination."),
		}
	}
}

impl std::error::Error for LatchError {}
