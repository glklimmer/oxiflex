#!/bin/bash

# List of numbers
numbers=(10 20)

# Define the directory path
datafiles="problems/slow_convergence/minizinc/datafiles"

# Check if the directory exists
if [ ! -d "$datafiles" ]; then
  # If the directory doesn't exist, create it
  mkdir -p "$datafiles"
fi

# Loop through the list
for num in "${numbers[@]}"; do
  # Format the filename without leading zeros
  filename="${num}.dzn"

  # Create the file with the specified content
  echo -e "\nn = $num;" >$datafiles/"$filename"

  # Format the output filename for the minizinc command
  outputfile="${num}.fzn"

  # Execute the minizinc command
  minizinc -c problems/slow_convergence/minizinc/slow_convergence.mzn -d "$datafiles/$filename" -o "$datafiles/$outputfile"
done

echo "Files created and compiled successfully."
