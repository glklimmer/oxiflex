#!/bin/bash

# Ensure jq is installed
if ! command -v jq &>/dev/null; then
  echo "jq could not be found, please install it."
  exit
fi

# Path to the JSON file
json_file="Problems/slow_convergence/data/slow_convergence.json"

# Process the JSON
jq '[.results[] | {(.command): .mean}]' $json_file >output.json
