#!/bin/bash

# Create or clear the results.csv file
echo "run,size,version,random,best,worse" > results.csv

count=1
# Set the problems sizes
for size in 10 20 30 40 50 60 70 80 90 100 200 300 400 500 600 700 800 900 1000 2000 3000 4000 5000 6000 7000 8000 9000 10000 20000 30000 40000 50000 60000 70000 80000 90000 100000 ; do
  # Run the binary 10 times
  output2=$("../../c/bin/sort_ram" $size 0 0)
  echo "$count,$size,$output2" >> results.csv
  for i in {1..10}; do
    # Capture the output of the binary
    output1=$("../../c/bin/sort" $size 0 0)
    # Write the binary name, run number, and output to the CSV file
    echo "$i,$size,$output1" >> results.csv
  done
  count=$((count+1))
done