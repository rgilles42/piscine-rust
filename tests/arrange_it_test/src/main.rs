use std::alloc::{self, GlobalAlloc, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

use arrange_it::*;

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
    println!("{}", arrange_phrase("is2 Thi1s T4est 3a"));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Deliberately suboptimal solution to give the student some room for heap allocation
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

    #[test]
    fn test_memory_allocation() {
        let test_value = "w7ork t3he a4rt o5f Per1formance is2 a6voiding";

        let test = |f: &dyn Fn(&str) -> String| {
            ALLOCATOR.reset_counter();
            f(test_value);
            ALLOCATOR.counter()
        };

        let sol_alloc = test(&arrange_phrase_sol);
        let stu_alloc = test(&arrange_phrase);

        assert!(stu_alloc <= sol_alloc);
    }

    #[test]
    fn test_function() {
        let cases = [
            (
                "4of Fo1r pe6ople g3ood th5e the2",
                "For the good of the people",
            ),
            ("is2 Thi1s T4est 3a", "This is a Test"),
            (
                "w7ork t3he a4rt o5f Per1formance is2 a6voiding",
                "Performance is the art of avoiding work",
            ),
        ];

        cases
            .into_iter()
            .for_each(|(n, e)| assert_eq!(arrange_phrase(n), e));
    }
}
