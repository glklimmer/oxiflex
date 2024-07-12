#!/bin/bash

start_directory="$1"
iterations="$2"

if [[ -z "$start_directory" || -z "$iterations" ]]; then
  echo "Usage: $0 <start_directory> <iterations>"
  exit 1
fi

visited_directories=()

function find_random_directory() {
  local directories=($(find "$start_directory" -type d))
  local random_dir=""

  while true; do
    random_dir="${directories[RANDOM % ${#directories[@]}]}"
    if [[ ! " ${visited_directories[@]} " =~ " ${random_dir} " ]]; then
      visited_directories+=("$random_dir")
      break
    fi
  done

  echo "$random_dir"
}

function process_directory() {
  local dir="$1"
  cd "$dir" || return

  local mzn_file=$(find . -maxdepth 1 -type f -name "*.mzn" | head -n 1)
  local dzn_file=$(find . -maxdepth 1 -type f -name "*.dzn" | head -n 1)

  if [[ -n "$mzn_file" && -n "$dzn_file" ]]; then
    echo "Running minizinc in $dir"
    minizinc --solver Oxiflex "$mzn_file" "$dzn_file"
  else
    echo "No .mzn or .dzn file found in $dir"
  fi

  cd "$start_directory" || return
}

cd "$start_directory" || exit

for ((i = 0; i < iterations; i++)); do
  random_dir=$(find_random_directory)
  if [[ -z "$random_dir" ]]; then
    echo "No more directories to visit."
    break
  fi
  process_directory "$random_dir"
done

echo "Script completed."
