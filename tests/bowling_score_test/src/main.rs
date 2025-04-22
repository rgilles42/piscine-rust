use bowling_score::*;

fn main() -> Result<(), Error> {
    let mut game = BowlingGame::new();
    game.roll(0)?; // frame 1
    game.roll(10)?; // frame 1: spare
    game.roll(10)?; // frame 2: strike
    game.roll(5)?; // frame 3
    game.roll(5)?; // frame 3: spare
    game.roll(10)?; // frame 4: strike
    game.roll(10)?; // frame 5: strike
    game.roll(10)?; // frame 6: strike
    game.roll(10)?; // frame 7: strike
    game.roll(10)?; // frame 8: strike
    game.roll(10)?; // frame 9: strike
    game.roll(10)?; // frame 10: strike
    game.roll(2)?; // fill ball 1
    game.roll(8)?; // fill ball 2
    println!("{:?}", game.score());

    let mut perfect_game = BowlingGame::new();
    for _ in 0..12 {
        perfect_game.roll(10)?;
    }
    println!("{:?}", perfect_game.score());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject_example() {
        let mut game = BowlingGame::new();
        let _ = game.roll(0);
        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(5);
        let _ = game.roll(5);
        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(10);
        let _ = game.roll(2);
        let _ = game.roll(8);

        assert_eq!(Some(252), game.score());
    }

    #[test]
    fn test_not_enough_pins() {
        let mut game = BowlingGame::new();
        assert_eq!(game.roll(11), Err(Error::NotEnoughPinsLeft));
        let _ = game.roll(6);
        assert_eq!(game.roll(5), Err(Error::NotEnoughPinsLeft));
    }

    #[test]
    fn test_perfect_game() {
        let mut game = BowlingGame::new();
        for _ in 0..12 {
            let _ = game.roll(10);
        }
        assert_eq!(game.score(), Some(300));
    }

    #[test]
    fn test_game_already_complete() {
        let mut game = BowlingGame::new();
        for _ in 0..10 {
            let _ = game.roll(0);
            let _ = game.roll(0);
        }
        assert_eq!(game.roll(0), Err(Error::GameComplete));
    }

    #[test]
    fn test_only_one_fill_ball_after_spare() {
        let mut game = BowlingGame::new();
        for _ in 0..9 {
            let _ = game.roll(0);
            let _ = game.roll(0);
        }
        assert!(game.roll(5).is_ok());
        assert!(game.roll(5).is_ok());
        assert!(game.roll(2).is_ok());
        assert_eq!(game.roll(0), Err(Error::GameComplete));
        assert_eq!(Some(12), game.score());
    }

    #[test]
    fn test_two_fill_balls_after_strike() {
        let mut game = BowlingGame::new();
        for _ in 0..9 {
            let _ = game.roll(0);
            let _ = game.roll(0);
        }
        assert!(game.roll(10).is_ok());
        assert!(game.roll(1).is_ok());
        assert!(game.roll(1).is_ok());
        assert_eq!(game.roll(0), Err(Error::GameComplete));
        assert_eq!(Some(12), game.score());
    }

    #[test]
    fn test_no_more_balls_after_fill_balls_even_on_strikes() {
        let mut game = BowlingGame::new();
        for _ in 0..9 {
            let _ = game.roll(0);
            let _ = game.roll(0);
        }
        assert!(game.roll(10).is_ok());
        assert!(game.roll(10).is_ok());
        assert!(game.roll(10).is_ok());
        assert_eq!(game.roll(0), Err(Error::GameComplete));
        assert_eq!(Some(30), game.score());
    }
}
