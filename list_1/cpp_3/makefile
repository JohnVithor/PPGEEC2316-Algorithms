CC = clang++
CFLAGS = -Wall -O3
all: ./bin/insertion_sort ./bin/merge_sort

./bin/insertion_sort: ./bin/insertion_sort.o ./bin/utils.o
	$(CC) $(CFLAGS) -I./include -o ./bin/insertion_sort ./bin/insertion_sort.o ./bin/utils.o

./bin/merge_sort: ./bin/merge_sort.o ./bin/utils.o
	$(CC) $(CFLAGS) -I./include -o ./bin/merge_sort ./bin/merge_sort.o ./bin/utils.o

./bin/insertion_sort.o: src/insertion_sort.cpp
	$(CC) $(CFLAGS) -c src/insertion_sort.cpp -I./include -o ./bin/insertion_sort.o

./bin/merge_sort.o: src/merge_sort.cpp
	$(CC) $(CFLAGS) -c src/merge_sort.cpp -I./include -o ./bin/merge_sort.o

./bin/utils.o: src/utils.cpp include/utils.hpp
	$(CC) $(CFLAGS) -c src/utils.cpp -I./include -o ./bin/utils.o

clean:
	rm -f ./bin/*