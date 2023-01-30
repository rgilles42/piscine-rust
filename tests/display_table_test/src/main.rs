use display_table::Table;

fn main() {
    let mut table = Table::new();
    table.headers = vec![
        "ID".to_string(),
        "Car Brand".to_string(),
        "Model".to_string(),
        "Is Electric".to_string(),
    ];
    table.add_row(&[
        "1".to_string(),
        "Tesla".to_string(),
        "Model 3".to_string(),
        "True".to_string(),
    ]);
    table.add_row(&[
        "2".to_string(),
        "Ford".to_string(),
        "Focus".to_string(),
        "False".to_string(),
    ]);
    println!("{}", table);
}

#[cfg(test)]
mod tests {
    use display_table::*;

    #[test]
    fn it_displays() {
        let mut table = Table::new();
        table.headers = vec![
            "Name".to_string(),
            "Last Name".to_string(),
            "ID Number".to_string(),
        ];
        table.add_row(&[
            "Ackerley".to_string(),
            "Fillips".to_string(),
            "123456789".to_string(),
        ]);
        table.add_row(&[
            "Adamaris".to_string(),
            "Fillips".to_string(),
            "1111123456789".to_string(),
        ]);
        table.add_row(&[
            "Ackerley".to_string(),
            "Fillips".to_string(),
            "123456789".to_string(),
        ]);
        assert_eq!(
			table.to_string(),
			"|   Name   | Last Name |   ID Number   |\n|----------+-----------+---------------|\n| Ackerley |  Fillips  |   123456789   |\n| Adamaris |  Fillips  | 1111123456789 |\n| Ackerley |  Fillips  |   123456789   |\n"
		);
    }

    // An empty table must not display anything
    #[test]
    fn display_table_with_no_headers() {
        let table = Table::new();
        assert_eq!(table.to_string(), "");
    }

    #[test]
    fn display_table_with_headers_only() {
        let mut table = Table::new();
        table.headers = vec![
            "Name".to_string(),
            "Last Name".to_string(),
            "ID Number".to_string(),
        ];
        assert_eq!(
            table.to_string(),
            "| Name | Last Name | ID Number |\n|------+-----------+-----------|\n"
        );
    }

    #[test]
    fn display_second() {
        let mut table = Table::new();
        table.headers = vec![
            "ID".to_string(),
            "Car Brand".to_string(),
            "Model".to_string(),
            "Is Electric".to_string(),
        ];
        table.add_row(&[
            "1".to_string(),
            "Tesla".to_string(),
            "Model 3".to_string(),
            "True".to_string(),
        ]);
        table.add_row(&[
            "2".to_string(),
            "Ford".to_string(),
            "Focus".to_string(),
            "False".to_string(),
        ]);
        assert_eq!(
			table.to_string(),
			"| ID | Car Brand |  Model  | Is Electric |\n|----+-----------+---------+-------------|\n| 1  |   Tesla   | Model 3 |    True     |\n| 2  |   Ford    |  Focus  |    False    |\n"
		);
    }
}
