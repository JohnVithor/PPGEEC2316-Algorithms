#!/bin/bash

taskset -c 2  "../../rust/target/release/selection" "data.bin" > results_r.csv