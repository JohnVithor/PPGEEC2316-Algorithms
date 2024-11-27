#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <math.h>

#include "utils.h"
#include "matrix.h"

#define THRESHOLD 128

void matrix_add_parallel(Matrix* a, Matrix* b, Matrix* c) {
  #pragma omp parallel for if (a->size > THRESHOLD)
  for (size_t i = 0; i < a->size; ++i) {
    #pragma omp parallel for if (b->size > THRESHOLD)
    for (size_t j = 0; j < b->size; ++j) {
      matrix_set(c, i, j, matrix_get(a, i, j) + matrix_get(b, i, j));
    }
  }
}

void matrix_subtract_parallel(Matrix* a, Matrix* b, Matrix* c) {
  #pragma omp parallel for if (a->size > THRESHOLD)
  for (size_t i = 0; i < a->size; ++i) {
    #pragma omp parallel for if (b->size > THRESHOLD)
    for (size_t j = 0; j < b->size; ++j) {
      matrix_set(c, i, j, matrix_get(a, i, j) - matrix_get(b, i, j));
    }
  }
}

void matrix_multiply_parallel(Matrix* a, Matrix* b, Matrix* c) {
  #pragma omp parallel for if (a->size > THRESHOLD)
  for (size_t i = 0; i < a->size; ++i) {
    #pragma omp parallel for if (a->size > THRESHOLD)
    for (size_t j = 0; j < a->size; ++j) {
      matrix_set(c, i, j, 0);
      for (size_t k = 0; k < a->size; ++k) {
        matrix_e_add(c, i, j, matrix_get(a, i, k) * matrix_get(b, k, j));
      }
    }
  }
}

inline void transpose_parallel(Matrix* m) {
  #pragma omp parallel for if (m->size > THRESHOLD)
  for (size_t i = 0; i < m->size; ++i) {
    #pragma omp parallel for if (m->size > THRESHOLD)
    for (size_t j = i + 1; j < m->size; ++j) {
      T x = matrix_get(m, i, j);
      matrix_set(m, i, j, matrix_get(m, j, i));
      matrix_set(m, j, i, x);
    }
  }
};

void matrix_multiply_transposed_parallel(Matrix* a, Matrix* b, Matrix* c) {
  transpose_parallel(b);
  #pragma omp parallel for if (a->size > THRESHOLD)
  for (size_t i = 0; i < a->size; ++i) {
    #pragma omp parallel for if (a->size > THRESHOLD)
    for (size_t j = 0; j < a->size; ++j) {
      matrix_set(c, i, j, 0);
      for (size_t k = 0; k < a->size; ++k) {
        matrix_e_add(c, i, j, matrix_get(a, i, k) * matrix_get(b, j, k));
      }
    }
  }
  transpose_parallel(b);
}


