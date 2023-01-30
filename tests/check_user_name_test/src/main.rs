use check_user_name::*;

fn main() {
    let user0 = User::new("Didier".to_string(), AccessLevel::Admin);
    println!("{:?}", check_user_name(&user0));

    let user1 = User::new("Mary".to_string(), AccessLevel::Normal);
    println!("{:?}", check_user_name(&user1));

    let user2 = User::new("John".to_string(), AccessLevel::Guest);
    println!("{:?}", check_user_name(&user2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error() {
        let guest_case = User::new("Michael".to_string(), AccessLevel::Guest);
        assert_eq!(
            check_user_name(&guest_case),
            (false, "ERROR: User is guest")
        );
    }

    #[test]
    fn test_ok() {
        let admin_case = User::new("Fatima".to_string(), AccessLevel::Admin);
        let normal_case = User::new("Sara".to_string(), AccessLevel::Normal);
        assert_eq!(check_user_name(&admin_case), (true, "Fatima"));
        assert_eq!(check_user_name(&normal_case), (true, "Sara"));
    }

    #[test]
    fn test_send_name() {
        let admin_case = User::new("Fatima".to_string(), AccessLevel::Admin);
        let normal_case = User::new("Sara".to_string(), AccessLevel::Normal);
        let guest_case = User::new("Michael".to_string(), AccessLevel::Guest);
        assert_eq!(admin_case.send_name(), Some("Fatima"));
        assert_eq!(normal_case.send_name(), Some("Sara"));
        assert_eq!(guest_case.send_name(), None);
    }
}
