CC = clang
CFLAGS = -Wall -O3
all: mkdir ./bin/binary_search

mkdir:
	mkdir -p ./bin

./bin/binary_search: ./bin/binary_search.o
	$(CC) $(CFLAGS) -o ./bin/binary_search ./bin/binary_search.o

./bin/binary_search.o: src/binary_search.c
	$(CC) $(CFLAGS) -c src/binary_search.c -o ./bin/binary_search.o

clean:
	rm -f ./bin/*