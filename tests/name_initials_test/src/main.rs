use std::alloc::{self, GlobalAlloc, Layout};
use std::sync::atomic::{AtomicUsize, Ordering};

use name_initials::*;

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
    let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
    println!("{:?}", initials(names));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Deliberately suboptimal solution to give the student some room for heap allocation
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
        let test_value = ["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];

        let test = |f: &dyn Fn(Vec<&str>) -> Vec<String>| {
            let test_value = test_value.to_vec();
            ALLOCATOR.reset_counter();
            f(test_value);
            ALLOCATOR.counter()
        };

        let sol_alloc = test(&initials_sol);
        let stu_alloc = test(&initials);

        assert!(stu_alloc <= sol_alloc);
    }

    #[test]
    fn test_function() {
        let cases = [
            (
                vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"],
                vec!["H. P.", "S. E.", "J. L.", "B. O."],
            ),
            (
                vec![
                    "James John",
                    "David Joseph",
                    "Matthew Brian",
                    "Jacob Sousa",
                    "Bruce Banner",
                    "Scarlett Johansson",
                    "Graydon Hoare",
                ],
                vec![
                    "J. J.", "D. J.", "M. B.", "J. S.", "B. B.", "S. J.", "G. H.",
                ],
            ),
        ];

        cases
            .into_iter()
            .for_each(|(n, e)| assert_eq!(initials(n), e));
    }
}
