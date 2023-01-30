#!/usr/bin/env bash
 
set -eo pipefail
IFS='
'

# cp -a /app/tests .
# cp -a student solutions

if [ $# -le 0 ]; then
	echo "Usage: $0 exercise_name"
	exit 1
fi
EXERCISE=$1

# if test "$EXAM_MODE"; then				# in exam mode, you just have a single text edit box so just one file is created, workarounds necessary
# 	cd "solutions/$EXERCISE"
# 	if test "$EXAM_RUN_ONLY"; then
# 		mv src/lib.rs src/main.rs 2>&1 ||:
# 	fi
# 	cargo init
# 	cd
# fi

if ! test -f "tests/${EXERCISE}_test/Cargo.toml"; then
	echo "No test file found for the exercise : $EXERCISE"
	exit 1
fi

if find solutions -type f -name '*.rs' -exec grep -q 'std::process' {} +; then
	echo "Your Rust source code cannot use std::process"
	exit 1
fi

# if test "$EXAM_RUN_ONLY"; then										# in exam mode the moulinette can simply run your code to show you the results
# 	cargo run --manifest-path "solutions/$EXERCISE/Cargo.toml" -- "$@"
# else
cargo test --manifest-path "tests/${EXERCISE}_test/Cargo.toml"
# fi
