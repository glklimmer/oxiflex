#!/bin/bash

# List of numbers
numbers=(4 6 8 10 12 14)

# Loop through the list
for num in "${numbers[@]}"; do
  # Format the filename without leading zeros
  filename="${num}.dzn"

  # Create the file with the specified content
  echo -e "\nn = $num;" >problems/queens/"$filename"

  # Format the output filename for the minizinc command
  outputfile="${num}.fzn"

  # Execute the minizinc command
  minizinc -c problems/queens/queens.mzn -d "problems/queens/$filename" -o "problems/queens/$outputfile"
done

echo "Files created and compiled successfully."
