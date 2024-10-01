#include <stdio.h>
#include <stdlib.h>
#include <time.h>

void add(int* a, int* b, int* c, unsigned int n) {
    for(unsigned int i = 0; i < n; ++i) {
        for(unsigned int j = 0; j < n; ++j) {
            c[i * n + j] = a[i * n + j] + b[i * n + j];
        }
    }
}

void subtract(int* a, int* b, int* c, unsigned int n) {
    for(unsigned int i = 0; i < n; ++i) {
        for(unsigned int j = 0; j < n; ++j) {
            c[i * n + j] = a[i * n + j] - b[i * n + j];
        }
    }
}

void multiply(int* a, int* b, int* c, unsigned int n) {
    if (n == 1) {
        c[0] = a[0] * b[0];
        return;
    }

    unsigned int mid = n / 2;

    int* a11 = malloc(mid*mid*sizeof(int));
    int* a12 = malloc(mid*mid*sizeof(int));
    int* a21 = malloc(mid*mid*sizeof(int));
    int* a22 = malloc(mid*mid*sizeof(int));
    int* b11 = malloc(mid*mid*sizeof(int));
    int* b12 = malloc(mid*mid*sizeof(int));
    int* b21 = malloc(mid*mid*sizeof(int));
    int* b22 = malloc(mid*mid*sizeof(int));
    int* c11 = malloc(mid*mid*sizeof(int));
    int* c12 = malloc(mid*mid*sizeof(int));
    int* c21 = malloc(mid*mid*sizeof(int));
    int* c22 = malloc(mid*mid*sizeof(int));

    int* m1 = malloc(mid*mid*sizeof(int));
    int* m2 = malloc(mid*mid*sizeof(int));
    int* m3 = malloc(mid*mid*sizeof(int));
    int* m4 = malloc(mid*mid*sizeof(int));
    int* m5 = malloc(mid*mid*sizeof(int));
    int* m6 = malloc(mid*mid*sizeof(int));
    int* m7 = malloc(mid*mid*sizeof(int));

    int* aux1 = malloc(mid*mid*sizeof(int));
    int* aux2 = malloc(mid*mid*sizeof(int));

    for (unsigned int i = 0; i < mid; ++i) {
        for (unsigned int j = 0; j < mid; ++j) {
            a11[i * mid + j] = a[i         * n + j      ];
            a12[i * mid + j] = a[i         * n + j + mid];
            a21[i * mid + j] = a[(i + mid) * n + j      ];
            a22[i * mid + j] = a[(i + mid) * n + j + mid];
            b11[i * mid + j] = b[i         * n + j      ];
            b12[i * mid + j] = b[i         * n + j + mid];
            b21[i * mid + j] = b[(i + mid) * n + j      ];
            b22[i * mid + j] = b[(i + mid) * n + j + mid];
        }
    }

    add(a11, a22, aux1, mid);
    add(b11, b22, aux2, mid);
    multiply(aux1, aux2, m1, mid);
    
    add(a21, a22, aux1, mid);
    multiply(aux1, b11, m2, mid);

    subtract(b12, b22, aux2, mid);
    multiply(a11, aux2, m3, mid);

    subtract(b21, b11, aux1, mid);
    multiply(a22, aux1, m4, mid);

    add(a11, a12, aux2, mid);
    multiply(aux2, b22, m5, mid);

    subtract(a21, a11, aux1, mid);
    add(b11, b12, aux2, mid);
    multiply(aux1, aux2, m6, mid);

    subtract(a12, a22, aux1, mid);
    add(b21, b22, aux2, mid);
    multiply(aux1, aux2, m7, mid);

    for (unsigned int i = 0; i < mid; ++i) {
        for (unsigned int j = 0; j < mid; ++j) {
            c11[i * mid + j] = m1[i * mid + j] + m4[i * mid + j] - m5[i * mid + j] + m7[i * mid + j];
            c12[i * mid + j] = m3[i * mid + j] + m5[i * mid + j];
            c21[i * mid + j] = m2[i * mid + j] + m4[i * mid + j];
            c22[i * mid + j] = m1[i * mid + j] - m2[i * mid + j] + m3[i * mid + j] + m6[i * mid + j];
        }
    }

    for (unsigned int i = 0; i < mid; ++i) {
        for (unsigned int j = 0; j < mid; ++j) {
            c[i         * n + j       ] = c11[i * mid + j];
            c[i         * n + j + mid ] = c12[i * mid + j];
            c[(i + mid) * n + j       ] = c21[i * mid + j];
            c[(i + mid) * n + j + mid ] = c22[i * mid + j];
        }
    }

    free(a11);
    free(a12);
    free(a21);
    free(a22);
    free(b11);
    free(b12);
    free(b21);
    free(b22);
    free(c11);
    free(c12);
    free(c21);
    free(c22);

    free(m1);
    free(m2);
    free(m3);
    free(m4);
    free(m5);
    free(m6);
    free(m7);

    free(aux1);
    free(aux2);
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
    int* a = (int*) malloc(n*n * sizeof(int)); 
    int* b = (int*) malloc(n*n * sizeof(int)); 
    int* c = (int*) malloc(n*n * sizeof(int)); 
    for (int i = 0; i < n*n; i++){
        a[i] = rand() % (n*n);
        b[i] = rand() % (n*n);
    }
    
    clock_t start_time = clock();
    multiply(a, b, c, n);
    clock_t end_time = clock();
    double time_taken = (double)(end_time - start_time) / CLOCKS_PER_SEC;
    printf("%lf\n", time_taken);

    for (int i = 0; i < n*n; i++){
        printf("%d ", a[i]);
    }
    printf("\n");
    for (int i = 0; i < n*n; i++){
        printf("%d ", b[i]);
    }
    printf("\n");
    for (int i = 0; i < n*n; i++){
        printf("%d ", c[i]);
    }
    printf("\n");

    free(a);
    free(b);
    free(c);
    return 0;
}
