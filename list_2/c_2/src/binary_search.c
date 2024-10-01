#include <stdio.h>
#include <stdlib.h>
#include <time.h>

typedef struct Result {
    unsigned int index;
    char found;
} Result;

Result binary_search(int* arr, unsigned int start, unsigned int end, int x) {
    Result r;
    if (start <= end) {    
        int mid = (start + end) / 2;
        printf("%d\n", arr[mid]);
        if (arr[mid] == x){
            r.index = mid;
            r.found = 1;
            return r;
        }
        if (x < arr[mid])
            return binary_search(arr, start, mid - 1, x);
        else
            return binary_search(arr, mid + 1, end, x);
	} else {
        r.found = 0;
		return r;
	}
    
}

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

    srand(seed);
    int* a = (int*) malloc(n * sizeof(int)); 
    for (int i = 0; i < n; i++){
        a[i] = i;
    }
    
    struct timespec ts_start;
    struct timespec ts_end;
    
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    Result r = binary_search(a, 0, n, a[1]);
    clock_gettime(CLOCK_MONOTONIC, &ts_end);

    double time_taken = (double)(ts_end.tv_sec - ts_start.tv_sec) + ((double)(ts_end.tv_nsec - ts_start.tv_nsec)/1000000000L);

    printf("%lf\n", time_taken);

    if (r.found)
        printf("Found %d at index %d\n", a[2], r.index);
    else
        printf("Not found\n");

    // for (int i = 0; i < n; i++){
    //     printf("%d ", a[i]);
    // }
    // printf("\n");

    free(a);
    return 0;
}
