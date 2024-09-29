CC = clang++
CFLAGS = -Wall -O3
all: ./bin/insertion_sort ./bin/insertion_sort_ram

./bin/insertion_sort: ./bin/insertion_sort.o
	$(CC) $(CFLAGS) -o ./bin/insertion_sort ./bin/insertion_sort.o

./bin/insertion_sort_ram: ./bin/insertion_sort_ram.o
	$(CC) $(CFLAGS) -o ./bin/insertion_sort_ram ./bin/insertion_sort_ram.o

./bin/insertion_sort.o: src/insertion_sort.cpp
	$(CC) $(CFLAGS) -c src/insertion_sort.cpp -o ./bin/insertion_sort.o

./bin/insertion_sort_ram.o: src/insertion_sort_ram.cpp
	$(CC) $(CFLAGS) -c src/insertion_sort_ram.cpp -o ./bin/insertion_sort_ram.o

clean:
	rm -f ./bin/*