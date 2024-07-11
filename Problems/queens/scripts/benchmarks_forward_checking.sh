#!/bin/bash

# Define the path to your program
PROGRAM_PATH='oxiflex'

# Define the problem file pattern
PROBLEM_FILE='Problems/queens/minizinc/datafiles/{n}.fzn'

hyperfine --warmup 3 \
  "$PROGRAM_PATH $PROBLEM_FILE -n -r" \
  "$PROGRAM_PATH $PROBLEM_FILE -n" \
  "$PROGRAM_PATH $PROBLEM_FILE -f -r" \
  "$PROGRAM_PATH $PROBLEM_FILE -f" \
  --parameter-scan n 8 20 \
  --parameter-step-size 2 \
  --export-json Problems/queens/data/queens_forward_checking.json
