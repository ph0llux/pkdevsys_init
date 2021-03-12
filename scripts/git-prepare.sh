#!/bin/bash
# ph0llux:e5daafc0921097c940f81f79d4dfb643b522a1a44edfd453198f6b2bfc79f19e

GIT_REPO_PATH="$(git rev-parse --show-toplevel)"
CURRENT_DIR="$(pwd)"

# prepare rust source-code
## format src code with rustfmt
cargo fmt

#cd to git repo root
cd $GIT_REPO_PATH

# cd back to current dir
cd $CURRENT_DIR

## sign source code files in repository
folder_code_sign $GIT_REPO_PATH