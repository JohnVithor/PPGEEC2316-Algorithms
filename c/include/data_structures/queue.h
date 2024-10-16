#ifndef _QUEUE_H_
#define _QUEUE_H_

#include <stddef.h>

typedef int T;
typedef struct Queue {
  T* data;
  size_t capacity;
  size_t head;
  size_t tail;
} Queue;

typedef enum QueueError {
  EMPTY,
  FULL,
  NOT_FOUND
} QueueError;

typedef struct QueueResult {
  char ok;
  union {
    T data;
    QueueError status;
  };
} QueueResult;


Queue* create_Queue(size_t capacity);
char is_empty(Queue* queue, T data);
char is_full(Queue* queue, T data);
QueueResult enqueue(Queue* queue, T data);
QueueResult dequeue(Queue* queue);

#endif  // _QUEUE_H_
