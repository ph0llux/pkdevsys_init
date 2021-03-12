#!/bin/bash
# ph0llux:204916dad984b4afea176b77cb678a4dd4bdc865e8d58b209acd89e9a90cfb4a
PKDEVSYS_CONFIG="./configs/pkdevsys_config.toml"
ZSHRC="./configs/zshrc"

mkdir -p "$HOME/.config"
cp -v "$PKDEVSYS_CONFIG" "$HOME/.config/pkdevsys_config.toml"
cp -v "$ZSHRC" "$HOME/.zshrc"