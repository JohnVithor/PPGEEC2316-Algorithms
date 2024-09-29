#ifndef _UTILS_H_
#define _UTILS_H_

#include <iostream>
#include <vector>
#include <algorithm>

std::vector<int> create_desc_array(unsigned int n);

void fill_random(std::vector<int> &arr, int seed);

void print_array(std::vector<int> &arr);

void validate(std::vector<int> &arr);

#endif // _UTILS_H_
