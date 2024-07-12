#!/bin/bash

# Define the path to your program
PROGRAM_PATH='Oxiflex'

# Define the problem file pattern
PROBLEM_FILE='Problems/slow_convergence/minizinc/datafiles/{n}.fzn'

hyperfine --warmup 3 \
  "$PROGRAM_PATH $PROBLEM_FILE -n" \
  "$PROGRAM_PATH $PROBLEM_FILE -f" \
  --parameter-scan n 10 60 \
  --parameter-step-size 10 \
  --export-json Problems/slow_convergence/data/slow_convergence.json