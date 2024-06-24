#!/bin/bash

numbers=(4 6 8 10 12 14 16 18 20)

for num in "${numbers[@]}"; do
	filename=$(printf "%03d.dzn" "$num")

	# create dzn file
	echo -e "\nn = $num;" >problems/queens/"$filename"

	# compile fzn file
	outputfile=$(printf "%03d.fzn" "$num")
	minizinc -c problems/queens/queens.mzn -d "problems/queens/$filename" -o "problems/queens/$outputfile"
done

echo "Files created successfully."
