#ifndef _UTILS_H_
#define _UTILS_H_

#include <stddef.h>

int* create_random_array(size_t n, int seed);
void revert_array(int* arr, size_t n);
char validate_sorting(int* arr, size_t n);
void print_array(int* arr, size_t n);
double measure_time_sort(int* arr, size_t n,
                         void (*sort)(int*, size_t));
int array_max_value(int* arr, size_t size);

void* safe_malloc(size_t n);

#endif  // _UTILS_H_
