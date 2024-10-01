#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include "utils.h"
#include "binary_heap.h"

int main(int argc, char** argv) {
    if (argc != 3) {
        printf("Uso: %s <n> <seed> (n > 1 e seed >=0)\n", argv[0]);
        return 1;
    }
    int n = atoi(argv[1]);
    int seed = atoi(argv[2]);

    if (n <= 1 || seed < 0) {
        printf("Uso: %s <n> <seed> (n > 1 e seed >=0)\n", argv[0]);
        return 1;
    }

    struct timespec ts_start;
    struct timespec ts_end;

    int* arr = create_random_array(n, seed);

    Data* data = (Data*) malloc(n * sizeof(Data));
    for (int i = 0; i < n; i++) {
        data[i].key = arr[i];
        sprintf(data[i].name, "name_%d", i);
    }

    print_array(data, n);

    BinaryHeap heap = build_max_heap(data, n);

    print_heap(&heap);

    Result r = extract_max(&heap);
    if (r.ok) {
        printf("extracted: key: %d - name: %s\n", r.data.key, r.data.name);
    } else {
        printf("Error: %s\n", r.error);
    }
    printf("size: %d\n", heap.size);

    print_heap(&heap);

    r.data.key = 4;

    printf("inserting: key: %d - name: %s\n", r.data.key, r.data.name);

    r = insert(&heap, r.data);

    print_heap(&heap);

    free(arr);
    free(data);
    return 0;
}
