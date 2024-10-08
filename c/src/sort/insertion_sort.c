#include "sort.h"

void insertion_sort(int* arr, size_t n) {
  for (size_t i = 1; i < n; ++i) {
    int key = arr[i];
    size_t j = i;
    while (j > 0 && arr[j - 1] > key) {
      arr[j] = arr[j - 1];
      --j;
    }
    arr[j] = key;
  }
}

unsigned long long insertion_sort_ram(int* arr, size_t n) {
  unsigned long long op = 1;  // inicialização da variável i
  for (size_t i = 1; i < n; ++i) {
    op += 3;  // comparação com n, acesso a arr[i] e atribuição a key
    int key = arr[i];
    op += 1;  // atribuição a j
    size_t j = i;
    while (j > 0 && arr[j - 1] > key) {  // comparação j > 0, subtração j-1,
                                         // acesso arr[j-1], comparação
      op += 5;  // arr[j-1] > key e conjunção dos resultados
      op += 4;  // subtração j-1, acesso arr[j-1] e arr[j], além da atribuição
                // arr[j-1] a arr[j]
      arr[j] = arr[j - 1];
      j--;
      op += 2;  // decremento e atribuição ao j
    }
    // (ao sair do laço)
    op += 5;  // comparação j > 0, subtração j-1, acesso arr[j-1], comparação
              // arr[j-1] > key e conjunção dos resultados
    op += 2;  // acesso arr[j] e atribuição key a arr[j]
    arr[j] = key;
    op += 2;  // incremento e atribuição do i
  }
  op += 1;  // comparação i < n ao sair do laço
  return op;
}