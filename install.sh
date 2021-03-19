#!/bin/bash
# ph0llux:356bfc895cc24a33d4cc7ad101d99bb5cd95e75e98f9e5665b99bf7a80595800

#Vars, you could change!
AUTHOR="ph0llux" #name of the author (mostly used: nicknames or firstnames)
EMAIL="ph0llux@pm.me" #your email address (will be configured globally)
SIGN_KEY="$(openssl rand -base64 32)" #change is optional. This will generate a random, new secret key.

#other vars
PWD="$(pwd)"
CARGO_DEFAULT_TARGETDIR="/tmp/cargo/targets/release/"

LOCAL_BINFOLDER="$HOME/.bin"
mkdir -pv $LOCAL_BINFOLDER

install_necessary_tools() {
	./scripts/install_tools.sh
	cp -v scripts/git-prepare.sh "$LOCAL_BINFOLDER/git-prepare"
}

copy_config_files() {
	./scripts/install_configs.sh $AUTHOR $EMAIL $SIGN_KEY $CARGO_DEFAULT_TARGETDIR
}

install_pkdevsys_tools() {
	cd pkdevsys_tools
	cargo build --release
	cp -v "$CARGO_DEFAULT_TARGETDIR/code_sign" "$LOCAL_BINFOLDER/code_sign"
	cp -v "$CARGO_DEFAULT_TARGETDIR/folder_code_sign" "$LOCAL_BINFOLDER/folder_code_sign"
}


copy_config_files
install_necessary_tools
install_pkdevsys_tools

"$LOCAL_BINFOLDER/folder_code_sign" "$HOME/.cargo/config.toml"