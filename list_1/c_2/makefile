CC = clang
CFLAGS = -Wall -O3
all: mkdir ./bin/insertion_sort ./bin/insertion_sort_ram

mkdir:
	mkdir -p ./bin

./bin/insertion_sort: ./bin/insertion_sort.o
	$(CC) $(CFLAGS) -o ./bin/insertion_sort ./bin/insertion_sort.o

./bin/insertion_sort_ram: ./bin/insertion_sort_ram.o
	$(CC) $(CFLAGS) -o ./bin/insertion_sort_ram ./bin/insertion_sort_ram.o

./bin/insertion_sort.o: src/insertion_sort.c
	$(CC) $(CFLAGS) -c src/insertion_sort.c -o ./bin/insertion_sort.o

./bin/insertion_sort_ram.o: src/insertion_sort_ram.c
	$(CC) $(CFLAGS) -c src/insertion_sort_ram.c -o ./bin/insertion_sort_ram.o

clean:
	rm -f ./bin/*