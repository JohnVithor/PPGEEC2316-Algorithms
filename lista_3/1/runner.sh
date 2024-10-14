#!/bin/bash

# Create or clear the results.csv file
echo "size,classic,strassen" > results.csv

# Set the problems sizes
for size in 2 4 8 16 32 64 128 256 512 1024 2048 4096 8192 16384; do
  # Capture the output of the binary
  output=$("../../c/bin/matrix" $size 0)
  # Write the binary name, run number, and output to the CSV file
  echo "$size,$output" >> results.csv
done