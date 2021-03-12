/*************************************************************************
* ph0llux:d398a78f74c5a81d8521bbdcfce72be10e88cc591d62061cf2b84bf44035ba1c
*************************************************************************/
// - STD
use std::env;

// - internal
use pkdevsys_tools::CustomError;
use code_sign::code_sign;

// - modules
mod code_sign;


fn main() -> Result<(), CustomError> {
	let args: Vec<String> = env::args().collect();
	if args.len() > 2 {
		return Err(CustomError::Arguments(pkdevsys_tools::ERROR_TOO_MANY_ARGUMENTS.to_string()));
	} else if args.len() <= 1 {
		return Err(CustomError::Arguments(pkdevsys_tools::ERROR_TOO_FEW_ARGUMENTS.to_string()));
	}
	let filename = &args[1];
	let hash = code_sign(filename)?;
	println!("Set Signature for {} : {}", filename, hash);
	Ok(())
}