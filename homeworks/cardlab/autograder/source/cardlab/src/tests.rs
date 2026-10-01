use crate::Card;

#[test]
fn test_simple_card_comparisons() {
    let ace_spades = Card::new("spade", 14);
    let king_spades = Card::new("spade", 13);
    let two_spades = Card::new("spade", 2);

    assert!(ace_spades > king_spades);
    assert!(king_spades > two_spades);
    assert!(ace_spades > two_spades);
}

#[test]
fn test_suit_comparisons() {
    let ace_spades = Card::new("spade", 14);
    let ace_hearts = Card::new("heart", 14);
    let ace_clubs = Card::new("club", 14);
    let ace_diamonds = Card::new("diamond", 14);

    assert!(ace_spades > ace_hearts);
    assert!(ace_hearts > ace_clubs);
    assert!(ace_clubs > ace_diamonds);
}

#[test]
fn test_equality() {
    let ace_spades1 = Card::new("spade", 14);
    let ace_spades2 = Card::new("spade", 14);
    let king_spades = Card::new("spade", 13);

    // Note that we do not use `assert_eq` and `assert_ne` here because that requires `Card` to
    // implement `Debug`, and for the purposes of this homework we will not require that.
    assert!(ace_spades1 == ace_spades2);
    assert!(ace_spades1 != king_spades);
}

#[test]
fn test_deck_sorting_easy() {
    let mut cards = [
        Card::new("diamond", 2),
        Card::new("spade", 14),
        Card::new("heart", 7),
        Card::new("club", 10),
        Card::new("diamond", 14),
        Card::new("spade", 2),
        Card::new("heart", 14),
        Card::new("club", 14),
    ];

    cards.sort();

    let expected = [
        Card::new("diamond", 2),
        Card::new("spade", 2),
        Card::new("heart", 7),
        Card::new("club", 10),
        Card::new("diamond", 14),
        Card::new("club", 14),
        Card::new("heart", 14),
        Card::new("spade", 14),
    ];

    assert!(cards == expected);
}

#[test]
fn test_deck_sorting_medium() {
    let mut cards = [
        Card::new("diamond", 2),
        Card::new("spade", 14),
        Card::new("heart", 7),
        Card::new("club", 10),
        Card::new("diamond", 14),
        Card::new("spade", 2),
        Card::new("heart", 14),
        Card::new("club", 14),
        Card::new("diamond", 8),
        Card::new("heart", 2),
        Card::new("club", 3),
        Card::new("spade", 11),
        Card::new("diamond", 13),
        Card::new("heart", 13),
        Card::new("club", 13),
        Card::new("spade", 13),
    ];

    cards.sort();

    let expected = [
        Card::new("diamond", 2),
        Card::new("heart", 2),
        Card::new("spade", 2),
        Card::new("club", 3),
        Card::new("heart", 7),
        Card::new("diamond", 8),
        Card::new("club", 10),
        Card::new("spade", 11),
        Card::new("diamond", 13),
        Card::new("club", 13),
        Card::new("heart", 13),
        Card::new("spade", 13),
        Card::new("diamond", 14),
        Card::new("club", 14),
        Card::new("heart", 14),
        Card::new("spade", 14),
    ];

    assert!(cards == expected);
}

#[test]
fn test_full_deck_sorting() {
    let mut cards = Vec::new();
    let suits = ["diamond", "club", "heart", "spade"];
    let ranks = (2..=14).collect::<Vec<_>>();

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

    // Sort the cards.
    cards.sort();

    // Verify that the cards are properly sorted.
    let mut prev_card = &cards[0];
    assert!(prev_card == &Card::new("diamond", 2));
    assert!(cards[51] == Card::new("spade", 14));

    for card in cards.iter().skip(1) {
        assert!(card >= prev_card);

        if card.suit_name() == "club" {
            assert_eq!(prev_card.suit_name(), "diamond");
            assert_eq!(prev_card.rank_value(), card.rank_value());
        } else if card.suit_name() == "heart" {
            assert_eq!(prev_card.suit_name(), "club");
            assert_eq!(prev_card.rank_value(), card.rank_value());
        } else if card.suit_name() == "spade" {
            assert_eq!(prev_card.suit_name(), "heart");
            assert_eq!(prev_card.rank_value(), card.rank_value());
        } else if card.suit_name() == "diamond" {
            assert_eq!(prev_card.suit_name(), "spade");
            assert_eq!(prev_card.rank_value(), card.rank_value() - 1);
        }

        prev_card = card;
    }
}
