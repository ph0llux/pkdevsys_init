/*************************************************************************
* ph0llux:c180c7363f6eb15e1de1ea413e4d9641a959c3748cab3cb152a4ce4f9a5fca29
*************************************************************************/
// - STD
use std::env;


// - internal
use pkdevsys_tools;
use pkdevsys_tools::{CustomError};
use code_sign::code_sign;

// - modules
mod code_sign;

// - external
use walkdir::WalkDir;
use phollaits::StringExt;

fn main() -> Result<(), CustomError> {
	let args: Vec<String> = env::args().collect();
	// handle the arguments
	if args.len() > 2 {
		println!("{}", pkdevsys_tools::HELP_USAGE_FOLDER_CODE_SIGN);
		return Err(CustomError::Arguments(pkdevsys_tools::ERROR_TOO_MANY_ARGUMENTS.to_string()));
	} else if args.len() <= 1 {
		println!("{}", pkdevsys_tools::HELP_USAGE_FOLDER_CODE_SIGN);
		return Err(CustomError::Arguments(pkdevsys_tools::ERROR_TOO_FEW_ARGUMENTS.to_string()));
	}

	let folder = &args[1];
	let pkdevsys_config = pkdevsys_tools::get_config(pkdevsys_tools::PATH_TO_CONFIG_FILE.to_string().shellexpand())?;
	for entry in WalkDir::new(folder).into_iter().filter_map(|x| x.ok()) {
		if !entry.file_type().is_file() {
			continue;
		}
		let path = match entry.path().to_str() {
			None => continue,
			Some(x) => x,
		};
		match path.rsplit(pkdevsys_tools::SEPARATOR_POINT).next() {
			None => continue,
			Some(x) => {
				if !pkdevsys_config.syntax_c.file_extensions.contains(&String::from(x)) && 
				!pkdevsys_config.syntax_hashtag.file_extensions.contains(&String::from(x)) {
					continue;
				}
			}
		}
		let hash = code_sign(path)?;
		println!("Set Signature for {} : {}", path, hash);
	}
	Ok(())
}