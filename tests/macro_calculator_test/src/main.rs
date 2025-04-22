use macro_calculator::*;

#[test]
fn testing_macros_values() {
    let a = Food {
        name: "light milk".to_owned(),
        calories: ("148kJ".to_owned(), "35kcal".to_owned()),
        proteins: 3.5,
        fats: 0.1,
        carbs: 5.0,
        nbr_of_portions: 0.7,
    };
    let b = Food {
        name: "oat cookies".to_owned(),
        calories: ("1996kJ".to_owned(), "477kcal".to_owned()),
        proteins: 8.2,
        fats: 21.0,
        carbs: 60.4,
        nbr_of_portions: 1.2,
    };

    let macros = calculate_macros(&[a, b]);

    assert_eq!(macros["cals"].as_f64().unwrap(), 596.9);
    assert_eq!(macros["carbs"].as_f64().unwrap(), 75.98);
    assert_eq!(macros["proteins"].as_f64().unwrap(), 12.29);
    assert_eq!(macros["fats"].as_f64().unwrap(), 25.27);
}

#[test]
fn testing_no_food() {
    let macros = calculate_macros(&[]);

    assert_eq!(macros["cals"].as_f64().unwrap(), 0.0);
    assert_eq!(macros["carbs"].as_f64().unwrap(), 0.0);
    assert_eq!(macros["proteins"].as_f64().unwrap(), 0.0);
    assert_eq!(macros["fats"].as_f64().unwrap(), 0.0);
}

#[test]
fn big_values() {
    let macros = calculate_macros(&[
        Food {
            name: "big mac".to_owned(),
            calories: ("2133.84kJ".to_owned(), "510kcal".to_owned()),
            proteins: 27.0,
            fats: 26.0,
            carbs: 41.0,
            nbr_of_portions: 2.0,
        },
        Food {
            name: "pizza margherita".to_owned(),
            calories: ("1500.59kJ".to_owned(), "358.65kcal".to_owned()),
            proteins: 13.89,
            fats: 11.21,
            carbs: 49.07,
            nbr_of_portions: 4.9,
        },
    ]);

    assert_eq!(macros["cals"].as_f64().unwrap(), 2777.39);
    assert_eq!(macros["carbs"].as_f64().unwrap(), 322.44);
    assert_eq!(macros["proteins"].as_f64().unwrap(), 122.06);
    assert_eq!(macros["fats"].as_f64().unwrap(), 106.93);
}
