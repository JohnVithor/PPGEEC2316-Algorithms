#include <stdio.h>
#include <stdlib.h>
#include "utils.h"

#define SIZE_MAX 250000000

int main(int argc, char* argv[]) {
  if (argc != 3) {
    printf("Uso: %s <path> <seed>\n", argv[0]);
    return 1;
  }
  int seed = atoi(argv[2]);
  srand(seed);
  FILE* f = fopen(argv[1], "wb");
  if (f == NULL) {
    printf("Erro ao abrir o arquivo\n");
    return 1;
  }
  int* v = (int*)safe_malloc(SIZE_MAX * sizeof(int));
    for (size_t i = 0; i < SIZE_MAX; ++i) {
    v[i] = i;
  }
  for (size_t i = 0; i < SIZE_MAX; ++i) {
    size_t j = rand() % SIZE_MAX;
    int tmp = v[i];
    v[i] = v[j];
    v[j] = tmp;
  }
  print_array(v, 10);
  fwrite(v, sizeof(int), SIZE_MAX, f);
  fclose(f);
  return 0;
}