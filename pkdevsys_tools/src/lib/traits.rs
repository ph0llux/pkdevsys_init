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

pub trait StringExtensions {
	fn trim_newline_end(&self) -> String;
}

impl StringExtensions for String {
	fn trim_newline_end(&self) -> String {
		let mut trimmed_string = self.clone();
		loop {
			if trimmed_string.ends_with('\n') {
				trimmed_string.pop();
			} else if trimmed_string.ends_with('\r') {
				trimmed_string.pop();
			} else {
				break;
			}
		}
	    String::from(trimmed_string)
	}
}