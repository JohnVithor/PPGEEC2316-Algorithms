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

ResultPair minimum_maximum_naive(int* arr, size_t size) {
  ResultPair result;
  if (size == 0) {
    result.status = 0;
    return result;
  }
  result.status = 1;
  result.pair.max = arr[0];
  result.pair.min = arr[0];
  for (size_t i = 0; i < size; ++i) {
    if (arr[i] < result.pair.min) {
      result.pair.min = arr[i];
    }
    if (arr[i] > result.pair.max) {
      result.pair.max = arr[i];
    }
  }
  return result;
}

ResultPair minimum_maximum(int* arr, size_t size) {
  ResultPair result;
  if (size == 0) {
    result.status = 0;
    return result;
  }
  result.status = 1;
  size_t i;
  if (size % 2 == 0) {
    if (arr[0] > arr[1]) {
      result.pair.max = arr[0];
      result.pair.min = arr[1];
    } else {
      result.pair.max = arr[1];
      result.pair.min = arr[0];
    }
    i = 2;
  } else {
    result.pair.max = arr[0];
    result.pair.min = arr[0];
    i = 1;
  }
  while (i+1 < size) {
    if (arr[i] > arr[i + 1]) {
      if (arr[i] > result.pair.max) {
        result.pair.max = arr[i];
      }
      if (arr[i + 1] < result.pair.min) {
        result.pair.min = arr[i + 1];
      }
    } else {
      if (arr[i + 1] > result.pair.max) {
        result.pair.max = arr[i + 1];
      }
      if (arr[i] < result.pair.min) {
        result.pair.min = arr[i];
      }
    }
    i+=2;
  }
  return result;
}