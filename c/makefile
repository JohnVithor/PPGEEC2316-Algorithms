CC = clang
CFLAGS = -Wall -O3

all: mkdir ./bin/insertion_sort_1 ./bin/insertion_sort_ram

debug: CFLAGS = -Wall -O0 -g
debug: all

mkdir:
	mkdir -p ./bin

# Q 1.2
./bin/insertion_sort: ./bin/insertion_sort.o
	$(CC) $(CFLAGS) -o ./bin/insertion_sort ./bin/insertion_sort.o

./bin/insertion_sort_ram: ./bin/insertion_sort_ram.o
	$(CC) $(CFLAGS) -o ./bin/insertion_sort_ram ./bin/insertion_sort_ram.o

./bin/insertion_sort.o: src/insertion_sort.c
	$(CC) $(CFLAGS) -c src/insertion_sort.c -o ./bin/insertion_sort.o

./bin/insertion_sort_ram.o: src/insertion_sort_ram.c
	$(CC) $(CFLAGS) -c src/insertion_sort_ram.c -o ./bin/insertion_sort_ram.o


./bin/priority_queue: ./bin/priority_queue.o ./bin/binary_heap.o ./bin/utils.o
	$(CC) $(CFLAGS) -I./include ./bin/priority_queue.o ./bin/binary_heap.o ./bin/utils.o -o ./bin/priority_queue

./bin/priority_queue.o: src/priority_queue.c ./bin/binary_heap.o ./bin/utils.o
	$(CC) $(CFLAGS) -I./include -c src/priority_queue.c -o ./bin/priority_queue.o

./bin/binary_heap.o: src/binary_heap.c include/binary_heap.h
	$(CC) $(CFLAGS) -c src/binary_heap.c -I./include -o ./bin/binary_heap.o

./bin/utils.o: src/utils.c include/utils.h
	$(CC) $(CFLAGS) -c src/utils.c -I./include -o ./bin/utils.o

clean:
	rm -f ./bin/*