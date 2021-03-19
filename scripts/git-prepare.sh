#!/bin/bash
# ph0llux:8e7a405d0556039a460d12d197a16ad8facddddc6e0bdb321a319b930af92c89

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