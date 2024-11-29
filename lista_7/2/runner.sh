#!/bin/bash
#SBATCH --job-name=matrix_multiply_PPGEEC
#SBATCH --output=results.csv
#SBATCH --error=errors.txt
#SBATCH --hint=compute_bound
#SBATCH --partition=amd-512
#SBATCH --cpus-per-task=128
#SBATCH --time=1-00:00:00
#SBATCH --qos=qos1

# Create or clear the results.csv file
echo "size,i,p,classic,strassen,transposed" > results.csv

# Set the problems sizes
for p in 1 2 4 8 16 32 64 128; do
  export OMP_NUM_THREADS=$p
  for size in 4 8 16 32 64 128 256 512 1024 2048 4096; do
    for i in {1..10}; do
      # Capture the output of the binary
      output=$("../../c/bin/matrix_multiply" $size 0)
      # Write the binary name, run number, and output to the CSV file
      echo "$size,$i,$output" >> results.csv
    done
  done
done
