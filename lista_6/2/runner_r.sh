#!/bin/bash

# Create or clear the results.csv file
echo "run,nodes,edges,modifier,kruskal,prim,kruskal_cost,prim_cost" > results.csv

# Set the problems sizes
for size in 10 20 30 40 50 60 70 80 90 100 200 300 400 500 600 700 800 900 1000 2000 3000 4000 5000 6000 7000 8000 9000 10000; do
    # Run the binary 10 times
    for i in {1..10}; do
      # Capture the output of the binary
      output=$(taskset -c 2 "../../rust/target/release/graph" $size $size 0)
      for line in $output; do
        echo "$i,$line" >> results.csv
      done
    done
  done
done
