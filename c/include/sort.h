#ifndef _SORT_H_
#define _SORT_H_

#include <stddef.h>

void insertion_sort(int* arr, size_t n);

void merge(int* left, size_t left_size, int* right,
           size_t right_size, int* ret);
void merge_sort(int* arr, size_t size);

size_t partition(int* arr, size_t size);
void quick_sort(int* arr, size_t size);

size_t randomized_partition(int* arr, size_t size);
void randomized_quick_sort(int* arr, size_t size);

void count_sort_book(int* arr, size_t size);
void count_sort(int* arr, size_t size);

void radix_sort(int* arr, size_t size);

unsigned long long insertion_sort_ram(int* arr, size_t n);

unsigned long long merge_ram(int* left, size_t left_size, int* right,
                             size_t right_size, int* ret);
unsigned long long merge_sort_ram(int* arr, size_t size);

size_t partition_ram(int* arr, size_t size, unsigned long long* op);
unsigned long long quick_sort_ram(int* arr, size_t size);

size_t randomized_partition_ram(int* arr, size_t size,
                                      unsigned long long* op);
unsigned long long randomized_quick_sort_ram(int* arr, size_t size);

unsigned long long count_sort_book_ram(int* arr, size_t size);
unsigned long long count_sort_ram(int* arr, size_t size);

unsigned long long radix_sort_ram(int* arr, size_t size);

#endif  // _SORT_H_
