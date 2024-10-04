#ifndef _UTILS_H_
#define _UTILS_H_

int* create_random_array(int n, int seed);
void revert_array(int* arr, unsigned int n);
char validate_sorting(int* arr, unsigned int n);
void print_array(int* arr, unsigned int n);
double measure_time_sort(int* arr, unsigned int n, void (*sort)(int*, unsigned int));
int array_max_value(int* arr, unsigned int size);

#endif // _UTILS_H_
