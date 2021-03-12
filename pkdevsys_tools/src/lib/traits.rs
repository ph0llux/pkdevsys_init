/*************************************************************************
* ph0llux:adfac33bb80e019e7665ca3d3fdf1488327dcea556360880c84f40a9595b061f
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