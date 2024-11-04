#!/bin/bash

taskset -c 2  "../../rust/target/release/min_max" "data.bin" > results_r.csv