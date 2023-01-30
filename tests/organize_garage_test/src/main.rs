use organize_garage::*;

fn main() {
    let mut garage = Garage {
        left: Some(5),
        right: Some(2),
    };

    println!("{:?}", garage);
    garage.move_to_right();
    println!("{:?}", garage);
    garage.move_to_left();
    println!("{:?}", garage);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_to_right() {
        let mut garage_int = Garage {
            left: Some(5),
            right: Some(2),
        };

        garage_int.move_to_right();
        assert_eq!(
            garage_int,
            Garage {
                left: None,
                right: Some(7)
            }
        );
        garage_int.move_to_right();
        assert_eq!(
            garage_int,
            Garage {
                left: None,
                right: Some(7)
            }
        );

        let mut garage_float = Garage {
            left: Some(4.25),
            right: Some(1.11),
        };

        garage_float.move_to_right();
        assert_eq!(
            garage_float,
            Garage {
                left: None,
                right: Some(5.36)
            }
        );
    }

    #[test]
    fn test_move_to_left() {
        let mut garage_int = Garage {
            left: Some(10),
            right: Some(2),
        };

        garage_int.move_to_left();
        assert_eq!(
            garage_int,
            Garage {
                left: Some(12),
                right: None
            }
        );
        garage_int.move_to_left();
        assert_eq!(
            garage_int,
            Garage {
                left: Some(12),
                right: None
            }
        );

        let mut garage_float = Garage {
            left: Some(4.25),
            right: Some(1.11),
        };

        garage_float.move_to_left();
        assert_eq!(
            garage_float,
            Garage {
                left: Some(5.36),
                right: None
            }
        );
    }
}
