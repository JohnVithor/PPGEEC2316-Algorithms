#ifndef _ORDER_STATISTICS_H_
#define _ORDER_STATISTICS_H_

#include <stddef.h>

typedef struct Pair {
  int min;
  int max;
} Pair;

typedef struct ResultPair {
  char status;
  Pair pair;
} ResultPair;

int minimum(int* arr, size_t size);
int maximum(int* arr, size_t size);

ResultPair minimum_maximum_naive(int* arr, size_t size);
ResultPair minimum_maximum(int* arr, size_t size);

int randomized_select_kth(int* arr, size_t size, size_t i);
int select_kth(int* arr, size_t size, size_t i);

int weighted_median(int* arr, float* weights, size_t size);


#endif  // _ORDER_STATISTICS_H_
 