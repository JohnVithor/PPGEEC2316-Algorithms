#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <math.h>

#include "utils.h"
#include "matrix.h"

int main(int argc, char* argv[]) {
  if (argc != 3) {
    printf("Uso: %s <n> <seed> (n = 2^x, para algum x >= 1 e seed >=0])\n",
           argv[0]);
    return 1;
  }
  int n = atoi(argv[1]);
  int seed = atoi(argv[2]);

  if (n < 2 || (n & (n - 1)) != 0 || seed < 0) {
    printf("Uso: %s <n> <seed> (n = 2^x, para algum x >= 1 e seed >=0])\n",
           argv[0]);
    return 1;
  }

  T* a_data = (T*)safe_malloc(n * n * sizeof(T));
  T* b_data = (T*)safe_malloc(n * n * sizeof(T));
  T* c_data = (T*)safe_malloc(n * n * sizeof(T));
  T* d_data = (T*)safe_malloc(n * n * sizeof(T));
  srand48(seed);
  for (size_t i = 0; i < n * n; i++) {
    a_data[i] = drand48();
    b_data[i] = drand48();
    c_data[i] = 0;
    d_data[i] = 0;
  }

  Matrix a = matrix_create(a_data, n);
  Matrix b = matrix_create(b_data, n);
  Matrix c = matrix_create(c_data, n);
  Matrix d = matrix_create(d_data, n);

  struct timespec ts_start;
  struct timespec ts_end;
  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  matrix_multiply(&a, &b, &c);
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  double time_spent_classic =
      (double)(ts_end.tv_sec - ts_start.tv_sec) +
      ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  matrix_multiply_strassen(&a, &b, &d);
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  double time_spent_strassen =
      (double)(ts_end.tv_sec - ts_start.tv_sec) +
      ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  matrix_multiply_transposed(&a, &b, &d);
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  double time_spent_transposed =
      (double)(ts_end.tv_sec - ts_start.tv_sec) +
      ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

  for (size_t i = 0; i < n * n; i++) {
    if (fabs(c_data[i] - d_data[i]) > 0.001) {
      printf("Erro: c[%zu] = %lf != %lf = d[%zu]\n", i, c_data[i], d_data[i], i);
      return 1;
    }
  }

  printf("%lf,%lf,%lf\n", time_spent_strassen, time_spent_classic, time_spent_transposed);

  free(a_data);
  free(b_data);
  free(c_data);
  free(d_data);
  return 0;
}
