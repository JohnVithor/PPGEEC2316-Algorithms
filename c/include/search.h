#ifndef _SEARCH_H_
#define _SEARCH_H_

typedef struct SearchResult {
    unsigned int index;
    char found;
} SearchResult;

SearchResult binary_search(int* arr, unsigned int start, unsigned int end, int x);

#endif // _SEARCH_H_
