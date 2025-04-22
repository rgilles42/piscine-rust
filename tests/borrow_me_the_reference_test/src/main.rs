use borrow_me_the_reference::*;

fn main() {
    let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
    let mut b = [
        "2+2".to_owned(),
        "3+2".to_owned(),
        "10-3".to_owned(),
        "5+5".to_owned(),
    ];

    delete_and_backspace(&mut a);
    do_operations(&mut b);

    println!("{:?}", (a, b));
}

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(a_1, "borrow");
        assert_eq!(a_2, "help");
        assert_eq!(a_3, "particular");
        assert_eq!(a_4, "");
    }

    #[test]
    fn test_do_operations() {
        let mut b_1 = [
            "2+2".to_owned(),
            "3+2".to_owned(),
            "10-3".to_owned(),
            "0+0".to_owned(),
            "0-0".to_owned(),
            "10-100".to_owned(),
        ];
        do_operations(&mut b_1);

        assert_eq!(b_1, ["4", "5", "7", "0", "0", "-90"]);
    }
}
