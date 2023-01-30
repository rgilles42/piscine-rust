use borrow_box::*;

fn main() {
    let mut game = GameSession::new(0, String::from("Joao"), String::from("Susana"), 5);
    println!("{:?}", game.read_winner());
    // output : ("Same score! tied", 0)

    game.update_score(String::from("Joao"));
    game.update_score(String::from("Joao"));
    game.update_score(String::from("Susana"));
    game.update_score(String::from("Susana"));
    println!("{:?}", game.read_winner());
    // output : ("Same score! tied", 2)

    game.update_score(String::from("Joao"));
    // this one will not count because it already 5 games played, the nb_games
    game.update_score(String::from("Susana"));

    println!("{:?}", game.read_winner());
    // output : ("Joao", 3)

    println!("{:?}", game.delete());
    // output : "game deleted: id -> 0"

    // game.read_winner();
    // this will give error
    // because the game was dropped, no longer exists on the heap
}
#[cfg(test)]
mod tests {
    use super::*;

    fn create_games() -> Vec<Box<GameSession>> {
        vec![
            GameSession::new(0, String::from("player1"), String::from("player2"), 1),
            GameSession::new(1, String::from("Alice"), String::from("Mark"), 3),
            GameSession::new(2, String::from("Jack"), String::from("Miller"), 5),
        ]
    }

    #[test]
    fn test_create() {
        let games = create_games();
        assert_eq!(
            *games[0],
            GameSession {
                id: 0,
                p1: (String::from("player1"), 0),
                p2: (String::from("player2"), 0),
                nb_games: 1
            }
        );
        assert_eq!(
            *games[1],
            GameSession {
                id: 1,
                p1: (String::from("Alice"), 0),
                p2: (String::from("Mark"), 0),
                nb_games: 3
            }
        );
        assert_eq!(
            *games[2],
            GameSession {
                id: 2,
                p1: (String::from("Jack"), 0),
                p2: (String::from("Miller"), 0),
                nb_games: 5
            }
        );
    }

    #[test]
    fn test_update_and_read() {
        let mut games = create_games();
        games[0].update_score(String::from("player1"));
        assert_eq!(games[0].read_winner(), (String::from("player1"), 1));

        games[0].update_score(String::from("player2"));
        // this will stay the same because the nb_games is 1 so if one
        // of the players wins just once it will no longer increment the score
        assert_eq!(games[0].read_winner(), (String::from("player1"), 1));

        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Miller"));
        games[2].update_score(String::from("Miller"));
        // tie between players
        assert_eq!(
            games[2].read_winner(),
            (String::from("Same score! tied"), 2)
        );

        games[2].update_score(String::from("Jack"));
        assert_eq!(games[2].read_winner(), (String::from("Jack"), 3));
    }

    #[test]
    fn test_stop_updating() {
        let mut games = create_games();
        games[0].update_score(String::from("player1"));
        games[0].update_score(String::from("player1"));
        assert_eq!(games[0].read_winner(), (String::from("player1"), 1));

        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        games[2].update_score(String::from("Jack"));
        assert_eq!(games[2].read_winner(), (String::from("Jack"), 3));
    }

    #[test]
    fn test_delete() {
        let game = GameSession::new(0, String::from("Alice"), String::from("Mark"), 3);
        let game1 = GameSession::new(23, String::from("Jack"), String::from("Miller"), 1);

        assert_eq!(game.delete(), String::from("game deleted: id -> 0"));
        assert_eq!(game1.delete(), String::from("game deleted: id -> 23"));
    }

    #[test]
    fn test_different_name() {
        let mut game = GameSession::new(0, String::from("Alice"), String::from("Mark"), 3);

        game.update_score(String::from("Mark"));
        assert_eq!(game.read_winner(), (String::from("Mark"), 1));

        game.update_score(String::from("Miller"));
        assert_eq!(game.read_winner(), (String::from("Mark"), 1));
    }

    // #[test]
    // #[should_panic]
    // fn test_delete_ownership() {
    //     let game = new(0, String::from("Alice"), String::from("Mark"), 3);
    //     {
    //         let a = &game;
    //         // error! cant destroy boxed while the inner value is borrowed later in scope
    //         delete(game);
    //         read_winner(&a);
    //     }
    // }
}
