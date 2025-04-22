use borrow::*;

fn main() {
    let s = "hello";
    let s1 = "camelCase".to_string();
    let s2 = "olá!";

    println!("\tstr_len(\"{}\") = {}", s, str_len(s));
    println!("\tstr_len(\"{}\") = {}", s1, str_len(&s1));
    println!("\tstr_len(\"{}\") = {}", s2, str_len(s2));
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn passes() {
        let inputs = [
            ("hello", 5),
            ("how", 3),
            ("are you", 7),
            ("change", 6),
            ("olá!", 4),
            ("bitteschön", 10),
        ];

        inputs
            .into_iter()
            .for_each(|(s, l)| assert_eq!(l, str_len(s)));
    }
}
