use macro_map::hash_map;
use std::collections::HashMap;

fn main() {
    let empty: HashMap<u32, u32> = hash_map!();
    let new = hash_map!('a' => 22, 'b' => 1, 'c' => 10);
    let nested = hash_map!(
        "first" => hash_map!(
            "Rob" => 32.2,
            "Gen" => 44.1,
            "Chris" => 10.,
        ),
        "second" => hash_map!()
    );
    println!("{:?}", empty);
    println!("{:?}", new);
    println!("{:?}", nested);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let expected: HashMap<u32, u32> = HashMap::new();
        let computed = hash_map!();

        assert_eq!(computed, expected);
    }

    #[test]
    fn one_element() {
        let mut expected = HashMap::new();
        expected.insert("my name", 10);
        let computed = hash_map!("my name" => 10);
        
        assert_eq!(computed, expected);
    }

    #[test]
    fn multiple_elements_one_line() {
        let mut expected = HashMap::new();
        expected.insert("my name", 10);
        expected.insert("another name", 22);
        let computed = hash_map!("my name" => 10, "another name" => 22);
        
        assert_eq!(computed, expected);
    }

    #[test]
    fn multiple_elements_multiple_lines() {
        let mut expected = HashMap::new();
        expected.insert("my name", 10);
        expected.insert("another name", 22);
        expected.insert("the third one", 33);
        let computed = hash_map!(
            "my name" => 10,
            "another name" => 22,
            "the third one" => 33,
        );
        
        assert_eq!(computed, expected);
    }
}
