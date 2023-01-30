use generics_list::*;

fn main() {
    let mut new_list_str = List::new();
    new_list_str.push("String Test 1");
    println!("The size of the list is {}", new_list_str.len());

    new_list_str.push("String Test 2");
    println!("The size of the list is {}", new_list_str.len());

    new_list_str.push("String Test 3");
    println!("The size of the list is {}", new_list_str.len());

    new_list_str.pop();
    println!("The size of the list is {}", new_list_str.len());
}

#[test]
fn new_list_test() {
    let mut new_list_str = List::new();
    new_list_str.push("String Test");

    let mut new_list_num = List::new();
    new_list_num.push(5);

    assert_eq!(new_list_str.head.unwrap().value, "String Test");
    assert_eq!(new_list_num.head.unwrap().value, 5);
}

#[test]
fn big_list_test() {
    let mut new_list_nbr = List::new();

    for i in 0..10 {
        new_list_nbr.push(i);
    }

    let mut aux = new_list_nbr.head.unwrap();
    for i in (1..10).collect::<Vec<i32>>().iter().rev() {
        assert_eq!(aux.value, *i);
        aux = *aux.next.unwrap();
    }
}

#[test]
fn remove_test() {
    let mut new_list_nbr = List::new();

    for i in 0..10 {
        new_list_nbr.push(i);
    }

    assert_eq!(new_list_nbr.len(), 10);

    for _ in 0..5 {
        new_list_nbr.pop();
    }

    assert_eq!(new_list_nbr.len(), 5);

    let mut aux = new_list_nbr.clone().head.unwrap();
    for i in (1..5).collect::<Vec<i32>>().iter().rev() {
        assert_eq!(aux.value, *i);
        aux = *aux.next.unwrap();
    }

    for _ in 0..5 {
        assert_eq!(new_list_nbr.head.as_ref().is_none(), false);
        new_list_nbr.pop();
    }

    assert_eq!(new_list_nbr.head.as_ref().is_none(), true);
}

#[test]
fn len_test() {
    let mut new_list_nbr = List::new();

    assert_eq!(new_list_nbr.len(), 0);

    for i in 0..10 {
        new_list_nbr.push(i);
        assert_eq!(new_list_nbr.len(), i + 1);
    }
}
