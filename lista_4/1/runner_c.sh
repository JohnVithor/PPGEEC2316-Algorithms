#!/bin/bash

taskset -c 0  "./../../c/bin/min_max" "data.bin" > results_c.csv