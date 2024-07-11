#!/bin/bash

python3 Problems/plot_hyperfine.py \
  -o Problems/queens/plots/queens.png \
  Problems/queens/data/queens.json

python3 Problems/plot_hyperfine.py \
  -o Problems/queens/plots/queens_forward_checking.png \
  Problems/queens/data/queens_forward_checking.json

python3 Problems/plot_hyperfine.py \
  -o Problems/queens/plots/queens_forward_checking.png \
  Problems/queens/data/queens_forward_checking.json

python3 Problems/queens/scripts/plot_iterations.py
