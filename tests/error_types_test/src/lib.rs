use error_types::*;

#[test]
fn test_error_type() {
    let cases = [
        (
            Form {
                name: "Katy".to_owned(),
                password: "qwTw12&%$3sa1dty_".to_owned(),
            },
            Ok(()),
        ),
        (
            Form {
                name: "".to_owned(),
                password: "qwTw12&%$3sa1dty_".to_owned(),
            },
            Err(FormError {
                form_values: ("name", "".to_owned()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Username is empty",
            }),
        ),
        (
            Form {
                name: "Someone".to_owned(),
                password: "12345".to_owned(),
            },
            Err(FormError {
                form_values: ("password", "12345".to_owned()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Password should be at least 8 characters long",
            }),
        ),
        (
            Form {
                name: "Someone".to_owned(),
                password: "sdASDsrW".to_owned(),
            },
            Err(FormError {
                form_values: ("password", "sdASDsrW".to_owned()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Password should be a combination of ASCII numbers, letters and symbols",
            }),
        ),
        (
            Form {
                name: "Someone".to_owned(),
                password: "dsGE1SAD213".to_owned(),
            },
            Err(FormError {
                form_values: ("password", "dsGE1SAD213".to_owned()),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Password should be a combination of ASCII numbers, letters and symbols",
            }),
        ),
        (
            Form {
                name: "Someone".to_owned(),
                password: "dsaSD&%DF!?=".to_owned(),
            },
            Err(FormError {
                form_values: ("password", String::from("dsaSD&%DF!?=")),
                date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                err: "Password should be a combination of ASCII numbers, letters and symbols",
            }),
        ),
    ];

    for (form, expected) in cases {
        assert_eq!(form.validate(), expected);
    }
}
