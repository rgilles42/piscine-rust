use std::{
    alloc::{self, GlobalAlloc, Layout},
    sync::atomic::{AtomicUsize, Ordering},
};
use string_literals::*;

struct CounterAlloc {
    counter: AtomicUsize,
}

#[allow(dead_code)] // incorrect false positive!
impl CounterAlloc {
    #[inline]
    fn reset_counter(&self) {
        self.counter.store(0, Ordering::SeqCst);
    }

    #[inline]
    fn counter(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }
}

unsafe impl GlobalAlloc for CounterAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { alloc::System.alloc(layout) };
        self.counter.fetch_add(1, Ordering::SeqCst);
        return ptr;
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe {
            alloc::System.dealloc(ptr, layout);
        }
    }
}

#[global_allocator]
static ALLOCATOR: CounterAlloc = CounterAlloc {
    counter: AtomicUsize::new(0),
};

fn main() {
    println!("{}", is_empty(""));
    println!("{}", is_ascii("rust"));
    println!("{}", contains("rust", "ru"));
    println!("{:?}", split_at("rust", 2));
    println!("{}", find("rust", 'u'));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functions_and_memory_allocation() {
        ALLOCATOR.reset_counter();

        assert!(is_empty(""));
        assert!(!is_empty("something"));
        assert!(is_ascii("rust"));
        assert!(!is_ascii("rust¬"));
        assert!(contains("rust", "ru"));
        assert!(!contains("something", "mer"));
        assert_eq!(split_at("rust", 2), ("ru", "st"));
        assert_eq!(find("ru-st-e", '-'), 2);
        assert_eq!(find("ru-st-e", 'e'), 6);

        assert_eq!(ALLOCATOR.counter(), 0);
    }
}
