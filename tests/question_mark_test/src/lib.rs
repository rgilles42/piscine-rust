use question_mark::*;

fn create_nested(value: Option<u16>) -> One {
    One {
        first_layer: Some(Two {
            second_layer: Some(Three {
                third_layer: Some(Four {
                    fourth_layer: value,
                }),
            }),
        }),
    }
}

#[test]
fn test_value() {
    assert_eq!(create_nested(Some(1000)).get_fourth_layer(), Some(1000));
    assert_eq!(create_nested(None).get_fourth_layer(), None);
}
