use profanity_filter::*;

fn main() {
    let m0 = Message::new("hello there".to_string(), "toby".to_string());
    println!("{:?}", check_ms(&m0));
    // output: (true, "hello there")

    let m1 = Message::new("".to_string(), "toby".to_string());
    println!("{:?}", check_ms(&m1));
    // output: (false, "ERROR: illegal")

    let m2 = Message::new("you are stupid".to_string(), "toby".to_string());
    println!("{:?}", check_ms(&m2));
    // output: (false, "ERROR: illegal")

    let m3 = Message::new("stupid".to_string(), "toby".to_string());
    println!("{:?}", check_ms(&m3));
    // output: (false, "ERROR: illegal")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_ms() {
        let v = vec![
            Message::new("".to_string(), "toby".to_string()),
            Message::new("stupid".to_string(), "jack".to_string()),
            Message::new("you are stupid".to_string(), "jacob".to_string()),
        ];
        for value in v {
            let (t, _) = check_ms(&value);
            assert!(!t);
        }
    }

    #[test]
    fn test_ok_ms() {
        let v = vec![
            Message::new("get out of the car".to_string(), "police".to_string()),
            Message::new("no!".to_string(), "thief".to_string()),
            Message::new("get the werewolf".to_string(), "police".to_string()),
            Message::new("wait the wha...".to_string(), "thief".to_string()),
        ];
        for value in v {
            let (t, _) = check_ms(&value);
            assert!(t);
        }
    }
}
