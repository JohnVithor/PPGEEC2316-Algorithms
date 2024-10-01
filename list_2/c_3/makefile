CC = clang
CFLAGS = -Wall -O3
all: mkdir ./bin/task_allocation 

mkdir:
	mkdir -p ./bin

./bin/task_allocation: ./bin/task_allocation.o
	$(CC) $(CFLAGS) -o ./bin/task_allocation ./bin/task_allocation.o

./bin/task_allocation.o: src/task_allocation.c
	$(CC) $(CFLAGS) -c src/task_allocation.c -o ./bin/task_allocation.o

clean:
	rm -f ./bin/*