// Write a function that receives a string slice and returns the
// length of character of the string

use strings::*;

fn main() {
    println!("length of {} = {}", "❤", "❤".len());
    println!("length of {} = {}", "❤", char_length("❤"));
    println!("length of {} = {}", "形声字", char_length("形聲字"));
    println!("length of {} = {}", "形声字", "形聲字".len());
    println!("length of {} = {}", "change", "change".len());
    println!("length of {} = {}", "change", char_length("change"));
    println!("char length of {} = {}", "😍", char_length("😍"));
}

// fn char_length(s: &str) -> usize {
// 	let mut chars = 0;
// 	for _ in s.chars() {
// 		chars += 1;
// 	}
// 	chars
// }

#[test]
fn test_ascii() {
    let s = "ascii";
    assert_eq!(char_length(s), 5);
}

#[test]
fn test_emoji() {
    let s = "❤😍";
    assert_eq!(char_length(s), 2);
}

#[test]
fn test_chinese_char() {
    let s = "形声字";
    assert_eq!(char_length(s), 3);
}
