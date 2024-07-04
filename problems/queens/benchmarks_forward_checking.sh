#!/bin/bash

# Define the path to your program
PROGRAM_PATH='./target/release/oxiflex'

# Define the problem file pattern
PROBLEM_FILE='problems/queens/{n}.fzn'

hyperfine --warmup 3 \
	"$PROGRAM_PATH $PROBLEM_FILE -n -r" \
	"$PROGRAM_PATH $PROBLEM_FILE -n" \
	"$PROGRAM_PATH $PROBLEM_FILE -f -r" \
	"$PROGRAM_PATH $PROBLEM_FILE -f" \
	--parameter-scan n 4 14 \
	--parameter-step-size 2 \
	--export-json problems/queens/queens_forward_checking.json
