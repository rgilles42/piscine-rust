use format_me::*;

fn main() {
    println!(
        "{}",
        Park {
            name: "Les Tuileries".to_string(),
            park_type: ParkType::Garden,
            address: "Pl. de la Concorde".to_string(),
            cap: "75001".to_string(),
            state: "France".to_string()
        }
    );
    println!(
        "{}",
        Park {
            name: "".to_string(),
            park_type: ParkType::Playground,
            address: "".to_string(),
            cap: "".to_string(),
            state: "".to_string()
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_park() {
        let park = Park {
            name: "Central Park".to_string(),
            park_type: ParkType::Garden,
            address: "Av. Sidónio Pais 4".to_string(),
            cap: "1050-214".to_string(),
            state: "Portugal".to_string(),
        };

        assert_eq!(
            park.to_string(),
            "garden - Central Park, Av. Sidónio Pais 4, 1050-214 - Portugal"
        );
    }

    #[test]
    fn test_empty_name() {
        let park = Park {
            name: "".to_string(),
            park_type: ParkType::Forest,
            address: "Av. Sidónio Pais 4".to_string(),
            cap: "1050-214".to_string(),
            state: "Portugal".to_string(),
        };

        assert_eq!(
            park.to_string(),
            "forest - No name, Av. Sidónio Pais 4, 1050-214 - Portugal"
        );
    }

    #[test]
    fn test_empty_all() {
        let park = Park {
            name: "".to_string(),
            park_type: ParkType::Playground,
            address: "".to_string(),
            cap: "".to_string(),
            state: "".to_string(),
        };

        assert_eq!(
            park.to_string(),
            "playground - No name, No address, No cap - No state"
        );
    }
}
