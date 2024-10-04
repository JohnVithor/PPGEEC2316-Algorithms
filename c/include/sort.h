#ifndef _SORT_H_
#define _SORT_H_

void insertion_sort(int* arr, unsigned int n);
unsigned long long insertion_sort_ram(int* arr, unsigned int n);

void merge(int* left, unsigned int left_size, int* right, unsigned int right_size, int* ret);
void merge_sort(int* arr, unsigned int size);

unsigned int partition(int* arr, unsigned int size);
void quick_sort(int* arr, unsigned int size);

unsigned int randomized_partition(int* arr, unsigned int size);
void randomized_quick_sort(int* arr, unsigned int size);

void count_sort_book(int* arr, unsigned int size);
void count_sort(int* arr, unsigned int size);

void radix_sort(int* arr, unsigned int size);

#endif // _SORT_H_
