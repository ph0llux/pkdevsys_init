#!/bin/bash
# ph0llux:cada02f03c70d7bb39787d7209057ebddff19cfd1c4f97a9c54464e3b4ef85d6

PWD="$(pwd)"

AUTHOR="$1"
EMAIL="$2"
SIGN_KEY="$3"
CARGO_DEFAULT_TARGETDIR="$4"
ZSHRC="../configs/zshrc.sh"
CARGO_CONFIG="./configs/cargo_config.toml"

mkdir -p "$HOME/.config"

install_cargo_config() {
	cp -v "$CARGO_CONFIG" "$HOME/.cargo/config.toml"
	echo "" >> "$HOME/.cargo/config.toml"
	echo "" >> "$HOME/.cargo/config.toml"
	echo "[cargo-new]" >> "$HOME/.cargo/config.toml"
	echo "name = \"$AUTHOR\"" >> "$HOME/.cargo/config.toml"
	echo "email = \"$EMAIL\"" >> "$HOME/.cargo/config.toml"
	echo "vcs = \"git\"" >> "$HOME/.cargo/config.toml"
}

install_pkdevsys_config() {
	cd pkdevsys_tools
	cargo build --release --bin gen_config 
	$CARGO_DEFAULT_TARGETDIR/gen_config $AUTHOR $EMAIL $SIGN_KEY > "$HOME/.config/pkdevsys_config.toml"
	cd $PWD
}


install_cargo_config
install_pkdevsys_config
cp -v "$ZSHRC" "$HOME/.zshrc"