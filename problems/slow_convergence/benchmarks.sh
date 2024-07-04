#!/bin/bash

# Define the path to your program
PROGRAM_PATH='./target/release/oxiflex'

# Define the problem file pattern
PROBLEM_FILE='problems/slow_convergence/0{n}.fzn'

hyperfine --warmup 3 \
  "$PROGRAM_PATH $PROBLEM_FILE -n -r" \
  "$PROGRAM_PATH $PROBLEM_FILE -n" \
  "$PROGRAM_PATH $PROBLEM_FILE -f -r" \
  "$PROGRAM_PATH $PROBLEM_FILE -f" \
  "$PROGRAM_PATH $PROBLEM_FILE -a 1 -r" \
  "$PROGRAM_PATH $PROBLEM_FILE -a 1" \
  "$PROGRAM_PATH $PROBLEM_FILE -r" \
  "$PROGRAM_PATH $PROBLEM_FILE" \
  --parameter-scan n 100 600 \
  --parameter-step-size 100 \
  --export-json problems/slow_convergence/slow_convergence.json
