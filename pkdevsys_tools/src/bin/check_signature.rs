/*************************************************************************
* ph0llux:171e160fd8926e3780587b210c146074c52a9b7f0699d3043a83f03b4cd30bc7
*************************************************************************/
// - STD
use std::env;

// - internal
use pkdevsys_tools;
use pkdevsys_tools::{CustomError};

// - external
use phollaits::{StringExt};

fn main() -> Result<(), CustomError> {
	let args: Vec<String> = env::args().collect();
	// handle the arguments
	if args.len() > 2 {
		println!("{}", pkdevsys_tools::HELP_USAGE_CHECK_SIGNATURE);
		return Err(CustomError::Arguments(pkdevsys_tools::ERROR_TOO_MANY_ARGUMENTS.to_string()));
	} else if args.len() <= 1 {
		println!("{}", pkdevsys_tools::HELP_USAGE_CHECK_SIGNATURE);
		return Err(CustomError::Arguments(pkdevsys_tools::ERROR_TOO_FEW_ARGUMENTS.to_string()));
	}
	let filename = &args[1];
	let pkdevsys_config = pkdevsys_tools::get_config(pkdevsys_tools::PATH_TO_CONFIG_FILE.to_string().shellexpand())?;
	let filetype = pkdevsys_tools::get_filetype(filename, &pkdevsys_config);
	let filecontent = pkdevsys_tools::get_filecontent(filename)?;


	Ok(())
}