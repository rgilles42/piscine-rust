use borrow_me_the_reference::*;

fn main() {}

#[test]
fn test_delete_and_backspace() {
    let mut a_1 = String::from("bpp--o+er+++sskroi-++lcw");
    let mut a_2 = String::from("hs-+deasdasd------l+++dsdp");
    let mut a_3 = String::from("pad-rtic+eulqw--+rar");
    let mut a_4 = String::from("--++++");

    delete_and_backspace(&mut a_1);
    delete_and_backspace(&mut a_2);
    delete_and_backspace(&mut a_3);
    delete_and_backspace(&mut a_4);

    assert_eq!(a_1, "borrow".to_string());
    assert_eq!(a_2, "help".to_string());
    assert_eq!(a_3, "particular".to_string());
    assert_eq!(a_4, "".to_string());
}
#[test]
fn test_do_operations() {
    let mut b_1: Vec<String> = vec![
        "2+2".to_string(),
        "3+2".to_string(),
        "10-3".to_string(),
        "0+0".to_string(),
        "0-0".to_string(),
        "10-100".to_string(),
    ];
    do_operations(&mut b_1);

    assert_eq!(b_1, vec!["4", "5", "7", "0", "0", "-90"]);
}
