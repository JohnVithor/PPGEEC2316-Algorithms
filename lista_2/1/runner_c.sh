#!/bin/bash

# Create or clear the results.csv file
echo "size,i,strassen,classic,transposed" > results_c.csv

# Set the problems sizes
for size in 2 4 8 16 32 64 128 256 512 1024 2048 4096 8192; do
  for i in {1..10}; do
    # Capture the output of the binary
    output=$(taskset -c 0 "../../c/bin/matrix" $size 0)
    # Write the binary name, run number, and output to the CSV file
    echo "$size,$i,$output" >> results_c.csv
  done
done