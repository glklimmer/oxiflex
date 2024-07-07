#!/bin/bash

# Define the problem file pattern
PROBLEM_FILE='Problems/slow_convergence/minizinc/datafiles/{n}.fzn'

hyperfine --warmup 3 \
  "oxiflex $PROBLEM_FILE -n" \
  "/usr/local/MiniZincIDE-2.8.3-bundle-linux-x86_64/bin/fzn-gecode $PROBLEM_FILE" \
  --parameter-scan n 100 600 \
  --parameter-step-size 100 \
  --export-json Problems/slow_convergence/data/gecode.json
