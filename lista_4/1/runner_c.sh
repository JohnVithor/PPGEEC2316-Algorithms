#!/bin/bash

taskset -c 0  "./../../c/bin/min_max" "data" > results_c.csv