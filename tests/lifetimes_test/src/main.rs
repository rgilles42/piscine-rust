// Create a struct called Person that has two fields: name of type
// string slice (&str) and age of type u8
// and create the associated function new which creates a new person
// with age 0 and with the name given

use lifetimes::Person;

fn main() {
    let person = Person::new("Leo");

    println!("Person = {:?}", person);
}

#[cfg(test)]
mod tests {
    use super::*;
    use lib::{Kind, TestProperties};

    #[test]
    fn fields() {
        let person = Person {
            name: "Dijkstra",
            age: 10,
        };
        let test = TestProperties {
            kind: Kind::Value,
            name: "age",
        };
        test.assert_with_message(&[Box::new(person.clone())], person.age, 10);
        let test = TestProperties {
            kind: Kind::Value,
            name: "name",
        };
        test.assert_with_message(&[Box::new(person.clone())], person.name, "Dijkstra");
    }

    #[test]
    fn create_person() {
        let person = Person::new("Leo");
        let test = TestProperties {
            kind: Kind::Value,
            name: "age",
        };
        test.assert_with_message(&[Box::new(person.clone())], person.age, 0);
        let test = TestProperties {
            kind: Kind::Value,
            name: "name",
        };
        test.assert_with_message(&[Box::new(person.clone())], person.name, "Leo");
    }
}
