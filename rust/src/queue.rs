use crate::raw_vec::{RawVec, RawVecError};
use core::fmt::Debug;

#[derive(Debug)]
pub enum QueueError {
    Memory(RawVecError),
    Full,
    Empty,
}

#[derive(Default)]
pub struct Queue<T> {
    buffer: RawVec<T>,
    head: usize,
    tail: usize,
    size: usize,
}

impl<T: core::fmt::Debug> core::fmt::Debug for Queue<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let buffer =
            unsafe { core::slice::from_raw_parts(self.buffer.data.as_ptr(), self.buffer.cap) };
        let slice: &dyn Debug = if self.tail > self.head {
            &buffer[self.head..self.tail].iter()
        } else {
            let start = buffer[self.head..].iter();
            let end = buffer[..self.tail].iter();
            &start.chain(end)
        };
        f.debug_struct("Queue")
            .field("buffer", slice)
            .field("cap", &self.buffer.cap)
            .field("len", &self.size)
            .finish()
    }
}

impl<T> Queue<T> {
    pub fn new(cap: usize) -> Result<Self, QueueError> {
        let buffer = match RawVec::new(cap) {
            Ok(buffer) => buffer,
            Err(e) => return Err(QueueError::Memory(e)),
        };
        Ok(Self {
            buffer,
            head: 0,
            tail: 0,
            size: 0,
        })
    }

    pub fn enqueue(&mut self, value: T) -> Result<(), QueueError> {
        if self.size == self.buffer.cap {
            return Err(QueueError::Full);
        }
        unsafe {
            self.buffer.data.as_ptr().add(self.tail).write(value);
        }
        if self.tail == self.buffer.cap {
            self.tail = 0;
        } else {
            self.tail += 1;
        }
        self.size += 1;
        Ok(())
    }

    pub fn dequeue(&mut self) -> Result<T, QueueError> {
        if self.size == 0 {
            return Err(QueueError::Empty);
        }
        self.size -= 1;
        let r = unsafe { Ok(self.buffer.data.as_ptr().add(self.head).read()) };
        if self.head == self.buffer.cap {
            self.head = 0;
        } else {
            self.head += 1;
        }
        r
    }
}
