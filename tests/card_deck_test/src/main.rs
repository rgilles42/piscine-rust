use card_deck::*;

fn main() {
    let your_card = Card {
        rank: Rank::random(),
        suit: Suit::random(),
    };

    println!("Your card is {:?}", your_card);

    if card_deck::winner_card(your_card) {
        println!("You are the winner!");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // We cannot test the randomness as there's no 100% accurate consistent way to prove through a predicate that it yields a truly random number

    #[test]
    fn test_winner() {
        let winner = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };

        for rank in 1..14 {
            for suit in 1..5 {
                let card = Card {
                    rank: Rank::translate(rank),
                    suit: Suit::translate(suit),
                };

                assert_eq!(card_deck::winner_card(card), card == winner);
            }
        }
    }
}
