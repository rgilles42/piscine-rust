#!/usr/bin/env bash

IFS='
'
cd -P "$(dirname "$0")"

NC="\033[0m"
WHT="\033[0;37m"
BLK="\033[0;30m"
RED="\033[0;31m"
YEL="\033[0;33m"
BLU="\033[0;34m"
GRN="\033[0;32m"

ARG=$1
IS_VERBOSE=false
CARGO_FORMAT=false
CARGO_FORMAT_CHECK=false
CARGO_CLIPPY=false
TEST_EXERCISES=true
REAL_ENV_TEST=false
LOCAL_REAL_ENV_TEST=false
CARGO_RUN=false
PULL_FROM="test-rust"

EXIT_CODE=0

update_exit_code() {
	$@
	if [ $? != 0 ]; then
		EXIT_CODE=1
	fi
}

run_test() {
	test_cargo_toml="${1}Cargo.toml"
	solution_cargo_toml="../solutions/${1%_test/}/Cargo.toml"
	ex_name=${1%_test/}

	if [[ $CARGO_FORMAT == true ]]; then
		printf "  ${GRN}[FORMAT]${NC} %s\n" $ex_name
		update_exit_code cargo fmt --manifest-path "$test_cargo_toml"
		update_exit_code cargo fmt --manifest-path "$solution_cargo_toml"
	fi
	if [[ $CARGO_FORMAT_CHECK == true ]]; then
		printf "  ${GRN}[FMT CHECK]${NC} %s\n" $ex_name
		update_exit_code cargo fmt --check --manifest-path "$test_cargo_toml"
		update_exit_code cargo fmt --check --manifest-path "$solution_cargo_toml"
	fi
	if [[ $CARGO_CLIPPY == true ]]; then
		printf "  ${YEL}[CLIPPY]${NC} %s\n" $ex_name
		update_exit_code cargo clippy -q --manifest-path "$test_cargo_toml"
		update_exit_code cargo clippy -q --manifest-path "$solution_cargo_toml"
	fi
	if [[ $LOCAL_REAL_ENV_TEST == true || $REAL_ENV_TEST == true ]]; then
		printf "  ${GRN}[LOCAL_REAL_ENV]${NC} %s\n" $ex_name

		update_exit_code docker run --read-only \
			--network none \
			--memory 500M \
			--cpus 2.0 \
			--user 1000:1000 \
			--env EXERCISE="$ex_name" \
			--env USERNAME=msessa \
			--env HOME=/jail \
			--env TMPDIR=/jail \
			--workdir /jail \
			--tmpfs /jail:size=100M,noatime,exec,nodev,nosuid,uid=1000,gid=1000,nr_inodes=5k,mode=1700 \
			--volume "$(dirname $(pwd))"/student:/jail/student:ro \
			rust_tests
	fi
	if [[ $CARGO_RUN == true ]]; then
		printf "  ${RED}[RUN   ]${NC} %s\n" $ex_name
		update_exit_code cargo run --manifest-path "$test_cargo_toml"
	fi
	if [[ $TEST_EXERCISES == true ]]; then
		printf "  ${BLU}[TEST  ]${NC} %s\n" $ex_name
		if [[ $IS_VERBOSE == true ]]; then
			update_exit_code cargo test --manifest-path "$test_cargo_toml"
		else
			update_exit_code cargo test -q --manifest-path "$test_cargo_toml" >/dev/null
		fi
	fi
}

if [ -n $ARG ] && ([[ $ARG == '-h' ]] || [[ $ARG == '--help' ]]); then
	echo "Run cargo test for all the exercises

	-h, --help          show this usage screen
	-t                  show a table with the time it takes to run each exercise
	-v                  show more details for each test
	-f                  apply \"cargo fmt\" to the exercises
    -fc                 check style with \"cargo fmt --check\"
	-c                  run \"cargo clippy\" to the exercises
	-n                  do NOT run \"cargo test\" on the exercises
	-real               execute the test using the same docker image used by the runner
	-local-real         build and run the docker image locally
    -m                  run the main() in tests
    -pull-from			specify the PR id to take the a specific package image (master by default)
	[exercise_name]     test one or more selected exercises (separated by spaces)
	[NO ARGUMENTS]      test all exercises in test directory"
elif [[ $ARG == '-t' ]]; then
	printf "NOTICE: This could take some minutes before to show any output\n"
	printf "| %-26s| %-14s|\n" Exercise Time
	# Print a table with the time that took to test each exercise
	for dir in */; do
		exercise_name=${dir%_test/}
		# Don't clean the folder that don't exist
		# This are the only exercises that don't follow the
		# pattern (1 solution -> 1 crate)
		if [ $exercise_name != 'matrix_mult' -a $exercise_name != 'matrix_ops' -a $exercise_name != 'roman_numbers_iter' ]; then
			cargo clean --manifest-path ../solutions/"$exercise_name"/Cargo.toml
		fi
		cargo clean --manifest-path "$dir"Cargo.toml

		time=$(/usr/bin/time -f '%e secs.' cargo test -q --manifest-path "$dir"Cargo.toml 1 2>&1 >/dev/null | grep secs)
		printf "| %-26s| %-14s|\n" $exercise_name "$time"
	done |
		sort -rn -k4
else
	# Arguments parsing
	while [[ $# -gt 0 ]]; do
		case $1 in
		-v)
			IS_VERBOSE=true
			shift
			;;
		-f)
			CARGO_FORMAT=true
			shift
			;;
		-fc)
			CARGO_FORMAT_CHECK=true
			shift
			;;
		-c)
			CARGO_CLIPPY=true
			shift
			;;
		-n)
			TEST_EXERCISES=false
			shift
			;;
		-real)
			REAL_ENV_TEST=true
			shift
			;;
		-local-real)
			LOCAL_REAL_ENV_TEST=true
			shift
			;;
		-m)
			CARGO_RUN=true
			shift
			;;
		-pull-from)
			PULL_FROM=$2
			shift
			shift
			;;
		*)
			break
			;;
		esac
	done

	if [[ $LOCAL_REAL_ENV_TEST == true || $REAL_ENV_TEST == true ]]; then
		rm -rf ../student
		cp -r ../solutions ../student
		rm -rf ../student/**/target

		docker build -t rust_tests ../.
	fi
	if [[ $# -gt 0 ]]; then
		while [[ $# -gt 0 ]]; do
			exercise="${1}_test/"
			run_test $exercise
			shift # past argument
		done
	else
		for dir in */; do
			# Lib is a dependency but it is in tests directory, need to be skipped
			if [[ $dir == "lib/" ]]; then
				continue
			fi
			run_test $dir
		done
	fi

	if [[ $REAL_ENV_TEST == true ]]; then
		rm -rf ../student.zip ../student
	fi
	if [[ $LOCAL_REAL_ENV_TEST == true ]]; then
		rm -rf ../student
	fi
fi

exit $EXIT_CODE
