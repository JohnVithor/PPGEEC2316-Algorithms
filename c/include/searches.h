#ifndef _SEARCHES_H_
#define _SEARCHES_H_

typedef struct SearchResult {
  unsigned int index;
  char found;
} SearchResult;

SearchResult binary_search(int* arr, unsigned int start, unsigned int end,
                           int x);

#endif  // _SEARCHES_H_
