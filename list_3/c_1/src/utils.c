#include <stdio.h>
#include <stdlib.h>
#include "binary_heap.h"
#include "utils.h"

int* create_random_array(int n, int seed) {
    srand(seed);
    int* arr = (int*) malloc(n * sizeof(int)); 
    for (int i = 0; i < n; ++i){
        arr[i] = rand() % n;
    }
    return arr;
}

void print_array(Data* arr, int n) {
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

    // Calcula o número máximo de nós no último nível
    int max_width = 1 << (height - 1);
    
    // Para cada nível da árvore
    for (int level = 0; level < height; level++) {
        int level_nodes = 1 << level;
        int node_spacing = max_width / (1 << level);
        
        // Espaçamento inicial para centralizar o nível
        for (int i = 0; i < node_spacing; i++) {
            printf("  ");
        }
        
        // Imprime os nós do nível atual
        for (int i = (1 << level) - 1; i < (1 << (level + 1)) - 1 && i < heap->size; i++) {
            printf("%d", heap->data[i].key);
            
            // Espaçamento entre nós
            for (int j = 0; j < node_spacing * 2 ; j++) {
                printf("  ");
            }
        }
        printf("\n");
        
        // Imprime as conexões entre os níveis (exceto para o último nível)
        if (level < height - 1) {
            // Espaçamento inicial para as conexões
            for (int i = 0; i < node_spacing / 2 - 1; i++) {
                printf("  ");
            }
            printf("\n");
        }
    }
}