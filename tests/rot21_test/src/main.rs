/*
## rot21

### Instructions

Your purpose in this exercise is to create a `rot21` function that works like the ROT13 cipher.

Your function will receive a string and it will rotate each letter of that string 21 times to the right.

Your function should only change letters. If the string includes punctuation, symbols and numbers
they will remain the same.

### Expected functions

```rust
pub fn rot21(input: &str) -> String {}
```

### Usage

Here is a program to test your function.

```rust
use rot21::rot21;

fn main() {
    println!("The letter \"a\" becomes: {}", rot21("a"));
    println!("The letter \"m\" becomes: {}", rot21("m"));
    println!("The word \"MISS\" becomes: {}", rot21("MISS"));
    println!("Your cypher wil be: {}", rot21("Testing numbers 1 2 3"));
    println!("Your cypher wil be: {}", rot21("rot21 works!"));
}

```

And its output

```console
The letter "a" becomes: v
The letter "m" becomes: h
The word "MISS" becomes: HDNN
Your cypher wil be: Oznodib iphwzmn 1 2 3
Your cypher wil be: mjo21 rjmfn!
student@ubuntu:~/[[ROOT]]/test$
```
*/
use rot21::*;

fn main() {
    println!("The letter \"a\" becomes: {}", rot21("a"));
    println!("The letter \"m\" becomes: {}", rot21("m"));
    println!("The word \"MISS\" becomes: {}", rot21("MISS"));
    println!("Your cypher wil be: {}", rot21("Testing numbers 1 2 3"));
    println!("Your cypher wil be: {}", rot21("rot21 works!"));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rot21_multiple_cases() {
        assert_eq!("ocdn dn v ozno", rot21("this is a test"));
        assert_eq!("mviyjh ndhkgz rjmyn", rot21("random simple words"));
        assert_eq!(
            "ojj  hpxc    nkvxzn      rjmfn",
            rot21("too  much    spaces      works")
        );
        assert_eq!("mv😋w", rot21("ra😋b"));
        assert_eq!("12Â nãj ábpv", rot21("12Â são água"));

        assert_eq!("VWXY", rot21("ABCD"));
        assert_eq!("GJJFDIB BJJY", rot21("LOOKING GOOD"));
        assert_eq!("WTZ", rot21("BYE"));
    }
}
