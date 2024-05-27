```
~/unibas/bachelor/oxiflex (forward_checking ✘)✹✭ ᐅ hyperfine './target/release/oxiflex problems/queens/008.fzn -n' './target/release/oxiflex problems/queens/008.fzn' --export-csv result_080_queens.csv
Benchmark 1: ./target/release/oxiflex problems/queens/008.fzn -n
  Time (mean ± σ):       1.7 ms ±   0.6 ms    [User: 1.5 ms, System: 0.3 ms]
  Range (min … max):     1.1 ms …   5.0 ms    1134 runs

  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.

Benchmark 2: ./target/release/oxiflex problems/queens/008.fzn
  Time (mean ± σ):      84.8 ms ±  98.9 ms    [User: 84.1 ms, System: 0.7 ms]
  Range (min … max):    17.0 ms … 485.6 ms    26 runs

Summary
  './target/release/oxiflex problems/queens/008.fzn -n' ran
   48.93 ± 59.64 times faster than './target/release/oxiflex problems/queens/008.fzn'
```

```
~/unibas/bachelor/oxiflex (forward_checking ✘)✹✭ ᐅ hyperfine './target/release/oxiflex problems/queens/008.fzn -n' './target/release/oxiflex problems/queens/008.fzn' --export-csv result2_080_queens.csv
Benchmark 1: ./target/release/oxiflex problems/queens/008.fzn -n
  Time (mean ± σ):       1.8 ms ±   0.6 ms    [User: 1.6 ms, System: 0.3 ms]
  Range (min … max):     1.1 ms …   5.4 ms    939 runs

  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.

Benchmark 2: ./target/release/oxiflex problems/queens/008.fzn
  Time (mean ± σ):      78.7 ms ±  61.8 ms    [User: 78.0 ms, System: 0.7 ms]
  Range (min … max):    17.1 ms … 238.2 ms    55 runs

Summary
  './target/release/oxiflex problems/queens/008.fzn -n' ran
   43.08 ± 37.13 times faster than './target/release/oxiflex problems/queens/008.fzn'
```

```
~/unibas/bachelor/oxiflex (forward_checking ✘)✹✭ ᐅ hyperfine './target/release/oxiflex problems/queens/008.fzn -n' './target/release/oxiflex problems/queens/008.fzn' --export-markdown result_080_queens.md
Benchmark 1: ./target/release/oxiflex problems/queens/008.fzn -n
  Time (mean ± σ):       1.4 ms ±   0.7 ms    [User: 1.4 ms, System: 0.3 ms]
  Range (min … max):     0.6 ms …   5.1 ms    1124 runs

  Warning: Command took less than 5 ms to complete. Note that the results might be inaccurate because hyperfine can not calibrate the shell startup time much more precise than this limit. You can try to use the `-N`/`--shell=none` option to disable the shell completely.

Benchmark 2: ./target/release/oxiflex problems/queens/008.fzn
  Time (mean ± σ):      96.6 ms ±  90.0 ms    [User: 96.2 ms, System: 0.5 ms]
  Range (min … max):    16.3 ms … 450.0 ms    95 runs

Summary
  './target/release/oxiflex problems/queens/008.fzn -n' ran
   67.71 ± 70.61 times faster than './target/release/oxiflex problems/queens/008.fzn'
```
