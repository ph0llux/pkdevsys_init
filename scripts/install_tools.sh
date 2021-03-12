#!/bin/bash
# ph0llux:31f39b07f18ec55db1b6fdcbf02bb3e6bb5e1c2970a4c055c5c826654e0d9990

# install rustup
install_rustup() {
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rustup.sh
	chmod +x /tmp/rustup.sh
	/tmp/rustup.sh -y
	rustup toolchain install nightly #install nightly toolchain
}

install_rustup