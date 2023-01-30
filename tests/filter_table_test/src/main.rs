use filter_table::Table;

fn main() {
    let mut table = Table::new();
    table.headers = vec![
        "Name".to_string(),
        "Last Name".to_string(),
        "ID Number".to_string(),
    ];
    table.add_row(&[
        "Adam".to_string(),
        "Philips".to_string(),
        "123456789".to_string(),
    ]);
    table.add_row(&[
        "Adamaris".to_string(),
        "Shelby".to_string(),
        "1111123456789".to_string(),
    ]);
    table.add_row(&[
        "Ackerley".to_string(),
        "Philips".to_string(),
        "123456789".to_string(),
    ]);
    let filter_names = |col: &str| col == "Name";
    println!("{:?}", table.filter_col(filter_names));

    let filter_philips = |lastname: &str| lastname == "Philips";
    println!("{:?}", table.filter_row("Last Name", filter_philips));
}

#[cfg(test)]
mod tests {
    use filter_table::*;

    #[test]
    fn filtering_columns() {
        let mut table = Table::new();
        table.headers = vec![
            "name".to_string(),
            "lastname".to_string(),
            "id number".to_string(),
        ];
        table.add_row(&[
            "Ackerley".to_string(),
            "Philips".to_string(),
            "123456789".to_string(),
        ]);
        table.add_row(&[
            "Adamaris".to_string(),
            "Philips".to_string(),
            "1111123456789".to_string(),
        ]);
        table.add_row(&[
            "Ackerley".to_string(),
            "Philips".to_string(),
            "123456789".to_string(),
        ]);

        let filter = |col: &str| col == "name";

        let new_table = Table {
            headers: vec!["name".to_string()],
            body: vec![
                vec!["Ackerley".to_string()],
                vec!["Adamaris".to_string()],
                vec!["Ackerley".to_string()],
            ],
        };
        assert_eq!(new_table, table.filter_col(filter).unwrap());
    }

    #[test]
    fn filtering_rows() {
        let tab = Table {
            headers: vec![
                "Name".to_string(),
                "Last Name".to_string(),
                "ID Number".to_string(),
            ],
            body: vec![
                vec![
                    "Adamaris".to_string(),
                    "Philips".to_string(),
                    "1111123456789".to_string(),
                ],
                vec![
                    "Thomas".to_string(),
                    "Shelby".to_string(),
                    "123456789".to_string(),
                ],
                vec![
                    "Ackerley".to_string(),
                    "Philips".to_string(),
                    "123456789".to_string(),
                ],
            ],
        };

        let get_fillips = |s: &str| s == "Philips";
        // filter the elements with last name Philips
        let expected_table = Table {
            headers: vec![
                "Name".to_string(),
                "Last Name".to_string(),
                "ID Number".to_string(),
            ],
            body: vec![
                vec![
                    "Adamaris".to_string(),
                    "Philips".to_string(),
                    "1111123456789".to_string(),
                ],
                vec![
                    "Ackerley".to_string(),
                    "Philips".to_string(),
                    "123456789".to_string(),
                ],
            ],
        };
        assert_eq!(
            tab.filter_row("Last Name", get_fillips).unwrap(),
            expected_table
        );
    }
}
