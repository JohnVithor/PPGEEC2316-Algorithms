extern crate alloc;
use alloc::alloc::{alloc, dealloc, realloc};
use core::{alloc::Layout, mem, ptr::NonNull};

#[derive(Debug)]
pub enum RawVecError {
    OutOfMemory,
    OnMaxSize,
    OverMaxSize,
}

pub struct RawVec<T> {
    pub data: NonNull<T>,
    pub cap: usize,
}

impl<T: core::fmt::Debug> core::fmt::Debug for RawVec<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RawVec")
            .field("data", unsafe {
                &core::slice::from_raw_parts(self.data.as_ptr(), self.cap).iter()
            })
            .field("cap", &self.cap)
            .finish()
    }
}

impl<T> Default for RawVec<T> {
    fn default() -> Self {
        let cap = if mem::size_of::<T>() == 0 { !0 } else { 0 };
        Self {
            data: NonNull::dangling(),
            cap,
        }
    }
}

impl<T> RawVec<T> {
    pub fn new(cap: usize) -> Result<Self, RawVecError> {
        let layout = match Layout::array::<T>(cap) {
            Ok(layout) => layout,
            Err(_) => return Err(RawVecError::OverMaxSize),
        };

        if layout.size() >= isize::MAX as usize {
            return Err(RawVecError::OverMaxSize);
        }

        let ptr = unsafe { alloc(layout) };

        let ptr = match NonNull::new(ptr as *mut T) {
            Some(p) => p,
            None => return Err(RawVecError::OutOfMemory),
        };
        Ok(Self { data: ptr, cap })
    }

    pub fn grow(&mut self) -> Result<(), RawVecError> {
        if mem::size_of::<T>() != 0 {
            return Err(RawVecError::OnMaxSize);
        }

        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 2 * self.cap;
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        if new_layout.size() >= isize::MAX as usize {
            return Err(RawVecError::OverMaxSize);
        }

        let new_ptr = if self.cap == 0 {
            unsafe { alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.data.as_ptr() as *mut u8;
            unsafe { realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.data = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => return Err(RawVecError::OutOfMemory),
        };
        self.cap = new_cap;
        Ok(())
    }

    pub fn get(&self, index: usize) -> &T {
        unsafe { &*self.data.as_ptr().add(index) }
    }

    pub fn set(&mut self, index: usize, value: T) {
        unsafe {
            *self.data.as_ptr().add(index) = value;
        }
    }

    pub fn fill(&mut self, value: T)
    where
        T: Copy,
    {
        for i in 0..self.cap {
            self.set(i, value);
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 && mem::size_of::<T>() != 0 {
            unsafe {
                dealloc(
                    self.data.as_ptr() as *mut u8,
                    Layout::array::<T>(self.cap).unwrap(),
                );
            }
        }
    }
}
