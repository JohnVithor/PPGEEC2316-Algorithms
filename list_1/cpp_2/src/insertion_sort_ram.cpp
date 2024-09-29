#include <stdio.h>
#include <stdlib.h>

unsigned long long insertion_sort_ram(Iterator begin, Iterator end) {
    unsigned long long op = 2; // soma begin + 1 e inicialização do iterador 
    for (Iterator it = begin + 1; it < end; ++it) {
        op += 3; // comparação de it < end, acesso a *it e atribuição a key
        int key = *it;
        op += 1; // atribuição a j
        Iterator jt = it;
        while (jt > begin && *(jt-1) > key) {
            op += 4; // comparação jt > begin, subtração jt-1, acesso *(jt-1) e comparação *(jt-1) > key
            op += 4; // subtração jt-1, acesso *(jt-1) e *j, além da atribuição *(jt-1) a *j
            *jt = *(jt-1);
            --jt;
            op += 2; // decremento e atribuição ao jt
        }
        op += 4; // comparação jt > begin, subtração jt-1, acesso *(jt-1) e comparação *(jt-1) > key ao sair do laço
        op += 2; // acesso *jt e atribuição key a *jt
        *jt = key;
        op += 2; // incremento e atribuição do it
    }
    op += 1; // comparação it < end ao sair do laço
    return op
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

    std::vector<int> arr = std::vector<int>(n);
    std::srand(seed);
    std::generate(arr.begin(), arr.end(), std::rand);
    
    unsigned long long op = insertion_sort_ram(arr.begin(), arr.end());
    
    // printf("Quantidade de operações no modelo RAM: %d\n", op);
    printf("%llu\n", op);

    free(arr);
    return 0;
}
