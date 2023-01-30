use car_rental::*;
use std::cell::RefCell;

fn main() {
    let car_rental = RentalBusiness {
        car: RefCell::new(Car {
            color: "red".to_string(),
            plate: "AAA".to_string(),
        }),
    };

    println!("{:?}", car_rental.rent_car());
    println!("{:?}", car_rental.repair_car());

    {
        let mut car = car_rental.repair_car();
        car.color = "blue".to_string();
    }

    println!("{:?}", car_rental.rent_car());

    car_rental.change_car(Car {
        color: "pink".to_string(),
        plate: "WWW".to_string(),
    });

    println!("{:?}", car_rental.rent_car());

    println!("{:?}", car_rental.sell_car());
    println!("{:?}", car_rental.sell_car());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rent_car() {
        let car_rental = RentalBusiness {
            car: RefCell::new(Car {
                color: "violet".to_string(),
                plate: "ABCDEF".to_string(),
            }),
        };
        let car_result1 = Car {
            color: "violet".to_string(),
            plate: "ABCDEF".to_string(),
        };

        assert_eq!(*car_rental.rent_car(), car_result1);
        assert_eq!(*car_rental.rent_car(), car_result1);
        assert_eq!(*car_rental.rent_car(), car_result1);
    }

    #[test]
    fn test_repair_car() {
        let car_rental = RentalBusiness {
            car: RefCell::new(Car {
                color: "violet".to_string(),
                plate: "ABCDEF".to_string(),
            }),
        };

        assert_eq!(
            *car_rental.rent_car(),
            Car {
                color: "violet".to_string(),
                plate: "ABCDEF".to_string(),
            }
        );

        {
            let mut car = car_rental.repair_car();
            car.color = "yellow".to_string();
        }

        assert_eq!(
            *car_rental.rent_car(),
            Car {
                color: "yellow".to_string(),
                plate: "ABCDEF".to_string(),
            }
        );
    }

    #[test]
    #[should_panic]
    fn test_repair_car_panic() {
        let car_rental = RentalBusiness {
            car: RefCell::new(Car {
                color: "violet".to_string(),
                plate: "ABCDEF".to_string(),
            }),
        };
        let _car = car_rental.repair_car();
        let _car2 = car_rental.repair_car();
    }

    #[test]
    #[should_panic]
    fn test_repair_and_rent() {
        let car_rental = RentalBusiness {
            car: RefCell::new(Car {
                color: "violet".to_string(),
                plate: "ABCDEF".to_string(),
            }),
        };
        let _car = car_rental.repair_car();
        car_rental.rent_car();
    }

    #[test]
    fn test_change_car() {
        let car_rental = RentalBusiness {
            car: RefCell::new(Car {
                color: "violet".to_string(),
                plate: "ABCDEF".to_string(),
            }),
        };

        car_rental.change_car(Car {
            color: "yellow".to_string(),
            plate: "ABCDEF".to_string(),
        });

        assert_eq!(
            *car_rental.rent_car(),
            Car {
                color: "yellow".to_string(),
                plate: "ABCDEF".to_string(),
            }
        );
    }

    #[test]
    fn test_sell_car() {
        let car_rental = RentalBusiness {
            car: RefCell::new(Car {
                color: "violet".to_string(),
                plate: "ABCDEF".to_string(),
            }),
        };

        assert_eq!(
            car_rental.sell_car(),
            Car {
                color: "violet".to_string(),
                plate: "ABCDEF".to_string(),
            }
        );
        assert_eq!(
            car_rental.sell_car(),
            Car {
                color: "".to_string(),
                plate: "".to_string(),
            }
        );
    }
}
