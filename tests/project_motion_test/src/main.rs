use project_motion::*;

fn main() {
    let mut obj = ThrowObject::new(Object { x: 50.0, y: 50.0 }, Object { x: 0.0, y: 0.0 });
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
    println!("{:?}", obj.next());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_without_acelaration_velocity() {
        let mut obj = ThrowObject::new(Object { x: 50.0, y: 50.0 }, Object { x: 0.0, y: 0.0 });

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 50.0, y: 50.0 },
                init_velocity: Object { x: 0.0, y: 0.0 },
                actual_position: Object { x: 50.0, y: 45.1 },
                actual_velocity: Object { x: 0.0, y: -9.8 },
                time: 1.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 50.0, y: 50.0 },
                init_velocity: Object { x: 0.0, y: 0.0 },
                actual_position: Object { x: 50.0, y: 30.4 },
                actual_velocity: Object { x: 0.0, y: -19.6 },
                time: 2.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 50.0, y: 50.0 },
                init_velocity: Object { x: 0.0, y: 0.0 },
                actual_position: Object { x: 50.0, y: 5.9 },
                actual_velocity: Object { x: 0.0, y: -29.4 },
                time: 3.0,
            })
        );

        assert!(obj.next().is_none(), "{:?} instead of None", obj);

        assert!(obj.next().is_none(), "{:?} instead of None", obj);
    }

    #[test]
    fn test_with_velocity() {
        let mut obj = ThrowObject::new(Object { x: 0.0, y: 50.0 }, Object { x: 10.0, y: 10.0 });

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 50.0 },
                init_velocity: Object { x: 10.0, y: 10.0 },
                actual_position: Object { x: 10.0, y: 55.1 },
                actual_velocity: Object { x: 10.0, y: 0.2 },
                time: 1.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 50.0 },
                init_velocity: Object { x: 10.0, y: 10.0 },
                actual_position: Object { x: 20.0, y: 50.4 },
                actual_velocity: Object { x: 10.0, y: -9.6 },
                time: 2.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 50.0 },
                init_velocity: Object { x: 10.0, y: 10.0 },
                actual_position: Object { x: 30.0, y: 35.9 },
                actual_velocity: Object { x: 10.0, y: -19.4 },
                time: 3.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: 0.0, y: 50.0 },
                init_velocity: Object { x: 10.0, y: 10.0 },
                actual_position: Object { x: 40.0, y: 11.6 },
                actual_velocity: Object { x: 10.0, y: -29.2 },
                time: 4.0,
            })
        );

        assert!(obj.next().is_none(), "{:?} instead of None", obj);
    }

    #[test]
    fn test_with_negative_velocity() {
        let mut obj = ThrowObject::new(Object { x: -10.0, y: 50.0 }, Object { x: -10.0, y: -10.0 });

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: -10.0, y: 50.0 },
                init_velocity: Object { x: -10.0, y: -10.0 },
                actual_position: Object { x: -20.0, y: 35.1 },
                actual_velocity: Object { x: -10.0, y: -19.8 },
                time: 1.0,
            })
        );

        assert_eq!(
            obj.next(),
            Some(ThrowObject {
                init_position: Object { x: -10.0, y: 50.0 },
                init_velocity: Object { x: -10.0, y: -10.0 },
                actual_position: Object { x: -30.0, y: 10.4 },
                actual_velocity: Object { x: -10.0, y: -29.6 },
                time: 2.0,
            })
        );

        assert!(obj.next().is_none(), "{:?} instead of None", obj);
    }

    #[test]
    fn test_with_zero() {
        let mut obj = ThrowObject::new(Object { x: 0.0, y: 0.0 }, Object { x: 0.0, y: 0.0 });

        assert_eq!(obj.next(), None);
    }
}
