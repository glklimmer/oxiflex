#!/bin/bash

# Define the path to your program
PROGRAM_PATH='./target/release/oxiflex'

# Define the problem file pattern
PROBLEM_DIR='problems/queens/minizinc/datafiles'
OUTPUT_FILE='problems/queens/data/iterations.json'

# Start the JSON file
echo "{" >$OUTPUT_FILE

# Loop over each problem size from 4 to 14
for n in {4..14..2}; do
  PROBLEM_FILE="$PROBLEM_DIR/${n}.fzn"

  # Print the problem file number in JSON
  if [ $n -ne 4 ]; then
    echo "," >>$OUTPUT_FILE
  fi
  echo "\"$n\": {" >>$OUTPUT_FILE

  # Define an array of options
  declare -a options=(
    "-n -r"
    "-n"
    "-f -r"
    "-f"
    "-a 1 -r"
    "-a 1"
    "-r"
    ""
  )

  # Initialize counter for options
  opt_count=0

  # Loop through each option set
  for opt in "${options[@]}"; do
    # Remove leading/trailing spaces and replace internal spaces with underscores for key names
    key=$(echo "$opt" | sed 's/^\s\+//;s/\s\+$//;s/\s\+/_/g')

    # Special case for an empty option string to ensure uniqueness in JSON keys
    if [ -z "$key" ]; then
      key="no_flags"
    fi

    if [ $opt_count -ne 0 ]; then
      echo "," >>$OUTPUT_FILE
    fi
    echo "\"$key\": " >>$OUTPUT_FILE

    # Run the program with the current options
    echo "Running ${PROBLEM_FILE} ${opt}"

    # Initialize a variable to accumulate results
    total=0
    # Run the benchmark 5 times
    for i in {1..5}; do
      # Run the program with the current options
      result=$($PROGRAM_PATH $PROBLEM_FILE $opt)
      total=$(($total + $result))
    done

    # Calculate average result
    average=$(($total / 5))

    # Append the average result to the JSON
    echo "\"$average\"" >>$OUTPUT_FILE

    ((opt_count++))
  done

# Close the JSON file
echo "}" >>$OUTPUT_FILE
