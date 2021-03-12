/*************************************************************************
* ph0llux:c64842cc079a6b9c5538353b68d9b1e74a8a73983b5da2c4b21231e1b69e7179
*************************************************************************/
#![forbid(unsafe_code)] // unsafe is forbidden
#![cfg_attr(test, deny(warnings))] // deny warnings
// - STD
use std::io;
use std::fs;

// - external
use serde::{Deserialize};
use toml::de::Error as TomlError;

// - internal
use crate::traits::CustomErrorTrait;

// - modules
pub mod traits;

#[derive(Deserialize,Clone)]
pub struct Config {
    pub general: SubconfigGeneral,
    pub syntax_c: SubconfigSyntaxC,
    pub syntax_hashtag: SubconfigSyntaxHashtag,
}

impl Config {
	pub fn get_generic_ownerline(&self, filetype: &Filetype) -> String {
		let generic_line = match filetype {
			Filetype::SyntaxC => C_SYNTAX_COMMENT_OWNER_LINE,
			Filetype::SyntaxHashtag => HASHTAG_SYNTAX_COMMENT_OWNER_LINE,
		};
		generic_line.replace(VAR_OWNER, &self.general.author)
	}
	pub fn get_full_comment_line<S: Into<String>>(&self, filetype: &Filetype, key: S) -> String {
		let key = key.into();
		let mut comment_line = String::new();
		match filetype {
			Filetype::SyntaxC => comment_line.push_str(C_SYNTAX_COMMENT_LINE_START),
			Filetype::SyntaxHashtag => comment_line.push_str(HASHTAG_SYNTAX_COMMENT_LINE_START)
		};
		comment_line.push_str(SEPARATOR_NEWLINE);
		comment_line.push_str(&self.get_generic_ownerline(filetype));
		comment_line.push_str(&key);
		comment_line.push_str(SEPARATOR_NEWLINE);
		match filetype {
			Filetype::SyntaxC => comment_line.push_str(C_SYNTAX_COMMENT_LINE_END),
			Filetype::SyntaxHashtag => comment_line.push_str(HASHTAG_SYNTAX_COMMENT_LINE_END)
		};
		comment_line.push_str(SEPARATOR_NEWLINE);
		comment_line
	}
}

#[derive(Deserialize,Clone)]
pub struct SubconfigGeneral {
	pub author: String,
	pub email: String,
	pub sign_key: String,
}

#[derive(Deserialize,Clone)]
pub struct SubconfigSyntaxC {
	pub file_extensions: Vec<String>,
}

#[derive(Deserialize,Clone)]
pub struct SubconfigSyntaxHashtag {
	pub file_extensions: Vec<String>,
}

#[derive(Clone,Debug)]
pub enum Filetype {
	SyntaxC,
	SyntaxHashtag
}

#[derive(Debug)]
pub enum CustomError {
	IoError(io::Error),
	Arguments(String),
	ReadConfig(io::Error),
	ReadFile(io::Error),
	ReadToml(TomlError),
	Filetype,
	OptionNone
}

pub fn get_config<S: Into<String>>(path_to_config: S) -> Result<Config, CustomError> {
	let path_to_config = path_to_config.into();
	let toml_content = match fs::read_to_string(path_to_config) {
		Ok(x) => x,
		Err(err) => return Err(CustomError::ReadConfig(err)),
	};
	let config: Config = toml::from_str(&toml_content).to_ce()?;
	Ok(config)
}

//C or Rust syntax styled comment styles
pub const C_SYNTAX_COMMENT_LINE_START: &'static str = "/*************************************************************************";
pub const C_SYNTAX_COMMENT_LINE_END: &'static str = "*************************************************************************/";
pub const C_SYNTAX_COMMENT_OWNER_LINE: &'static str = "* $OWNER:";

//Hashtag syntax styled comment styles
pub const HASHTAG_SYNTAX_COMMENT_LINE_START: &'static str = "##########################################################################";
pub const HASHTAG_SYNTAX_COMMENT_LINE_END: &'static str = "##########################################################################";
pub const HASHTAG_SYNTAX_COMMENT_OWNER_LINE: &'static str = "# $OWNER:";
pub const HASHTAG_SYNTAX_SHEBANG_STARTLINE: &'static str = "#!";


//Error consts
pub const ERROR_TOO_MANY_ARGUMENTS: &'static str = "Too many arguments";
pub const ERROR_TOO_FEW_ARGUMENTS: &'static str = "Too few arguments";

//Help messages
pub const HELP_USAGE_CODE_SIGN: &'static str = "
usage:
	./code_sign <path/to/codefile.rs>
";
pub const HELP_USAGE_FOLDER_CODE_SIGN: &'static str = "
usage:
	./folder_code_sign <path/to/codefiles/>
";


//SEPARATORS
pub const SEPARATOR_POINT: char = '.';
pub const SEPARATOR_NEWLINE: &'static str = "\n";
pub const SEPARATOR_EMPTYSTRING: &'static str = "";

//Other
pub const PATH_TO_CONFIG_FILE: &'static str = "~/.config/pkdevsys_config.toml";

//vars
pub const VAR_OWNER: &'static str = "$OWNER";