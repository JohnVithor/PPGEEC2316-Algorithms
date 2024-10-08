#ifndef _BINARY_HEAP_H_
#define _BINARY_HEAP_H_

typedef struct Data {
  size_t key;
  char name[100];
} Data;

typedef struct BinaryHeapResult {
  char ok;
  union {
    Data data;
    char error[100];
  };
} BinaryHeapResult;

typedef struct BinaryHeap {
  Data* data;
  size_t size;
  size_t capacity;
} BinaryHeap;

size_t parent(size_t i);
size_t left(size_t i);
size_t right(size_t i);

void max_heapify(BinaryHeap* heap, size_t i);
BinaryHeap build_max_heap(Data* arr, size_t n);
void heap_sort(Data* arr, size_t n);

BinaryHeapResult get_max(BinaryHeap* heap);
BinaryHeapResult extract_max(BinaryHeap* heap);
BinaryHeapResult insert(BinaryHeap* heap, Data data);

void print_data_array(Data* arr, int n);
void print_heap(BinaryHeap* heap);

#endif  // _BINARY_HEAP_H_
