#!/bin/bash
#SBATCH --job-name=matrix_multiply_PPGEEC
#SBATCH --output=results.csv
#SBATCH --error=errors.txt
#SBATCH --hint=compute_bound
#SBATCH --partition=amd-512
#SBATCH --time=1-00:00:00
#SBATCH --qos=qos1

# Create or clear the results.csv file
echo "size,i,classic,classic_parallel,strassen,strassen_parallel,transposed,transposed_parallel" > results.csv

# Set the problems sizes
for size in 4 8 16 32 64 128 256 512 1024 2048 4096; do
  for i in {1..10}; do
    # Capture the output of the binary
    output=$("../../c/bin/matrix_multiply" $size 0)
    # Write the binary name, run number, and output to the CSV file
    echo "$size,$i,$output" >> results.csv
  done
done
