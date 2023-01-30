// Write three methods for BloodType:
// - can_receive_from(&self, other: BloodType) -> bool {}: which
// returns true if self can receive blood from `other` blood type
// - donors(&self) -> Vec<BloodType>: which returns
// all the blood types that can give blood to self
// - recipients(&self) -> Vec<BloodType>: which returns all the blood
// types that can receive blood from self

use blood_types_s::{Antigen, BloodType, RhFactor};

fn main() {
    let blood_type = BloodType {
        antigen: Antigen::O,
        rh_factor: RhFactor::Positive,
    };
    println!("recipients of O+ {:?}", blood_type.recipients());
    println!("donors of O+ {:?}", blood_type.donors());
    let another_blood_type = BloodType {
        antigen: Antigen::O,
        rh_factor: RhFactor::Positive,
    };
    println!(
        "donors of O+ can receive from {:?} {:?}",
        &another_blood_type,
        blood_type.can_receive_from(&another_blood_type)
    );
}

#[cfg(test)]
mod tests {
    use blood_types_s::*;

    #[test]
    fn compatible_ab_neg_with_a_pos() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        };
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_a_pos() {
        let blood_type = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        };
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_a_neg_with_ab_neg() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_neg_with_o_pos() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Positive,
        };
        assert!(!blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn compatible_ab_pos_with_o_pos() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        let other_bt = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Positive,
        };
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_compatible_ab_neg_with_o_neg() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        };
        let other_bt = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        };
        assert!(blood_type.can_receive_from(&other_bt));
    }

    #[test]
    fn test_antigen_ab_from_str() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        assert_eq!(blood_type.antigen, Antigen::AB);
        assert_eq!(blood_type.rh_factor, RhFactor::Positive);
    }

    #[test]
    fn test_antigen_a_from_str() {
        let blood_type = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        assert_eq!(blood_type.antigen, Antigen::A);
        assert_eq!(blood_type.rh_factor, RhFactor::Negative);
    }

    #[test]
    fn test_donors() {
        let blood_type = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        let mut givers = blood_type.donors();
        // println!("Before sorting {:?}", &givers);
        givers.sort();
        // println!("{:?}", &givers);
        let mut expected = vec![
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::B,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            },
        ];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_a_neg_donors() {
        let blood = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };
        let mut givers = blood.donors();
        givers.sort();
        let mut expected = vec![
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
        ];

        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_o_neg_donors() {
        let blood = BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        };

        let mut givers = blood.donors();
        givers.sort();
        let mut expected = vec![blood.clone()];
        expected.sort();
        assert_eq!(givers, expected);
    }

    #[test]
    fn test_ab_pos_recipients() {
        let blood = BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        };
        let mut recipients = blood.recipients();
        recipients.sort();
        let mut expected = vec![blood.clone()];
        expected.sort();
        assert_eq!(recipients, expected);
    }

    #[test]
    fn test_a_neg_recipients() {
        let blood = BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        };

        let mut recipients = blood.recipients();
        recipients.sort();
        let mut expected = vec![
            blood.clone(),
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
        ];
        expected.sort();
        assert_eq!(recipients, expected);
    }
}
