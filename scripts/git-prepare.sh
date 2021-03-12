#!/bin/bash
# ph0llux:addb7c35646c7229df5275efd48e711064078acd1b3ef726e1ef1b24a7239038

GIT_REPO_PATH="$(git rev-parse --show-toplevel)"
CURRENT_DIR="$(pwd)"

#cd to git repo root
cd $GIT_REPO_PATH

# prepare rust source-code
## format src code with rustfmt
cargo fmt

# cd back to current dir
cd $CURRENT_DIR

## sign source code files in repository
folder_code_sign $GIT_REPO_PATH