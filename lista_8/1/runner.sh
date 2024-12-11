#!/bin/bash

# Create or clear the results.csv file
echo "run,size,mtf_cost,foresee_cost,mtf_time,foresee_time" > results.csv

# Set the problems sizes
for size in 10 20 30 40 50 60 70 80 90 100 200 300 400 500 600 700 800 900 1000 2000 3000 4000 5000 6000 7000 8000 9000 10000 20000 30000 40000 50000 60000 70000 80000 90000 100000; do
  # Run the binary 10 times
  for i in {1..10}; do
    # Capture the output of the binary
    output=$(taskset -c 2 "../../rust/target/release/move_to_front" $size 0)
    for line in $output; do
      echo "$i,$line" >> results.csv
    done
  done
done