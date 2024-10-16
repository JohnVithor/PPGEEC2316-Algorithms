#ifndef _STACK_H_
#define _STACK_H_
#include <stddef.h>

typedef int T;
typedef struct Stack {
  T* data;
  size_t capacity;
  size_t top;
} Stack;

typedef enum StackError {
  EMPTY,
  FULL,
  NOT_FOUND
} StackError;

typedef struct StackResult {
  char ok;
  union {
    T data;
    StackError status;
  };
} StackResult;


Stack* create_stack(size_t capacity);
char is_empty(Stack* stack, T data);
char is_full(Stack* stack, T data);
StackResult push(Stack* stack, T data);
StackResult pop(Stack* stack);

#endif  // _STACK_H_
