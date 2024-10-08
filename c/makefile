CC = clang
CFLAGS = -Wall -O3

all: mkdir ./bin/sort ./bin/sort_ram ./bin/matrix ./bin/order_statistics

debug: CFLAGS = -Wall -O0 -g
debug: all

mkdir:
	mkdir -p ./bin

# Sort

./bin/sort_ram: ./bin/sort_ram.o ./bin/utils.o ./bin/insertion_sort.o ./bin/merge_sort.o ./bin/quick_sort.o ./bin/randomized_quick_sort.o ./bin/count_sort.o ./bin/radix_sort.o
	$(CC) $(CFLAGS) -o $@ $^

./bin/sort: ./bin/sort.o ./bin/utils.o ./bin/insertion_sort.o ./bin/merge_sort.o ./bin/quick_sort.o ./bin/randomized_quick_sort.o ./bin/count_sort.o ./bin/radix_sort.o
	$(CC) $(CFLAGS) -o $@ $^

./bin/sort.o: src/runners/sort.c
	$(CC) $(CFLAGS) -I./include -c $^ -o $@

./bin/sort_ram.o: src/runners/sort_ram.c
	$(CC) $(CFLAGS) -I./include -c $^ -o $@

./bin/insertion_sort.o: src/sort/insertion_sort.c
	$(CC) $(CFLAGS) -I./include -c $^ -o $@ 

./bin/merge_sort.o: src/sort/merge_sort.c
	$(CC) $(CFLAGS) -I./include -c $^ -o $@ 

./bin/quick_sort.o: src/sort/quick_sort.c
	$(CC) $(CFLAGS) -I./include -c $^ -o $@ 

./bin/randomized_quick_sort.o: src/sort/randomized_quick_sort.c
	$(CC) $(CFLAGS) -I./include -c $^ -o $@ 

./bin/count_sort.o: src/sort/count_sort.c
	$(CC) $(CFLAGS) -I./include -c $^ -o $@ 

./bin/radix_sort.o: src/sort/radix_sort.c
	$(CC) $(CFLAGS) -I./include -c $^ -o $@ 

./bin/utils.o: src/utils.c
	$(CC) $(CFLAGS) -I./include -c $^ -o $@ 

# Matrix

./bin/matrix: ./bin/matrix.o ./bin/utils.o ./bin/classic.o ./bin/strassen.o
	$(CC) $(CFLAGS) -o $@ $^

./bin/matrix.o: src/runners/matrix.c
	$(CC) $(CFLAGS) -I./include -c $^ -o $@

./bin/classic.o: src/matrix/classic.c
	$(CC) $(CFLAGS) -I./include -c $^ -o $@

./bin/strassen.o: src/matrix/strassen.c
	$(CC) $(CFLAGS) -I./include -c $^ -o $@

# Order statistics

./bin/order_statistics: ./bin/order_statistics.o ./bin/minimum_maximum.o ./bin/utils.o
	$(CC) $(CFLAGS) -o $@ $^

./bin/order_statistics.o: src/runners/order_statistics.c 
	$(CC) $(CFLAGS) -I./include -c $^ -o $@

./bin/minimum_maximum.o: src/order_statistics/minimum_maximum.c
	$(CC) $(CFLAGS) -I./include -c $^ -o $@

clean:
	rm -f ./bin/*