void matrix_multiply_strassen_parallel(Matrix* a, Matrix* b, Matrix* c) {
  if (a->size <= THRESHOLD) {
    matrix_multiply(a, b, c);
    // matrix_set(c, 0, 0, matrix_get(a, 0, 0) * matrix_get(b, 0, 0));
    return;
  }

  size_t mid = a->size / 2;

  MatrixSplit a_split = split4(a);
  Matrix a11 = a_split.m11;
  Matrix a12 = a_split.m12;
  Matrix a21 = a_split.m21;
  Matrix a22 = a_split.m22;

  MatrixSplit b_split = split4(b);
  Matrix b11 = b_split.m11;
  Matrix b12 = b_split.m12;
  Matrix b21 = b_split.m21;
  Matrix b22 = b_split.m22;

  T* aux1_data = (T*)safe_malloc(mid * mid * sizeof(T));
  T* aux2_data = (T*)safe_malloc(mid * mid * sizeof(T));
  T* p1_data = (T*)safe_malloc(mid * mid * sizeof(T));
  T* p2_data = (T*)safe_malloc(mid * mid * sizeof(T));
  T* p3_data = (T*)safe_malloc(mid * mid * sizeof(T));
  T* p4_data = (T*)safe_malloc(mid * mid * sizeof(T));
  T* p5_data = (T*)safe_malloc(mid * mid * sizeof(T));
  T* p6_data = (T*)safe_malloc(mid * mid * sizeof(T));
  T* p7_data = (T*)safe_malloc(mid * mid * sizeof(T));
  Matrix aux1 = matrix_create(aux1_data, mid);
  Matrix aux2 = matrix_create(aux2_data, mid);
  Matrix p1 = matrix_create(p1_data, mid);
  Matrix p2 = matrix_create(p2_data, mid);
  Matrix p3 = matrix_create(p3_data, mid);
  Matrix p4 = matrix_create(p4_data, mid);
  Matrix p5 = matrix_create(p5_data, mid);
  Matrix p6 = matrix_create(p6_data, mid);
  Matrix p7 = matrix_create(p7_data, mid);

  #pragma omp parallel sections
  {
    #pragma omp section
    matrix_add_parallel(&a11, &a22, &aux1);
    #pragma omp section
    matrix_add_parallel(&b11, &b22, &aux2);
  }
  matrix_multiply_strassen_parallel(&aux1, &aux2, &p1);
  
  #pragma omp parallel sections
  {
    #pragma omp section
    {
      matrix_add_parallel(&a21, &a22, &aux1);
      matrix_multiply_strassen_parallel(&aux1, &b11, &p2);
    }
    #pragma omp section
    {
      matrix_subtract_parallel(&b12, &b22, &aux2);
      matrix_multiply_strassen_parallel(&a11, &aux2, &p3);
    }
  }

  #pragma omp parallel sections
  {
    #pragma omp section
    {
      matrix_subtract_parallel(&b21, &b11, &aux1);
      matrix_multiply_strassen_parallel(&a22, &aux1, &p4);
    }
    #pragma omp section
    {
      matrix_add_parallel(&a11, &a12, &aux2);
      matrix_multiply_strassen_parallel(&aux2, &b22, &p5);
    }
  }

#pragma omp parallel sections
  {
    #pragma omp section
    {
      matrix_subtract_parallel(&a21, &a11, &aux1);
    }
    #pragma omp section
    {
      matrix_add_parallel(&b11, &b12, &aux2);
    }
  }

  matrix_multiply_strassen_parallel(&aux1, &aux2, &p6);

#pragma omp parallel sections
  {
    #pragma omp section
    {
      matrix_subtract_parallel(&a12, &a22, &aux1);
    }
    #pragma omp section
    {
      matrix_add_parallel(&b21, &b22, &aux2);
    }
  }

  matrix_multiply_strassen_parallel(&aux1, &aux2, &p7);

  #pragma omp parallel for if (mid > THRESHOLD)
  for (size_t i = 0; i < mid; ++i) {
    #pragma omp parallel for if (mid > THRESHOLD)
    for (size_t j = 0; j < mid; ++j) {
      matrix_set(c, i, j, matrix_get(&p1, i, j) + matrix_get(&p4, i, j) - matrix_get(&p5, i, j) + matrix_get(&p7, i, j));
      matrix_set(c, i, j + mid, matrix_get(&p3, i, j) + matrix_get(&p5, i, j));
      matrix_set(c, i + mid, j, matrix_get(&p2, i, j) + matrix_get(&p4, i, j));
      matrix_set(c, i + mid, j + mid, matrix_get(&p1, i, j) - matrix_get(&p2, i, j) + matrix_get(&p3, i, j) + matrix_get(&p6, i, j));
    }
  }
}


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
  matrix_multiply_parallel(&a, &b, &d);
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  double time_spent_parallel =
      (double)(ts_end.tv_sec - ts_start.tv_sec) +
      ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

  for (size_t i = 0; i < n * n; i++) {
    if (fabs(c_data[i] - d_data[i]) > 0.001) {
      printf("Erro: c[%zu] = %lf != %lf = d[%zu]\n", i, c_data[i], d_data[i], i);
      return 1;
    }
  }

  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  matrix_multiply_strassen(&a, &b, &c);
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  double time_spent_strassen =
      (double)(ts_end.tv_sec - ts_start.tv_sec) +
      ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  matrix_multiply_strassen_parallel(&a, &b, &c);
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  double time_spent_strassen_parallel =
      (double)(ts_end.tv_sec - ts_start.tv_sec) +
      ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  matrix_multiply_transposed(&a, &b, &c);
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  double time_spent_transposed =
      (double)(ts_end.tv_sec - ts_start.tv_sec) +
      ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

  clock_gettime(CLOCK_MONOTONIC, &ts_start);
  matrix_multiply_transposed_parallel(&a, &b, &d);
  clock_gettime(CLOCK_MONOTONIC, &ts_end);
  double time_spent_transposed_parallel =
      (double)(ts_end.tv_sec - ts_start.tv_sec) +
      ((double)(ts_end.tv_nsec - ts_start.tv_nsec) / 1000000000L);

  for (size_t i = 0; i < n * n; i++) {
    if (fabs(c_data[i] - d_data[i]) > 0.001) {
      printf("Erro: c[%zu] = %lf != %lf = d[%zu]\n", i, c_data[i], d_data[i], i);
      return 1;
    }
  }

  printf("%lf,%lf,%lf,%lf,%lf,%lf\n", time_spent_classic, time_spent_parallel, time_spent_strassen, time_spent_strassen_parallel, time_spent_transposed, time_spent_transposed_parallel);
  // printf("%lf,%lf,%lf,%lf,%lf\n", time_spent_classic, time_spent_parallel, time_spent_strassen, time_spent_transposed, time_spent_transposed_parallel);

  free(a_data);
  free(b_data);
  free(c_data);
  free(d_data);
  return 0;
}
