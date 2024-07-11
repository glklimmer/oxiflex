#!/bin/bash

# Define the path to your program
PROGRAM_PATH='oxiflex'

# Define the problem file pattern
PROBLEM_DIR='Problems/slow_convergence/minizinc/datafiles'
OUTPUT_FILE='Problems/slow_convergence/data/iterations.json'

# Start the JSON file
echo "{" >$OUTPUT_FILE

# Loop over each problem size from 4 to 14
for n in {10..60..10}; do
  PROBLEM_FILE="$PROBLEM_DIR/${n}.fzn"

  # Print the problem file number in JSON
  if [ $n -ne 4 ]; then
    echo "," >>$OUTPUT_FILE
  fi
  echo "\"$n\": {" >>$OUTPUT_FILE

  # Define an array of options
  declare -a options=(
    "-n"
    "-f"
  )

  # Initialize counter for options
  opt_count=0

  # Loop through each option set
  for opt in "${options[@]}"; do
    # Remove leading/trailing spaces and replace internal spaces with underscores for key names
    key=$(echo " $opt" | sed 's/^ //;s/ /_/g')

    # Special case for an empty option string to ensure uniqueness in JSON keys
    if [ -z "$key" ]; then
      key="no_flags"
    fi

    if [ $opt_count -ne 0 ]; then
      echo "," >>$OUTPUT_FILE
    fi

    # Run the program with the current options
    echo "Running n=${n} ${opt}"

    # Initialize a variable to accumulate results
    results=0

    # Run the program with the current options
    result=$($PROGRAM_PATH $PROBLEM_FILE $opt)
    echo "${result}"

    # Append the average result along with standard error to the JSON
    echo "\"$key\": \"$result\"" >>$OUTPUT_FILE

    ((opt_count++))
  done

  echo "}" >>$OUTPUT_FILE
done

# Close the JSON file
echo "}" >>$OUTPUT_FILE
