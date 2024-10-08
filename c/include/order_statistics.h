#ifndef _ORDER_STATISTICS_H_
#define _ORDER_STATISTICS_H_

#include <stddef.h>

typedef struct Pair {
  int min;
  int max;
} Pair;

int minimum(int* arr, size_t size);
int maximum(int* arr, size_t size);

Pair minimum_maximum_naive(int* arr, size_t size);
Pair minimum_maximum(int* arr, size_t size);

size_t randomized_select_kth(int* arr, size_t start, size_t end,size_t i);
size_t select_kth(int* arr, size_t start, size_t end,size_t i);

size_t weighted_median(int* arr, float* weights, size_t size);


#endif  // _ORDER_STATISTICS_H_
 