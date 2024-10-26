#!/bin/bash

taskset -c 2  "../../rust/target/release/selection" 0 > results_r.csv