CC = clang
CFLAGS = -Wall -O3

all: mkdir ./bin/sort

debug: CFLAGS = -Wall -O0 -g
debug: all

mkdir:
	mkdir -p ./bin

./bin/sort: ./bin/sort.o ./bin/utils.o ./bin/insertion_sort.o ./bin/merge_sort.o ./bin/quick_sort.o ./bin/randomized_quick_sort.o ./bin/count_sort.o ./bin/radix_sort.o
	$(CC) $(CFLAGS) -o $@ $^

./bin/sort.o: src/runners/sort.c
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

clean:
	rm -f ./bin/*