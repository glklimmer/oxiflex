#!/bin/bash

# Define the path to your program
PROGRAM_PATH='./target/release/oxiflex'

# Define the problem file pattern
PROBLEM_FILE='problems/slow_convergence/minizinc/datafiles/{n}.fzn'

hyperfine --warmup 3 \
  "$PROGRAM_PATH $PROBLEM_FILE -n" \
  "$PROGRAM_PATH $PROBLEM_FILE -f" \
  "$PROGRAM_PATH $PROBLEM_FILE -a 1" \
  "$PROGRAM_PATH $PROBLEM_FILE" \
  --parameter-scan n 2 10 \
  --parameter-step-size 1 \
  --export-json problems/slow_convergence/data/time_small.json
