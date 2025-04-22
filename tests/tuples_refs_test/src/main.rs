use tuples_refs::*;

fn main() {
    let student = Student(20, "Pedro".to_string(), "Domingos".to_string());
    println!("Student's first name: {}", first_name(&student));
    println!("Student's last name: {}", last_name(&student));
    println!("Student's id: {}", id(&student));
}

#[cfg(test)]
mod tests {
    use tuples_refs::*;

    #[test]
    fn test_id() {
        let student = Student(20, String::from("Pedro"), String::from("Domingos"));
        assert_eq!(id(&student), 20);
    }

    #[test]
    fn test_first_name() {
        let student = Student(20, String::from("Pedro"), String::from("Domingos"));
        assert_eq!(first_name(&student), "Pedro");
    }

    #[test]
    fn test_last_name() {
        let student = Student(20, String::from("Pedro"), String::from("Domingos"));
        assert_eq!(last_name(&student), "Domingos");
    }
}
