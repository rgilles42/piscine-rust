use own_and_return::*;

fn main() {
    let my_film = Film {
        name: "Terminator".to_string(),
    };
    // println!("{}", take_film_name(my_film));
    // uncomment the previous line to check that the value is consumed
    println!("{}", read_film_name(&my_film));
    println!("{}", take_film_name(my_film));
    // you can test this function by commenting out the first print statement, you should see the expected output without errors in this case
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consume() {
        assert_eq!(
            take_film_name(Film {
                name: "Matrix".to_string()
            }),
            "Matrix".to_string()
        );
    }
    #[test]
    fn test_only_print() {
        assert_eq!(
            read_film_name(&Film {
                name: "Matrix".to_string()
            }),
            "Matrix".to_string()
        )
    }
}
