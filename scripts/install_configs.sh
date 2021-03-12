#!/bin/bash
# ph0llux:2838783c908094bbb0ebf78c7f04856db8f6fe9a9fea8f5b23ed9cfde26518ca
PKDEVSYS_CONFIG="./configs/pkdevsys_config.toml"
ZSHRC="./configs/zshrc.sh"

mkdir -p "$HOME/.config"
cp -v "$PKDEVSYS_CONFIG" "$HOME/.config/pkdevsys_config.toml"
cp -v "$ZSHRC" "$HOME/.zshrc"