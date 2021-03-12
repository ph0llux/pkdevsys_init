#!/bin/bash
# ph0llux:815fa2b8bca55884c6c62698e1dd803af7140a19962b18c1e6cc6198b44c5141

# TODO: check if neccessary tools are installed
# curl
# git
# xmodmap
# zsh


# install rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > /tmp/rustup.sh
chmod +x /tmp/rustup.sh
/tmp/rustup.sh -y
rustup toolchain install nightly #install nightly toolchain