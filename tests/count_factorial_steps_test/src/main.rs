use count_factorial_steps::count_factorial_steps;

fn main() {
    println!("The factorial steps of 720 = {}", count_factorial_steps(720));
    println!("The factorial steps of 13 = {}", count_factorial_steps(13));
    println!("The factorial steps of 6 = {}", count_factorial_steps(6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_factorial_steps_edge_cases() {
        assert_eq!(0, count_factorial_steps(0));
        assert_eq!(0, count_factorial_steps(1));
        assert_eq!(0, count_factorial_steps(123));
    }
    #[test]
    fn count_factorial_steps_normal_cases() {
        assert_eq!(6, count_factorial_steps(720));
        assert_eq!(10, count_factorial_steps(3628800));
    }
}
