use groceries::*;

fn main() {
    let mut groceries = vec![
        "yogurt".to_string(),
        "panettone".to_string(),
        "bread".to_string(),
        "cheese".to_string(),
    ];
    insert(&mut groceries, String::from("nuts"));
    println!("groceries = {:?}", &groceries);
    println!("groceries[1] = {:?}", at_index(&groceries, 1));
}

#[cfg(test)]
mod tests {
    use groceries::{at_index, insert};

    #[test]
    fn test_insertions() {
        let mut groceries = Vec::new();
        insert(&mut groceries, "milk".to_owned());
        assert_eq!(groceries, ["milk"]);
        insert(&mut groceries, "bread".to_owned());
        assert_eq!(groceries, ["milk", "bread"]);
    }

    #[test]
    fn test_index() {
        let groceries = vec![
            "milk".to_owned(),
            "bread".to_owned(),
            "water".to_owned(),
            "wine".to_owned(),
        ];
        assert_eq!(at_index(&groceries, 0), "milk");
        assert_eq!(at_index(&groceries, 2), "water");
        assert_eq!(at_index(&groceries, 3), "wine");
    }

    #[test]
    #[should_panic]
    fn test_oob_index() {
        let groceries = vec!["milk".to_owned()];
        at_index(&groceries, 2);
    }
}
