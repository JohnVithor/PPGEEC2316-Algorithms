#!/bin/bash

taskset -c 2  "../../rust/target/release/min_max" 0 > results_r.csv