# Questões 1

1. (Exercício 2.1-3 do livro do Cormen) Escreva um pseudocódigo para a busca linear e mostre, usando invariância de laço, que o seu algoritmo está correto.

2. Implemente o algoritmo de ordenação por inserção e crie uma cópia anotada dele que mede o número de operações no modelo da Random Access Machine (RAM, seção 2.2 livro do Cormen). Usando entradas de tamanho crescente, mostre em um gráfico quando o tempo de execução no modelo RAM diverge de medições feitas em uma máquina real.

3. Mostre numericamente com suas implementações dos algoritmos de insertion-sort e merge-sort como se comporta o desempenho de cada algoritmo utilizando entradas de tamanho crescente, considerando entradas de pior caso, melhor caso e caso médio. Análise, para cada tipo de entrada, se existe algum ponto a partir do qual um algoritmo passa a ser mais rápido que o outro.

# Questões 2

1. Mostre numericamente com suas implementações dos algoritmos de multiplicação de matrizes que o algoritmo de Strassen é mais rápido que o algoritmo convencional.

2. Escolha um algoritmo recorrente para aplicar um dos 4 métodos de resolução de recorrência descritos no capítulo 4 para medir o custo da recorrência do algoritmo escolhido. Compare o resultado com medições de tempo.

3. O problema de balanceamento de cargas busca atribuir tarefas de tamanhos diferentes a trabalhadores, de modo a minimizar a carga máxima que um trabalhador irá executar. Em um problema em que temos n tarefas e k trabalhadores (n > k), considere que o balanceador irá distribuir as n/k primeiras tarefas para o primeiro trabalhador, as n/k tarefas seguintes para o segundo trabalhador, e assim por diante. Mostre numericamente como permutar aleatoriamente os dados de entrada, que são as cargas de cada tarefa, pode influenciar na solução desse balanceador.

# Questões 3

1. Implemente as funções da seção 6.5 do livro do Cormen 4th Ed. em sua linguagem favorita e proponha um exemplo de uso com uma demonstração.

2. Mostre com experimentos numéricos quando suas próprias implementações de Quicksort e do Quicksort aleatório são mais vantajosas quando comparadas uma com a outra.

3. Mostre com experimentos numéricos quando o Radix-sort com o Count-sort é mais rápido que o Count-sort sozinho. Utilize suas próprias implementações ou alguma implementação existente explicando os resultados.

# Questões 4

1. Implemente o algoritmo de mínimo e máximo simultâneos da seção 9.1 do livro do Cormen, 4a Ed na sua linguagem favorita e mostre através de medição de tempo que é mais rápido que a abordagem não-simultânea para um vetor de entrada suficientemente grande.

2. Implemente os algoritmos de seleção aleatória e seleção das seções 9.2 2 9.3 do livro do Cormen, 4a Ed., e realize experimentos numéricos para demonstrar em quais casos um tem vantagens com relação ao outro.

3. Implemente o algoritmo da mediana ponderada e use-o para resolver o item e do Problema 9-3 do Cormen, 4a Ed.
