/*************************************************************************
* ph0llux:d6fc0b70d74c4d1e3cc3558658fb8c96bf5911a32cfcc284feadab27328c2a2e
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