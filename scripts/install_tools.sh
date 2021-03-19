#!/bin/bash
# ph0llux:5abff48bb3feffa8563ecaa38ecd1c3062647e56357cb2753f068554a2d39b9d

# install rustup
install_rustup() {
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rustup.sh
	chmod +x /tmp/rustup.sh
	/tmp/rustup.sh -y
	rustup toolchain install nightly #install nightly toolchain
}

install_cool_stuff() {
	cargo install hexyl
	cargo install fd-find
	cargo install hyperfine
	cargo install diskus
}

install_rustup