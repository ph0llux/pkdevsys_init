#!/bin/bash
# ph0llux:b645ac8a932a1435f0208c5077467a64e6ca63947b542f40619648275ae564f9
# pk:02307349929623860afc7235b9e3c70295cb9bbe3966146ce4b9d873ed769951
GIT_REPO_PATH="$(git rev-parse --show-toplevel)"
CURRENT_DIR="$(pwd)"

# prepare rust source-code
## format src code with rustfmt
cargo fmt

## sign source code files in repository