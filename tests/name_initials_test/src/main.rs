/*
## name_initials

### Instructions

Create a function called `initials`, this function will receive a vector of string literals
with names and return a vector of Strings with the initials of each name.

### Example:

```rust
```

> This exercise will test the **heap allocation** of your function!
> So try your best to allocate the minimum data on the heap!

### Notions

- https://doc.rust-lang.org/1.22.0/book/first-edition/the-stack-and-the-heap.html

*/
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

#[allow(unused_imports)]
use name_initials::*;

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

// this solutions uses heap allocation but is not optimized
// to use the less possible. The reason for this is so that
// we give some space for the student to implement their own way
#[allow(dead_code)]
fn initials_sol(arr: Vec<&str>) -> Vec<String> {
    arr.iter()
        .map(|ele| {
            let mut names = ele.split_whitespace();
            let mut a = names.next().unwrap().chars().nth(0).unwrap().to_string();
            a.push_str(". ");
            let mut b = names.next().unwrap().chars().nth(0).unwrap().to_string();
            b.push_str(".");
            a.push_str(&b);
            a
        })
        .collect()
}

#[test]
fn test_memory_allocation() {
    let test_value = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    initials_sol(test_value.clone());
    let sol_alloc = violation_count();
    reset_violation_count();
    initials(test_value);
    let stu_alloc = violation_count();

    assert!(
        stu_alloc <= sol_alloc,
        "You are allocating to the heap {} times, and it must be less or equal to {} times",
        stu_alloc,
        sol_alloc
    );
}

#[allow(dead_code)]
struct Test<'a> {
    names: Vec<&'a str>,
    result: Vec<&'a str>,
}

#[test]
fn test_function() {
    let cases = vec![
        Test {
            names: vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"],
            result: vec!["H. P.", "S. E.", "J. L.", "B. O."],
        },
        Test {
            names: vec![
                "James John",
                "David Joseph",
                "Matthew Brian",
                "Jacob Sousa",
                "Bruce Banner",
                "Scarlett Johansson",
                "Graydon Hoare",
            ],
            result: vec![
                "J. J.", "D. J.", "M. B.", "J. S.", "B. B.", "S. J.", "G. H.",
            ],
        },
    ];

    for v in cases {
        assert_eq!(
            initials(v.names),
            v.result
                .iter()
                .map(|ele| ele.to_string())
                .collect::<Vec<String>>()
        );
    }
}

#[allow(dead_code)]
fn main() {
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
    // output: ["H. P.", "S. E.", "J. L.", "B. O."]
}
