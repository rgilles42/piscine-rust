use dress_code::*;

fn main() {
    println!(
        "My outfit will be: {:?}",
        choose_outfit(Some(0), Ok("Dear friend, ..."))
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_choose_outfit_case_1() {
        let exp_outfit = Outfit {
            jacket: Jacket::Black,
            hat: Hat::Fedora,
        };
        assert_eq!(exp_outfit, choose_outfit(Some(0), Ok("someting")));
    }
    #[test]
    fn test_choose_outfit_case_2() {
        let exp_outfit = Outfit {
            jacket: Jacket::White,
            hat: Hat::Fedora,
        };
        assert_eq!(exp_outfit, choose_outfit(Some(1), Ok("someting")));
    }
    #[test]
    fn test_choose_outfit_case_3() {
        let exp_outfit = Outfit {
            jacket: Jacket::Black,
            hat: Hat::Snapback,
        };
        assert_eq!(exp_outfit, choose_outfit(Some(0), Err("someting")));
    }
    #[test]
    fn test_choose_outfit_case_4() {
        let exp_outfit = Outfit {
            jacket: Jacket::White,
            hat: Hat::Snapback,
        };
        assert_eq!(exp_outfit, choose_outfit(Some(10), Err("someting")));
    }
    #[test]
    fn test_choose_outfit_case_5() {
        let exp_outfit = Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Baseball,
        };
        assert_eq!(exp_outfit, choose_outfit(None, Err("someting")));
    }
    #[test]
    fn test_choose_outfit_case_6() {
        let exp_outfit = Outfit {
            jacket: Jacket::Flowers,
            hat: Hat::Fedora,
        };
        assert_eq!(exp_outfit, choose_outfit(None, Ok("someting")));
    }
}
