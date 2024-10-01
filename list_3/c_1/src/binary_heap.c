#include <stdio.h>

#include "binary_heap.h"
#include "utils.h"

unsigned int parent(unsigned int i) {
    return i / 2;
}

unsigned int left(unsigned int i) {
    return 2 * i;
}

unsigned int right(unsigned int i) {
    return 2 * i + 1;
}

void max_heapify(BinaryHeap* heap, unsigned int i) {
    unsigned int l = left(i);
    unsigned int r = right(i);
    unsigned int largest;
    if (l < heap->size && heap->data[l].key > heap->data[i].key)
        largest = l;
    else
        largest = i;
    if (r < heap->size && heap->data[r].key > heap->data[largest].key)
        largest = r;

    if (largest != i) {
        Data swap = heap->data[i];
        heap->data[i] = heap->data[largest];
        heap->data[largest] = swap;
        max_heapify(heap, largest);
    }
}

BinaryHeap build_max_heap(Data* data, unsigned int n){
    BinaryHeap heap;
    heap.data = data;
    heap.size = n;
    heap.capacity = n;
    for (unsigned int i = n/2; i > 0; --i){
        max_heapify(&heap, i-1);
    }
    return heap;
}

void heap_sort(Data* arr, unsigned int n) {
    BinaryHeap heap = build_max_heap(arr, n);
    heap.size = n;
    for (unsigned int i = n-1; i > 0; --i) {
        Data swap = arr[0];
        arr[0] = arr[i];
        arr[i] = swap;
        heap.size -= 1;
        max_heapify(&heap, 0);
    }
}

Result get_max(BinaryHeap* heap) {
    Result r;
    if (heap->size == 0) {
        r.ok = 0;
        sprintf(r.error , "Heap is empty");
        return r;
    }
    r.ok = 1;
    r.data = heap->data[0];
    return r;
} 

Result extract_max(BinaryHeap* heap) {
    Result r = get_max(heap);
    if (!r.ok) {
        return r;
    }
    heap->data[0] = heap->data[heap->size-1];
    heap->size -= 1;
    max_heapify(heap, 0);
    return r;
}

Result increase_key(BinaryHeap* heap, Data* data, unsigned int k) {
    Result r;
    if (k < data->key) {
        r.ok = 0;
        sprintf(r.error , "new key is smaller than current key");
        return r;
    }
    data->key = k;
    unsigned int i = heap->size;
    while (i > 0 && heap->data[parent(i)].key < heap->data[i].key) {
        Data swap = heap->data[i];
        heap->data[i] = heap->data[parent(i)];
        heap->data[parent(i)] = swap;
        i = parent(i);
    }
    r.ok = 1;
    return r;
}

Result insert(BinaryHeap* heap, Data data) {
    Result r;
    if (heap->size == heap->capacity) {
        r.ok = 0;
        sprintf(r.error , "Heap is full");
        return r;
    }
    heap->size += 1;
    unsigned int k = data.key;
    data.key = k; // maybe error
    heap->data[heap->size-1] = data;
    increase_key(heap, &data, k);
    r.ok = 1;
    return r;
}