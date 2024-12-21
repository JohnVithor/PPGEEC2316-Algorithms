#!/bin/bash

# Create or clear the results.csv file
echo "cache,sequence,run,oracle_cost,lru_cost,oracle_time,lru_time" > results.csv

# Set the problems sizes
for cache in 100 200 300 400 500 600 700 800 900 1000; do
  for seq in 1000 2000 3000 4000 5000 6000 7000 8000 9000 10000 11000 12000 13000 14000 15000 16000 17000 18000 19000 20000; do
    # Run the binary 10 times
    for i in {1..10}; do
      # Capture the output of the binary
      output=$(taskset -c 2 "../../rust/target/release/cache" $cache $seq 0)
      for line in $output; do
        echo "$cache,$seq,$i,$line" >> results.csv
      done
    done
  done
done