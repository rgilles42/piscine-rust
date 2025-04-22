use hashing::*;

fn main() {
    let v = [4, 7, 5, 2, 5, 1, 3];

    println!("mean {}", mean(&v));
    println!("median {}", median(&v));
    println!("mode {}", mode(&v));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;

    #[inline]
    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < f64::EPSILON
    }

    #[test]
    fn test_mean() {
        let v = [4, 7, 5, 2, 5, 1, 3];
        assert!(approx_eq(mean(&v), 3.857142857142857));
    }

    #[test]
    fn test_median() {
        assert_eq!(median(&[4, 7, 5, 2, 5, 1, 3]), 4);
        assert_eq!(median(&[2, 1, 5, 2, 7, 4]), 3);
        assert_eq!(median(&[1, 7, 5, 5, 6, 4]), 5);
    }

    #[test]
    fn test_mode() {
        let v = [4, 7, 5, 2, 5, 1, 3];
        assert_eq!(mode(&v), 5);
    }
}
