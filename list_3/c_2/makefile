CC = clang
CFLAGS = -Wall -O3
all: mkdir ./bin/quick_sort ./bin/randomized_quick_sort

mkdir:
	mkdir -p ./bin

./bin/quick_sort: ./bin/quick_sort.o ./bin/utils.o
	$(CC) $(CFLAGS) -I./include -o ./bin/quick_sort ./bin/quick_sort.o ./bin/utils.o

./bin/randomized_quick_sort: ./bin/randomized_quick_sort.o ./bin/utils.o
	$(CC) $(CFLAGS) -I./include -o ./bin/randomized_quick_sort ./bin/randomized_quick_sort.o ./bin/utils.o

./bin/quick_sort.o: src/quick_sort.c
	$(CC) $(CFLAGS) -c src/quick_sort.c -I./include -o ./bin/quick_sort.o

./bin/randomized_quick_sort.o: src/randomized_quick_sort.c
	$(CC) $(CFLAGS) -c src/randomized_quick_sort.c -I./include -o ./bin/randomized_quick_sort.o

./bin/utils.o: src/utils.c include/utils.h
	$(CC) $(CFLAGS) -c src/utils.c -I./include -o ./bin/utils.o

clean:
	rm -f ./bin/*