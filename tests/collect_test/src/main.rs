//! IMPORTANT
// reviewer remind me of this! `collect` is a non-sensical name. I suggest changing it to `bubblesort` or something similar. How would we go on about this?

use collect::*;

fn main() {
    let mut v = [3, 2, 4, 5, 1, 7];
    let mut v_clone = v;

    bubble_sort(&mut v);
    println!("{:?}", v);

    v_clone.sort_unstable();
    println!("{:?}", v_clone);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ordering() {
        let mut v = [3, 2, 4, 5, 1, 7, 9, 8];
        let mut v_clone = v;

        v_clone.sort_unstable();
        bubble_sort(&mut v);

        assert_eq!(v, v_clone);
    }
}
