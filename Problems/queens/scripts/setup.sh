#!/bin/bash

# List of numbers
numbers=(4 6 8 10 12 14)
numbers=(20 22 24 26 28 30)

# Define the directory path
datafiles="Problems/queens/minizinc/datafiles"

# Check if the directory exists
if [ ! -d "$datafiles" ]; then
  # If the directory doesn't exist, create it
  mkdir -p "$datafiles"
fi

# Loop through the list
for num in "${numbers[@]}"; do
  # Format the data_file without leading zeros
  data_file="${num}.dzn"

  # Create the file with the specified content
  echo -e "\nn = $num;" >$datafiles/"$data_file"

  # Format the output data_file for the minizinc command
  flatzinc_file="${num}.fzn"

  # Execute the minizinc command
  minizinc -c Problems/queens/minizinc/queens.mzn -d "$datafiles/$data_file" -o "$datafiles/$flatzinc_file"
done

echo "Files created and compiled successfully."
