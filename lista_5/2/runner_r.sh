#!/bin/bash

# Create or clear the results.csv file
echo "run,size,mod,hash,insert,search_existing,delete,search_missing" > results.csv

# Set the problems sizes
for m in 1 1.1 1.2 1.3 1.4 1.5 1.6 1.7 1.8 1.9 2; do
  for size in 100 200 300 400 500 600 700 800 900 1000 2000 3000 4000 5000 6000 7000 8000 9000 10000 20000 30000 40000 50000 60000 70000 80000 90000 100000 200000 300000 400000 500000 600000 700000 800000 900000 1000000; do
      # Run the binary 10 times
      for i in {1..10}; do
        # Capture the output of the binary
        output=$(taskset -c 2 "../../rust/target/release/hash" 0 $size $m $size)
        for line in $output; do
          echo "$i,$size,$m,$line" >> results.csv
        done
      done
    done
  done
done