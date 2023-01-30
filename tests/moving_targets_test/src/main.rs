use moving_targets::*;

fn main() {
    let mut field = Field::new();

    println!("{:?}", field.pop());
    field.push(Target { size: 12, xp: 2 });
    println!("{:?}", *field.peek().unwrap());
    field.push(Target { size: 24, xp: 4 });
    println!("{:?}", field.pop());
    let last_target = field.peek_mut().unwrap();
    *last_target = Target { size: 2, xp: 0 };
    println!("{:?}", field.pop());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_pop() {
        let mut field = Field::new();
        field.push(Target { size: 10, xp: 2 });
        field.push(Target { size: 5, xp: 1 });
        field.push(Target { size: 20, xp: 4 });

        assert_eq!(field.pop().unwrap(), Target { size: 20, xp: 4 });
        assert_eq!(field.pop().unwrap(), Target { size: 5, xp: 1 });
        assert_eq!(field.pop().unwrap(), Target { size: 10, xp: 2 });
        assert_eq!(field.pop(), None);
    }

    #[test]
    fn test_peek() {
        let mut field = Field::new();
        field.push(Target { size: 10, xp: 2 });
        field.push(Target { size: 5, xp: 1 });
        field.push(Target { size: 20, xp: 4 });

        assert_eq!(*field.peek().unwrap(), Target { size: 20, xp: 4 });
        assert_eq!(field.pop().unwrap(), Target { size: 20, xp: 4 });
        assert_eq!(field.pop().unwrap(), Target { size: 5, xp: 1 });
        assert_eq!(*field.peek().unwrap(), Target { size: 10, xp: 2 });
        assert_eq!(*field.peek().unwrap(), Target { size: 10, xp: 2 });
    }

    #[test]
    fn test_peek_mut() {
        let mut field = Field::new();
        field.push(Target { size: 10, xp: 2 });
        field.push(Target { size: 5, xp: 1 });
        field.push(Target { size: 20, xp: 4 });

        assert_eq!(*field.peek().unwrap(), Target { size: 20, xp: 4 });
        assert_eq!(field.pop().unwrap(), Target { size: 20, xp: 4 });
        assert_eq!(field.pop().unwrap(), Target { size: 5, xp: 1 });
        assert_eq!(*field.peek_mut().unwrap(), Target { size: 10, xp: 2 });
        let last_target = field.peek_mut();
        last_target.map(|target| *target = Target { size: 50, xp: 44 });
        assert_eq!(*field.peek().unwrap(), Target { size: 50, xp: 44 });
    }
}
