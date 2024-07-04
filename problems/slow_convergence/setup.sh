#!/bin/bash

# List of numbers
numbers=(100 200 300 400 500 600)

# Loop through the list
for num in "${numbers[@]}"; do
  # Format the filename without leading zeros
  filename="0${num}.dzn"

  # Create the file with the specified content
  echo -e "\nn = $num;" >problems/slow_convergence/"$filename"

  # Format the output filename for the minizinc command
  outputfile="0${num}.fzn"

  # Execute the minizinc command
  minizinc -c problems/slow_convergence/slow_convergence.mzn -d "problems/slow_convergence/$filename" -o "problems/slow_convergence/$outputfile"
done

echo "Files created and compiled successfully."