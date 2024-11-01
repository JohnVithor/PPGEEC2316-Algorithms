#!/bin/bash

taskset -c 2  "../../rust/target/release/min_max" "data" > results_r.csv