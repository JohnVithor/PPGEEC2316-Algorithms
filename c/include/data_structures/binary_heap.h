#ifndef _BINARY_HEAP_H_
#define _BINARY_HEAP_H_

typedef struct Data {
    unsigned int key;
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
    unsigned int size;
    unsigned int capacity;
} BinaryHeap;

unsigned int parent(unsigned int i);
unsigned int left(unsigned int i);
unsigned int right(unsigned int i);

void max_heapify(BinaryHeap* heap, unsigned int i);
BinaryHeap build_max_heap(Data* arr, unsigned int n);
void heap_sort(Data* arr, unsigned int n);

BinaryHeapResult get_max(BinaryHeap* heap);
BinaryHeapResult extract_max(BinaryHeap* heap);
BinaryHeapResult insert(BinaryHeap* heap, Data data);

void print_data_array(Data* arr, int n);
void print_heap(BinaryHeap* heap);

#endif // _BINARY_HEAP_H_
