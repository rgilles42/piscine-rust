// Create a enum that represent the card suits
use card_deck::{self, Card, Rank, Suit};

// Write a program that takes that returns a random card in the deck
// A standard deck of cards has 52 cards: 4 suits and 13 cards per suit
#[allow(dead_code)]
fn main() {
    let your_card = Card {
        rank: Rank::random(),
        suit: Suit::random(),
    };

    println!("You're card is {:?}", your_card);

    // Now if the card is an Ace of Spades print "You are the winner"
    if card_deck::winner_card(&your_card) {
        println!("You are the winner!");
    }
}

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
            if card != winner {
                assert!(
                    !card_deck::winner_card(&card),
                    "\n\t{:?} is not the winner",
                    card
                );
            } else {
                assert!(
                    card_deck::winner_card(&card),
                    "\n\t{:?} is the winner",
                    card
                );
            }
        }
    }
}

#[test]
fn test_randomness() {
    let nb_tests = 10;
    let curr_card = Card {
        rank: Rank::random(),
        suit: Suit::random(),
    };
    let mut nb_same_rank = 0;
    let mut nb_same_suit = 0;
    for _ in 0..nb_tests {
        if curr_card.rank == Rank::random() {
            nb_same_rank += 1;
        }
        if curr_card.suit == Suit::random() {
            nb_same_suit += 1;
        }
    }
    assert!(nb_same_rank < nb_tests);
    assert!(nb_same_suit < nb_tests);
}
