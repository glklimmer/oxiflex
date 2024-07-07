#!/bin/bash

# Define the path to your program
PROGRAM_PATH='./target/release/oxiflex'

# Define the problem file pattern
PROBLEM_FILE='problems/slow_convergence/minizinc/datafiles/{n}.fzn'

hyperfine --warmup 3 \
  "$PROGRAM_PATH $PROBLEM_FILE -n" \
  "$PROGRAM_PATH $PROBLEM_FILE -f" \
  --parameter-scan n 10 20 \
  --parameter-step-size 10 \
  --export-json problems/slow_convergence/data/slow_convergence.json