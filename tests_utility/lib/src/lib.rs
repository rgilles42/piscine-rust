use std::fmt::{self, Debug, Display};

// TODO: TestProperties to a lib
#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum Kind {
    Method,   // Makes the message firstInput.MethodName(inputs[1], input[2], ..])
    Operator, // Makes the message inputs[0] OperatorName inputs[1] ex: 1 + 2
    Function, // Makes the message FunctionName(inputs[0], inputs[1], inputs[2], ..)
    Value,
}

pub struct Inputs<'a>(pub &'a [Input]);

impl<'a> Display for Inputs<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in self.0.iter().take(self.0.len() - 1) {
            write!(f, "{:?}, ", item)?;
        }
        write!(f, "{:?}", self.0[self.0.len() - 1])
    }
}

pub type Input = Box<dyn Debug>;

#[derive(Debug)]
pub struct TestProperties {
    pub name: &'static str,
    pub kind: Kind,
}

impl TestProperties {
    pub fn assert_with_message<U: std::fmt::Debug + std::cmp::PartialEq>(
        &self,
        inputs: &[Input],
        actual: U,
        expected: U,
    ) {
        let message_name = match (inputs.len(), self.kind) {
            (0, Kind::Function) => format!("{}()", self.name),
            (0, Kind::Value) => format!("{}", self.name),
            (0, _) => String::new(),
            (1, Kind::Method) => format!("{:?}.{}()", inputs[0], self.name),
            (1, Kind::Function) => format!("{}({:?})", self.name, inputs[0]),
            (1, Kind::Operator) => format!("{} {:?}", self.name, inputs[0]),
            (_, Kind::Function) => format!("{}({:?})", self.name, inputs),
            (_, Kind::Operator) => {
                format!("{:?} {} {}", inputs[0], self.name, Inputs(&inputs[1..]))
            }
            (_, Kind::Method) => {
                format!("{:?}.{}({})", inputs[0], self.name, Inputs(&inputs[1..]))
            }
            (_, Kind::Value) => {
                format!("{}.{}", Inputs(&inputs), self.name)
            }
        };
        assert_eq!(
            actual, expected,
            "\n\t`{}` == {:?}, expected == {:?}",
            message_name, actual, expected
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one() -> i32 {
        1
    }

    const TEST: TestProperties = TestProperties {
        kind: Kind::Function,
        name: "one",
    };

    #[test]
    fn test_a_function_with_zero_inputs() {
        TEST.assert_with_message(&[], one(), 1);
    }

    #[test]
    #[should_panic]
    fn test_with_wrong_result() {
        TEST.assert_with_message(&[], one(), 2);
    }

    #[test]
    fn test_operators() {
        let test = TestProperties {
            kind: Kind::Operator,
            name: "-",
        };
        test.assert_with_message(&[Box::new(2), Box::new(1)], 2 - 1, 1);
    }

    #[test]
    #[should_panic]
    fn test_unary_operators() {
        let test = TestProperties {
            kind: Kind::Operator,
            name: "-",
        };
        test.assert_with_message(&[Box::new(1)], -1, -2);
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct MockStruct {
        a: i32,
        b: &'static str,
        c: f64,
    }

    impl MockStruct {
        fn set_vals(&mut self, a: i32, b: &'static str, c: f64) {
            self.a = a;
            self.b = b;
            self.c = c;
        }
    }

    #[test]
    fn method_with_more_than_2_args() {
        let test = TestProperties {
            kind: Kind::Method,
            name: "set_vals",
        };
        let mut mock = MockStruct {
            a: 1,
            b: "str",
            c: 1.4,
        };
        mock.set_vals(2, "string", 3.23);
        test.assert_with_message(
            &[Box::new(2), Box::new("string"), Box::new(3.23)],
            mock,
            MockStruct {
                a: 2,
                b: "string",
                c: 3.23,
            },
        );
    }
}
