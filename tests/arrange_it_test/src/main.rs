use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

#[allow(unused_imports)]
use arrange_it::*;

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

#[allow(dead_code)]
fn arrange_phrase_sol(phrase: &str) -> String {
    let nbrs: Vec<&str> = phrase.matches(char::is_numeric).collect();
    let a = &phrase.replace(char::is_numeric, "");
    let mut m: Vec<&str> = a.split_whitespace().collect();

    for (i, ele) in nbrs.iter().enumerate() {
        let strs: Vec<&str> = a.split_whitespace().collect();
        m[ele.parse::<usize>().unwrap() - 1] = strs[i];
    }
    m.join(" ")
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
fn test_heap_memory_allocation() {
    let test_value = "w7ork t3he a4rt o5f Per1formance is2 a6voiding";
    arrange_phrase_sol(test_value);
    let sol_alloc = violation_count();
    reset_violation_count();
    arrange_phrase(test_value);
    let stu_alloc = violation_count();
    assert!(
		stu_alloc <= sol_alloc,
		"You are allocating to the heap {stu_alloc} times, and it must be less or equal to {sol_alloc} times"
	);
}

#[test]
fn test_function() {
    let cases = vec![
        "4of Fo1r pe6ople g3ood th5e the2",
        "is2 Thi1s T4est 3a",
        "w7ork t3he a4rt o5f Per1formance is2 a6voiding",
    ];
    for v in cases {
        assert_eq!(arrange_phrase(v), arrange_phrase_sol(v));
    }
}

#[allow(dead_code)]
fn main() {
    println!("{:?}", arrange_phrase("is2 Thi1s T4est 3a"));
}
