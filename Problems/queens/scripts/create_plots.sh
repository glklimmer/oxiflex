#!/bin/bash

# ./Problems/queens/scripts/benchmarks.sh
# ./Problems/queens/scripts/benchmarks_forward_checking.sh
# ./Problems/queens/scripts/count_iterations.sh

FULL_LEGEND="Naive,Naive w/ VO,FC,FC w/ VO,AC-1,AC-1 w/ VO,AC-3,AC-3 w/ VO"
NO_ARC="Naive,Naive w/ VO,FC,FC w/ VO"

python3 Problems/plot_hyperfine.py \
  -o Problems/queens/plots/time.png \
  --titles "${FULL_LEGEND}" \
  Problems/queens/data/queens.json

python3 Problems/plot_hyperfine.py \
  -o Problems/queens/plots/time_no_arc.png \
  --titles "${NO_ARC}" \
  Problems/queens/data/queens_forward_checking.json

python3 Problems/queens/scripts/plot_iterations.py

python3 Problems/queens/scripts/plot_iterations_inference.py
