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

BinaryHeapResult get_max(BinaryHeap* heap) {
    BinaryHeapResult r;
    if (heap->size == 0) {
        r.ok = 0;
        sprintf(r.error , "Heap is empty");
        return r;
    }
    r.ok = 1;
    r.data = heap->data[0];
    return r;
} 

BinaryHeapResult extract_max(BinaryHeap* heap) {
    BinaryHeapResult r = get_max(heap);
    if (!r.ok) {
        return r;
    }
    heap->data[0] = heap->data[heap->size-1];
    heap->size -= 1;
    max_heapify(heap, 0);
    return r;
}

BinaryHeapResult insert(BinaryHeap* heap, Data data) {
    BinaryHeapResult r;
    if (heap->size == heap->capacity) {
        r.ok = 0;
        sprintf(r.error , "Heap is full");
        return r;
    }
    heap->data[heap->size] = data;
    unsigned int i = heap->size;
    heap->size += 1;
    while (i > 0 && heap->data[parent(i)].key < heap->data[i].key) {
        Data swap = heap->data[i];
        heap->data[i] = heap->data[parent(i)];
        heap->data[parent(i)] = swap;
        i = parent(i);
    }
    r.ok = 1;
    return r;
}

void print_data_array(Data* arr, int n) {
    for (int i = 0; i < n; ++i) {
        printf("key: %d - name: %s\n", arr[i].key, arr[i].name);
    }
    printf("\n");
}

void print_heap(BinaryHeap* heap) {
    if (heap == NULL || heap->size == 0) {
        printf("Heap vazia!\n");
        return;
    }
    int height = 0;
    unsigned int size = heap->size;
    while (size > 0) {
        size >>= 1;
        height++;
    }
    int max_width = 1 << (height - 1);
    for (int level = 0; level < height; level++) {
        int level_nodes = 1 << level;
        int node_spacing = max_width / (1 << level);
        for (int i = 0; i < node_spacing; i++) {
            printf("  ");
        }
        for (int i = (1 << level) - 1; i < (1 << (level + 1)) - 1 && i < heap->size; i++) {
            printf("%d", heap->data[i].key);
            for (int j = 0; j < node_spacing * 2 ; j++) {
                printf("  ");
            }
        }
        printf("\n");
        if (level < height - 1) {
            for (int i = 0; i < node_spacing / 2 - 1; i++) {
                printf("  ");
            }
            printf("\n");
        }
    }
}