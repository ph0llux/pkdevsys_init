/*************************************************************************
* ph0llux:50ec937ef7a775b69cbad323b94b69e8e709e89b89b40baf3ae1700268fa9606
*************************************************************************/
// - STD
use std::env;

// - internal
use pkdevsys_tools;
use pkdevsys_tools::CustomError;
use pkdevsys_tools::traits::{CustomErrorTrait};

fn main() -> Result<(), CustomError> {
	let args: Vec<String> = env::args().collect();
	// handle the arguments
	if args.len() > 4 {
		return Err(CustomError::Arguments(pkdevsys_tools::ERROR_TOO_MANY_ARGUMENTS.to_string()));
	} else if args.len() < 4 {
		return Err(CustomError::Arguments(pkdevsys_tools::ERROR_TOO_FEW_ARGUMENTS.to_string()));
	}

	let general_config = pkdevsys_tools::SubconfigGeneral {
		author: args[1].clone(),
		email: args[2].clone(),
		sign_key: args[3].clone()
	};
	let syntaxc_config = pkdevsys_tools::SubconfigSyntaxC {
		file_extensions: vec!(
			"rs".to_string(),
			"cpp".to_string(),
			"js".to_string(),
			"c".to_string(),
			"go".to_string()),
	};

	let syntaxhashtag_config = pkdevsys_tools::SubconfigSyntaxHashtag {
		file_extensions: vec!(
			"sh".to_string(),
			"toml".to_string(),
			"py".to_string())
	};

	let config = pkdevsys_tools::Config {
		general: general_config,
		syntax_c: syntaxc_config,
		syntax_hashtag: syntaxhashtag_config
	};

	let toml = toml::to_string(&config).to_ce()?;
	println!("{}", toml);

	Ok(())
}