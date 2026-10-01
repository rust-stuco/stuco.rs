//! Tests for [`Card`] comparisons.
//!
//! Note that we are importing every variant of both [`Rank`] and [`Suit`] directly into this module
//! to make things easier to read.

use pokerlab::card::{Card, Rank::*, Suit::*};

#[test]
fn test_simple_card_comparisons() {
    let ace_spades = Card::new(Spade, Ace);
    let king_spades = Card::new(Spade, King);
    let two_spades = Card::new(Spade, Two);

    assert!(ace_spades > king_spades);
    assert!(king_spades > two_spades);
    assert!(ace_spades > two_spades);
}

#[test]
fn test_equality() {
    let ace_spades1 = Card::new(Spade, Ace);
    let ace_spades2 = Card::new(Spade, Ace);
    let king_spades = Card::new(Spade, King);

    assert_eq!(ace_spades1, ace_spades2);
    assert_ne!(ace_spades1, king_spades);
}

#[test]
fn test_deck_sorting_easy() {
    let mut cards = vec![
        Card::new(Club, Ten),
        Card::new(Spade, Ace),
        Card::new(Diamond, Two),
        Card::new(Heart, Seven),
    ];

    cards.sort();

    assert_eq!(
        cards,
        [
            Card::new(Diamond, Two),
            Card::new(Heart, Seven),
            Card::new(Club, Ten),
            Card::new(Spade, Ace)
        ]
    );
}

#[test]
fn test_deck_sorting_medium() {
    let mut cards = vec![
        Card::new(Club, Queen),
        Card::new(Heart, Eight),
        Card::new(Diamond, King),
        Card::new(Spade, Three),
        Card::new(Heart, Four),
        Card::new(Diamond, Six),
        Card::new(Club, Nine),
        Card::new(Diamond, Two),
        Card::new(Club, Five),
        Card::new(Spade, Seven),
        Card::new(Diamond, Ten),
        Card::new(Heart, Jack),
        Card::new(Spade, Ace),
    ];

    cards.sort();

    assert_eq!(
        cards,
        [
            Card::new(Diamond, Two),
            Card::new(Spade, Three),
            Card::new(Heart, Four),
            Card::new(Club, Five),
            Card::new(Diamond, Six),
            Card::new(Spade, Seven),
            Card::new(Heart, Eight),
            Card::new(Club, Nine),
            Card::new(Diamond, Ten),
            Card::new(Heart, Jack),
            Card::new(Club, Queen),
            Card::new(Diamond, King),
            Card::new(Spade, Ace),
        ]
    );
}

#[test]
fn test_full_deck_sorting() {
    let mut cards = Vec::new();
    let suits = [Diamond, Club, Heart, Spade];
    let ranks = [
        Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
    ];

    // Create full deck of 52 cards.
    for &suit in suits.iter() {
        for &rank in ranks.iter() {
            cards.push(Card::new(suit, rank));
        }
    }

    // Verify initial deck size.
    assert_eq!(cards.len(), 52);

    // Randomly shuffle the cards.
    use rand::seq::SliceRandom;
    let mut rng = rand::thread_rng();
    cards.shuffle(&mut rng);

    // Sort the cards. Note that the suits will be mixed up.
    cards.sort();

    // Verify that the cards are properly sorted.
    let mut prev_card = &cards[0];

    for card in cards.iter().skip(1) {
        assert!(card >= prev_card);
        prev_card = card;
    }
}
