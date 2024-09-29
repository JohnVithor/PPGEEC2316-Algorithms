#include <stdio.h>
#include <stdlib.h>

unsigned long long insertion_sort_ram(int* arr, unsigned int n) {
    unsigned long long op = 1; // inicialização da variável i 
    for (unsigned int i = 1; i < n; ++i) {
        op += 3; // comparação com n, acesso a arr[i] e atribuição a key
        int key = arr[i];
        op += 1; // atribuição a j
        unsigned int j = i;
        while (j > 0 && arr[j-1] > key) {
            op += 4; // comparação j > 0, subtração j-1, acesso arr[j-1] e comparação arr[j-1] > key
            op += 4; // subtração j-1, acesso arr[j-1] e arr[j], além da atribuição arr[j-1] a arr[j]
            arr[j] = arr[j-1];
            j--;
            op += 2; // decremento e atribuição ao j
        }
        op += 4; // comparação j > 0, subtração j-1, acesso arr[j-1] e comparação arr[j-1] > key ao sair do laço
        op += 2; // acesso arr[j] e atribuição key a arr[j]
        arr[j] = key;
        op += 2; // incremento e atribuição do i
    }
    op += 1; // comparação i < n ao sair do laço
    return op;
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

    int* arr = (int*) malloc(n * sizeof(int)); 
    srand(seed);
    for (int i = 0; i < n; i++){
        arr[i] = rand() % n;
    }
    
    unsigned long long op = insertion_sort_ram(arr, n);
    printf("%llu\n", op);

    free(arr);
    return 0;
}
