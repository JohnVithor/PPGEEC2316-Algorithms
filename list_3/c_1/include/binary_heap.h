#ifndef _BINARY_HEAP_H_
#define _BINARY_HEAP_H_

typedef struct Data {
    unsigned int key;
    char name[100];
} Data;

typedef struct Result {
    char ok;
    union {
        Data data;
        char error[100];
    };
} Result;

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

Result get_max(BinaryHeap* heap);
Result extract_max(BinaryHeap* heap);
Result increase_key(BinaryHeap* heap, Data* data, unsigned int k);
Result insert(BinaryHeap* heap, Data data);

#endif // _BINARY_HEAP_H_
