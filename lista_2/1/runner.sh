#!/bin/bash

# Create or clear the results.csv file
echo "size,classic,strassen" > results.csv

# Set the problems sizes
for size in 16384; do
  # Capture the output of the binary
  output=$("../../c/bin/matrix" $size 0)
  # Write the binary name, run number, and output to the CSV file
  echo "$size,$output" >> results.csv
done