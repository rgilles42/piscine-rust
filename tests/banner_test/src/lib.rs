use banner::*;
use std::{collections::HashMap, sync::LazyLock};

static HANDLER: LazyLock<FlagsHandler> = LazyLock::new(|| {
    let mut handler = FlagsHandler {
        flags: HashMap::new(),
    };

    handler.add_flag(Flag::opt_flag("division", "divides two numbers"), div);
    handler.add_flag(
        Flag::opt_flag(
            "remainder",
            "gives the remainder of the division between two numbers",
        ),
        rem,
    );

    handler
});

#[test]
fn test_simple() {
    for a in ["-d", "--division"] {
        assert_eq!(HANDLER.exec_func(a, &["1.0", "2.0"]), Ok("0.5".to_owned()));
    }

    for a in ["-r", "--remainder"] {
        assert_eq!(HANDLER.exec_func(a, &["2.0", "2.0"]), Ok("0".to_owned()));
    }

    for a in ["-d", "--division"] {
        assert_eq!(
            HANDLER.exec_func(a, &["12.323", "212.32"]),
            Ok("0.058039751318764134".to_owned())
        );
    }

    for a in ["-r", "--remainder"] {
        assert_eq!(
            HANDLER.exec_func(a, &["12.323", "212.32"]),
            Ok("12.323".to_owned())
        );
    }
}

#[test]
fn test_edge_cases() {
    for a in ["-d", "--division"] {
        assert_eq!(
            HANDLER.exec_func(a, &["a", "2.0"]),
            Err("invalid float literal".to_owned())
        );
    }

    for a in ["-r", "--remainder"] {
        assert_eq!(
            HANDLER.exec_func(a, &["2.0", "f"]),
            Err("invalid float literal".to_owned())
        );
    }

    for a in ["-d", "--division"] {
        assert_eq!(HANDLER.exec_func(a, &["1.0", "0.0"]), Ok("inf".to_owned()));
    }

    for a in ["-r", "--remainder"] {
        assert_eq!(HANDLER.exec_func(a, &["1.0", "0.0"]), Ok("NaN".to_owned()));
    }
}
