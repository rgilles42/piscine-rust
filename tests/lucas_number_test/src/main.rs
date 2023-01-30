use lucas_number::lucas_number;

fn main() {
    println!(
        "The element in the position {} in Lucas Numbers is {}",
        2,
        lucas_number(2)
    );
    println!(
        "The element in the position {} in Lucas Numbers is {}",
        5,
        lucas_number(5)
    );
    println!(
        "The element in the position {} in Lucas Numbers is {}",
        10,
        lucas_number(10)
    );
    println!(
        "The element in the position {} in Lucas Numbers is {}",
        13,
        lucas_number(13)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lucas_number_test() {
        assert_eq!(lucas_number(2), 3);
        assert_eq!(lucas_number(5), 11);
        assert_eq!(lucas_number(10), 123);
        assert_eq!(lucas_number(13), 521);
        assert_eq!(lucas_number(25), 167761);
    }
}
