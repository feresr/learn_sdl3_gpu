use std::{alloc::Layout};

pub struct Arena<const N: usize> {
    data: [u8; N],
    offset: usize,
}

impl<const N: usize> Default for Arena<N> {
    fn default() -> Self {
        Self {
            data: [0; N],
            offset: 0,
        }
    }
}

impl<const N: usize> Arena<N> {

    pub fn alloc<T>(&mut self, value: T) -> &mut T {
        let layout = Layout::new::<T>();
        let ptr = self.alloc_layout(layout);

        unsafe {
            let typed_ptr = ptr as *mut T;
            typed_ptr.write(value);
            &mut *typed_ptr
        }
    }

    /// Allocates raw memory with the given layout
    fn alloc_layout(&mut self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // Calculate aligned offset
        let current_ptr = self.data.as_ptr() as usize + self.offset;
        let aligned_ptr = (current_ptr + align - 1) & !(align - 1);
        let aligned_offset = aligned_ptr - self.data.as_ptr() as usize;

        // Check if we have enough space
        if aligned_offset + size > N {
            panic!("Not enough space in Arena to allocate T")
        }

        let ptr = unsafe { self.data.as_mut_ptr().add(aligned_offset) };

        self.offset = aligned_offset + size;
        ptr
    }

    /// Returns the number of bytes used
    pub fn bytes_used(&self) -> usize {
        self.offset
    }

    /// Returns the remaining capacity in bytes
    pub fn remaining_capacity(&self) -> usize {
        N - self.offset
    }

    /// Returns true if the arena is empty
    pub fn is_empty(&self) -> bool {
        self.offset == 0
    }

    /// Resets the arena (doesn't call destructors - use with caution!)
    pub unsafe fn reset(&mut self) {
        self.offset = 0;
    }

    /// Gets the total capacity of the arena
    pub fn capacity(&self) -> usize {
        N
    }
}
