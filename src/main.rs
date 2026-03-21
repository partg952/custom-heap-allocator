use std::{
    alloc::{GlobalAlloc, Layout},
    cell::RefCell,
    ptr::{addr_of_mut, null_mut},
    sync::Mutex,
};

//using unsafe at a lot of places for the following reasons:-
// - dealing with raw pointers and not traditional rust references
// - using mutable static BUFFER, since it is global and mutable, borrow checker can't track it , hence wrapped in an unsafe block whenever being used


static N: usize = 1024;
static mut BUFFER: [u8; N] = [0; N];
#[derive(Debug)]
struct BumpAllocator {
    start: *mut u8,
    end: *mut u8,
    current: Mutex<*mut u8>,
}
unsafe impl Sync for BumpAllocator {}
unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alignment = layout.align();
        let size = layout.size();
        let mut current = self.current.lock().unwrap(); //locking the current in this thread so only this read can write onto it
        let remainder = *current as usize % alignment;
        let mut aligned: *mut u8 = *current;
        if remainder != 0 {
            aligned = (*current as usize + (alignment - remainder)) as *mut u8; //calculating the next aligned the address if current is not aligned
        }
        if aligned as usize + size > self.end as usize {
            return null_mut(); //if requested amount is more than the available space, null pointer is returned
        }
        *current = (aligned as usize + size) as *mut u8; //moving the current to the next available address
        // println!("{} bytes allocated, The allocator is working!!" , size); will get stuck since this also calls the allocator and causes infinite recursion.
        return aligned;
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {}
}
#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator {
    start: unsafe { addr_of_mut!(BUFFER) as *mut u8 },
    end: unsafe { (addr_of_mut!(BUFFER) as *mut u8).add(N) },
    current: Mutex::new(unsafe { addr_of_mut!(BUFFER) as *mut u8 }),
};

fn main() {
    let v = vec![1.1, 2.2];
}
