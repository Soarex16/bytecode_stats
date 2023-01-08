
all: gc_runtime.o runtime.o
	ar rc libruntime.a gc_runtime.o runtime.o

gc_runtime.o: gc_runtime.s
	$(CC) -g -fstack-protector-all -m32 -c gc_runtime.s

runtime.o: runtime.c runtime.h
	$(CC) -g -fstack-protector-all -m32 -c runtime.c

test: runtime_test.c runtime.c runtime.h gc_runtime.o
	$(CC) -g -fstack-protector-all -m32 -c runtime_test.c runtime.c
	gcc -m32 -o runtime_test runtime_test.o gc_runtime.o

clean:
	$(RM) *.a *.o *~
