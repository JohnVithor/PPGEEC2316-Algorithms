#include "order_statistics.h"

int minimum(int* arr, size_t size) {
  int min = arr[0];
  for (size_t i = 0; i < size; ++i) {
    if (arr[i] < min) {
      min = arr[i];
    }
  }
  return min;  
}

int maximum(int* arr, size_t size) {
  int max = arr[0];
  for (size_t i = 0; i < size; ++i) {
    if (arr[i] > max) {
      max = arr[i];
    }
  }
  return max;
}

Pair minimum_maximum_naive(int* arr, size_t size) {
  Pair result;
  result.max = arr[0];
  result.min = arr[0];
  for (size_t i = 0; i < size; ++i) {
    if (arr[i] < result.min) {
      result.min = arr[i];
    } else if (arr[i] > result.max) {
      result.max = arr[i];
    }
  }
  return result;
}

Pair minimum_maximum(int* arr, size_t size) {
  Pair result;
  size_t i;
  if (size % 2 == 0) {
    if (arr[0] > arr[1]) {
      result.max = arr[0];
      result.min = arr[1];
    } else {
      result.max = arr[1];
      result.min = arr[0];
    }
    i = 2;
  } else {
    result.max = arr[0];
    result.min = arr[0];
    i = 1;
  }
  while (i+1 < size) {
    if (arr[i] > arr[i + 1]) {
      if (arr[i] > result.max) {
        result.max = arr[i];
      }
      if (arr[i + 1] < result.min) {
        result.min = arr[i + 1];
      }
    } else {
      if (arr[i + 1] > result.max) {
        result.max = arr[i + 1];
      }
      if (arr[i] < result.min) {
        result.min = arr[i];
      }
    }
    i+=2;
  }
  return result;
}