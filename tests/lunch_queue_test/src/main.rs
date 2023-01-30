use lunch_queue::*;

// Example :
fn main() {
    let mut list = Queue::new();
    list.add(String::from("Marie"), 20);
    list.add(String::from("Monica"), 15);
    list.add(String::from("Ana"), 5);
    list.add(String::from("Alice"), 35);
    println!("{:?}", list);

    // output:
    // Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: Some(Person { name: "Marie", discount: 20, next_person: None }) }) }) }) }
    println!("{:?}", list.search("Marie"));
    println!("{:?}", list.search("Alice"));
    println!("{:?}", list.search("someone"));
    // output:
    // Some(("Marie", 20))
    // Some(("Alice", 35))
    // None

    println!("removed {:?}", list.rm());
    println!("removed {:?}", list.rm());
    println!("list {:?}", list);
    list.invert_queue();
    println!("inverted list {:?}", list);
    // output:
    // removed Some(("Marie", 20))
    // list Queue { node: Some(Person { name: "Alice", discount: 35, next_person: Some(Person { name: "Ana", discount: 5, next_person: Some(Person { name: "Monica", discount: 15, next_person: None }) }) }) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let list = Queue::new();
        assert!(list.node.is_none());
    }

    #[test]
    fn test_one_person() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 14);
        list.rm();
        assert!(list.node.is_none());
    }

    #[test]
    fn test_two_person() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 13);
        list.add(String::from("Monica"), 54);
        list.rm();

        assert_eq!(list.node.as_ref().unwrap().name, "Monica");
        assert_eq!(list.node.as_ref().unwrap().discount, 54);
    }

    #[test]
    fn test_more_person() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 20);
        list.add(String::from("Monica"), 15);
        list.add(String::from("Ana"), 5);
        list.add(String::from("Alice"), 35);
        list.rm();

        assert_eq!(list.node.as_ref().unwrap().name, "Alice");
        assert_eq!(list.node.as_ref().unwrap().discount, 35);

        list.rm();
        list.rm();
        assert_eq!(list.node.as_ref().unwrap().name, "Alice");
        assert_eq!(list.node.as_ref().unwrap().discount, 35);
    }

    #[test]
    fn test_search() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 20);
        list.add(String::from("Monica"), 15);
        list.add(String::from("Ana"), 5);
        list.add(String::from("Alice"), 35);

        assert_eq!(list.search("Ana"), Some((String::from("Ana"), 5)));

        assert_eq!(list.search("Monica"), Some((String::from("Monica"), 15)));

        assert_eq!(list.search("Alice"), Some((String::from("Alice"), 35)));

        assert_eq!(list.search("someone_that_does_not_exist"), None);
    }

    #[test]
    fn test_invert() {
        let mut list = Queue::new();
        list.add(String::from("Marie"), 20);
        list.add(String::from("Monica"), 15);
        list.add(String::from("Ana"), 5);
        list.add(String::from("Alice"), 35);

        list.invert_queue();
        assert_eq!(list.node.as_ref().unwrap().name, "Marie");
        assert_eq!(list.node.as_ref().unwrap().discount, 20);

        list.rm();
        list.invert_queue();
        assert_eq!(list.node.as_ref().unwrap().name, "Ana");
        assert_eq!(list.node.as_ref().unwrap().discount, 5);
    }
}
