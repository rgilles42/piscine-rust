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

# if test "$EXAM_MODE"; then				# in exam mode, you might just have a single text edit box so just one file is created, workarounds necessary
# 	cd "solutions/$EXERCISE"
#	# ! to support both the old and the new version of the runner we
#	# ! need to check the files in the code editor
# 	if ! echo "$EDITOR_FILES" | tr ',' '\n' | grep -q 'src/main.rs'; then
#		if test "$CODE_EDITOR_RUN_ONLY"; then
#			mv src/lib.rs src/main.rs 2>&1 ||:
#		fi
#	fi
# 	cargo init
# 	cd
# fi

if ! test -f "tests/${EXERCISE}_test/Cargo.toml"; then
	echo "No test file found for the exercise : $EXERCISE"
	exit 1
fi

# if test "$CODE_EDITOR_RUN_ONLY"; then			# in exam mode the moulinette can simply run your code to show you the results
# 	cargo run --manifest-path "solutions/$EXERCISE/Cargo.toml" -- "$@"
# else
cargo test --manifest-path "tests/${EXERCISE}_test/Cargo.toml"
# fi
