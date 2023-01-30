/*
## string literals

### Instructions

Create the following functions:

- `is_empty`, that returns true if a string is empty
- `is_ascii`, that returns true if all characters of a given string is in ASCII range
- `contains`, that returns true if the string contains a pattern given
- `split_at`, that divides a string in two returning a tuple
- `find', that returns the index if the first character of a given string that matches the pattern

> This exercise will test the **heap allocation** of your function!
> So try your best to allocate the minimum data on the heap! (hit: &str)

### Notions

- https://doc.rust-lang.org/1.22.0/book/first-edition/the-stack-and-the-heap.html
- https://doc.rust-lang.org/rust-by-example/primitives/literals.html

*/
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

#[allow(unused_imports)]
use string_literals::*;

thread_local! {
    static ALLOC_VIOLATION_COUNT: Cell<u32> = Cell::new(0);
}

#[allow(dead_code)]
fn violation_count() -> u32 {
    ALLOC_VIOLATION_COUNT.with(|c| c.get())
}
#[allow(dead_code)]
fn reset_violation_count() {
    ALLOC_VIOLATION_COUNT.with(|c| c.set(0));
}

struct Counter;

unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOC_VIOLATION_COUNT.with(|c| c.set(c.get() + 1));
        let ret = System.alloc(layout);
        return ret;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        ALLOC_VIOLATION_COUNT.with(|c| c.set(c.get() + 1));
        System.dealloc(ptr, layout);
    }
}

#[global_allocator]
static A: Counter = Counter;

#[test]
fn test_memory() {
    let no_alloc = violation_count();
    reset_violation_count();
    is_empty("");
    is_ascii("rust");
    contains("rust", "ru");
    split_at("rust", 2);
    find("rust", 'u');
    let stu_alloc = violation_count();
    assert!(
        stu_alloc < no_alloc,
        "You are allocating to the heap {} times, you must not allocate to the heap",
        stu_alloc
    );
}

#[test]
fn test_functions() {
    assert!(is_empty(""));
    assert!(!is_empty("something"));
    assert!(is_ascii("rust"));
    assert!(!is_ascii("rust¬"));
    assert!(contains("rust", "ru"));
    assert!(!contains("something", "mer"));
    assert_eq!(split_at("rust", 2), ("ru", "st"));
    assert_eq!(find("ru-st-e", '-'), 2);
    assert_eq!(find("ru-st-e", 'e'), 6);
}

fn main() {
    println!("{}", is_empty(""));
    println!("{}", is_ascii("rust"));
    println!("{}", contains("rust", "ru"));
    println!("{:?}", split_at("rust", 2));
    println!("{}", find("rust", 'u'));
}
