CC = clang
CFLAGS = -Wall -O3
all: mkdir ./bin/classic ./bin/strassen 

mkdir:
	mkdir -p ./bin

./bin/classic: ./bin/classic.o
	$(CC) $(CFLAGS) -o ./bin/classic ./bin/classic.o

./bin/classic.o: src/classic.c
	$(CC) $(CFLAGS) -c src/classic.c -o ./bin/classic.o

./bin/strassen: ./bin/strassen.o
	$(CC) $(CFLAGS) -o ./bin/strassen ./bin/strassen.o

./bin/strassen.o: src/strassen.c
	$(CC) $(CFLAGS) -c src/strassen.c -o ./bin/strassen.o

clean:
	rm -f ./bin/*