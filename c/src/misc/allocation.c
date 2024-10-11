#include "misc.h"

float allocation(int* tasks, size_t n, float* workers, size_t k) {
  float max_alloc = 0;
    size_t j = 0;
    for (size_t i = 0; i < k; ++i) {
        workers[i] = 0;
        for(j = i*(n/k); j < (i+1)*(n/k); ++j) {
            workers[i] += tasks[j];
        }
        if (workers[i] > max_alloc){
            max_alloc = workers[i];
        }
    }
    for (; j < n; ++j) {
        workers[k-1] += tasks[j];
    }
    if (workers[k-1] > max_alloc){
        max_alloc = workers[k-1];
    }
    return max_alloc;
}
