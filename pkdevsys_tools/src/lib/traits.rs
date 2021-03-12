/*************************************************************************
* ph0llux:2d20de05692c3f655c987e875fd4a0617e62832e3432f40479127f448b2cbe44
*************************************************************************/
use super::*;

pub trait CustomErrorTrait<T> {
	fn to_ce(self) -> Result<T, CustomError>;
}

impl<T> CustomErrorTrait<T> for Result<T, TomlError> {
	fn to_ce(self) -> Result<T, CustomError> {
		match self {
			Ok(x) => Ok(x),
			Err(err) => Err(CustomError::ReadToml(err))
		}
	}
}

impl<T> CustomErrorTrait<T> for Result<T, TomlSerError> {
	fn to_ce(self) -> Result<T, CustomError> {
		match self {
			Ok(x) => Ok(x),
			Err(err) => Err(CustomError::WriteToml(err))
		}
	}
}

impl<T> CustomErrorTrait<T> for Result<T, io::Error> {
	fn to_ce(self) -> Result<T, CustomError> {
		match self {
			Ok(x) => Ok(x),
			Err(err) => Err(CustomError::IoError(err))
		}
	}
}

impl<T> CustomErrorTrait<T> for Option<T> {
	fn to_ce(self) -> Result<T, CustomError> {
		match self {
			Some(x) => Ok(x),
			None => Err(CustomError::OptionNone)
		}
	}
}