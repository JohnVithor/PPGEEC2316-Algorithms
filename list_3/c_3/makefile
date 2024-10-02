CC = clang
CFLAGS = -Wall -O3
all: mkdir ./bin/count_sort ./bin/radix_sort

mkdir:
	mkdir -p ./bin

./bin/count_sort: ./bin/count_sort.o ./bin/utils.o
	$(CC) $(CFLAGS) -I./include -o ./bin/count_sort ./bin/count_sort.o ./bin/utils.o

./bin/radix_sort: ./bin/radix_sort.o ./bin/utils.o
	$(CC) $(CFLAGS) -I./include -o ./bin/radix_sort ./bin/radix_sort.o ./bin/utils.o

./bin/count_sort.o: src/count_sort.c
	$(CC) $(CFLAGS) -c src/count_sort.c -I./include -o ./bin/count_sort.o

./bin/radix_sort.o: src/radix_sort.c
	$(CC) $(CFLAGS) -c src/radix_sort.c -I./include -o ./bin/radix_sort.o

./bin/utils.o: src/utils.c include/utils.h
	$(CC) $(CFLAGS) -c src/utils.c -I./include -o ./bin/utils.o

clean:
	rm -f ./bin/*