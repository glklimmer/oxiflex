#!/bin/bash

# ./Problems/slow_convergence/scripts/benchmarks.sh
# ./Problems/slow_convergence/scripts/benchmarks_small.sh
# ./Problems/slow_convergence/scripts/benchmark_gecode.sh

# ./Problems/slow_convergence/scripts/count_iterations.sh
# ./Problems/slow_convergence/scripts/count_iterations_small.sh

python3 Problems/plot_hyperfine.py \
  -o Problems/slow_convergence/plots/time.png \
  --titles "Naive w/ VO,FC w/ VO" \
  Problems/slow_convergence/data/slow_convergence.json

python3 Problems/plot_hyperfine.py \
  -o Problems/slow_convergence/plots/time_small.png \
  --titles "Naive w/ VO,FC w/ VO,AC-1 w/ VO,AC-3 w/ VO" \
  Problems/slow_convergence/data/slow_convergence_small.json

python3 Problems/plot_hyperfine.py \
  -o Problems/slow_convergence/plots/gecode.png \
  --titles "Oxiflex,gecode" \
  Problems/slow_convergence/data/gecode.json

python3 Problems/slow_convergence/scripts/plot_iterations.py

python3 Problems/slow_convergence/scripts/plot_iterations_small.py
