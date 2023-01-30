/*
## lastup

### Instructions

Complete the `lastup` function that takes a string and puts the last letter of each word in uppercase and the rest in lowercase.

### Expected Functions

```rust
pub fn lastup(input: &str) -> String {
}
```

### Usage

Here is a program to test your function.

```rust
use lastup::lastup;

fn main() {
    println!("{}", lastup("joe is missing"));
    println!("{}", lastup("jill is leaving A"));
    println!("{}",lastup("heLLo THere"));
}
```

And its output

```console
student@ubuntu:~/[[ROOT]]/test$ cargo run
joE iS missinG
jilL iS leavinG A
hellO therE
student@ubuntu:~/[[ROOT]]/test$
```
 */

use lastup::*;

fn main() {
    println!("{}", lastup("joe is missing"));
    println!("{}", lastup("jill is leaving A"));
    println!("{}", lastup("heLLo THere!"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lastup() {
        assert_eq!(lastup("hello"), "hellO");
        assert_eq!(lastup("this is working"), "thiS iS workinG");
        assert_eq!(lastup("HOW ARE YOU TODAY?"), "hoW arE yoU todaY?");
        assert_eq!(lastup("tHIs IS wOrking 10"), "thiS iS workinG 10");
    }

    #[test]
    fn test_empty() {
        assert_eq!(lastup(""), "");
    }
}
