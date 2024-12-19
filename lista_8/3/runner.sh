#!/bin/bash

# Create or clear the results.csv file
echo "size,cache,run,lru_cost,mark_cost,lru_elapsed,mark_elapsed" > results.csv

# Set the problems sizes
for size in 4 8 16 32; do
  r=$(taskset -c 2 "../../rust/target/release/matrix" $size 0 > input.data)
  for cache in 100 200 300 400 500 600 700 800 900 1000; do
    # Run the binary 10 times
    for i in {1..10}; do
      # Capture the output of the binary
      output=$(taskset -c 2 "../../rust/target/release/trace_cache" input.data $cache 0)
      for line in $output; do
        echo "$size,$cache,$i,$line" >> results.csv
      done
    done
  done
done