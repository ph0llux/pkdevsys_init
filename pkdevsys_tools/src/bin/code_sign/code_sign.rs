/*************************************************************************
* ph0llux:1dd06ff9c5334caf51e859acf1c91ea04ac05b18bdb27050bf287428f2c7de86
*************************************************************************/
// - STD
use std::io::Write;
use std::fs;

// - internal
use pkdevsys_tools;
use pkdevsys_tools::{CustomError, get_config};
use pkdevsys_tools::traits::{CustomErrorTrait};

// - external
use phollaits::{StringExt,HashExt};

pub fn code_sign<S: Into<String>>(filename: S) -> Result<String, CustomError>{
	let filename = filename.into();
	let config = get_config(pkdevsys_tools::PATH_TO_CONFIG_FILE.to_string().shellexpand())?;
	let filetype = pkdevsys_tools::get_filetype(&filename, &config)?;
	let filecontent = pkdevsys_tools::get_filecontent(&filename)?;

	let owner_comment_line = {
		let mut comment_line = None;
		for line in filecontent.lines() {
			if line.starts_with(&config.clone().get_generic_ownerline(&filetype)) {
				comment_line = Some(line);
				break;
			}
		}
		comment_line
	};
	let mut shebang = false;
	let hash = match owner_comment_line {
		Some(_) => {
			let mut concatenated_content = String::new();
			let mut skip_value: usize = 3;
			let mut content_iterator = filecontent.lines();
			match content_iterator.next() {
				None => (),
				Some(x) => if x.starts_with(pkdevsys_tools::HASHTAG_SYNTAX_SHEBANG_STARTLINE) {
					shebang = true;
					match content_iterator.next() {
						None => (),
						Some(_) => skip_value = 2,
					}
				}
			}
			for line in filecontent.lines().skip(skip_value) {
				concatenated_content.push_str(line);
				concatenated_content.push_str(pkdevsys_tools::SEPARATOR_NEWLINE);
			}
			let mut concatenated_content = concatenated_content.trim_newline_end() + &config.general.sign_key;
			concatenated_content.sha256sum().to_ce()?
		},
		None => {
			let mut content_iterator = filecontent.lines();
			match content_iterator.next() {
				None => {
					let mut concatenated_content = filecontent.trim_newline_end() + &config.general.sign_key;
					concatenated_content.sha256sum().to_ce()?
				},
				Some(x) => if x.starts_with(pkdevsys_tools::HASHTAG_SYNTAX_SHEBANG_STARTLINE) {
					shebang = true;
					let mut concatenated_content = String::new();
					for line in content_iterator {
						concatenated_content.push_str(line);
						concatenated_content.push_str(pkdevsys_tools::SEPARATOR_NEWLINE);
					}
					let mut concatenated_content = concatenated_content.trim_newline_end() + &config.general.sign_key;
					concatenated_content.sha256sum().to_ce()?
				} else {
					let mut concatenated_content = filecontent.trim_newline_end() + &config.general.sign_key;
					concatenated_content.sha256sum().to_ce()?
				}
			}
		},
	};
	let mut new_content = String::new();
	match owner_comment_line {
		None => {
			if shebang {
				new_content.push_str(&filecontent.lines().next().unwrap());
				new_content.push_str(pkdevsys_tools::SEPARATOR_NEWLINE);
				new_content.push_str(&config.get_generic_ownerline(&filetype));
				new_content.push_str(&hash);
				new_content.push_str(pkdevsys_tools::SEPARATOR_NEWLINE);
				for line in filecontent.lines().skip(1) {
					new_content.push_str(line);
					new_content.push_str(pkdevsys_tools::SEPARATOR_NEWLINE);
				}		
			} else {
				new_content.push_str(&config.get_full_comment_line(&filetype, &hash));
				new_content.push_str(&filecontent);
			}	
		},
		Some(_) => {
			if shebang {
				new_content.push_str(&filecontent.lines().next().unwrap());
				new_content.push_str(pkdevsys_tools::SEPARATOR_NEWLINE);
				new_content.push_str(&config.get_generic_ownerline(&filetype));
				new_content.push_str(&hash);
				new_content.push_str(pkdevsys_tools::SEPARATOR_NEWLINE);
				for line in filecontent.lines().skip(2) {
					new_content.push_str(&line);
					new_content.push_str(pkdevsys_tools::SEPARATOR_NEWLINE);
				}
			} else {
				new_content.push_str(&config.get_full_comment_line(&filetype, &hash));
				for line in filecontent.lines().skip(3) {
					new_content.push_str(&line);
					new_content.push_str(pkdevsys_tools::SEPARATOR_NEWLINE);
				}
			}
		}
	}
	let mut file = fs::File::create(filename).to_ce()?;
	file.write_all(new_content.trim_newline_end().as_bytes()).to_ce()?;
	Ok(hash)
}