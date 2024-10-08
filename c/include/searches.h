#ifndef _SEARCHES_H_
#define _SEARCHES_H_

#include <stddef.h>

typedef struct SearchResult {
  size_t index;
  char found;
} SearchResult;

SearchResult binary_search(int* arr, size_t start, size_t end,
                           int x);

#endif  // _SEARCHES_H_
