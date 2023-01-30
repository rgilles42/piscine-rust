// Write a function that receives a string slice and returns the
// length of character of the string

use strings::*;

fn main() {
    println!("lenght of {} = {}", "❤", "❤".len());
    println!("lenght of {} = {}", "❤", char_length("❤"));
    println!("lenght of {} = {}", "形声字", char_length("形聲字"));
    println!("lenght of {} = {}", "形声字", "形聲字".len());
    println!("lenght of {} = {}", "change", "change".len());
    println!("lenght of {} = {}", "change", char_length("change"));
    println!("char lenght of {} = {}", "😍", char_length("😍"));
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
