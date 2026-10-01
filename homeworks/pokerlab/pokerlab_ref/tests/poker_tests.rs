//! Tests for [`PokerHand`] rank comparisons.
//!
//! Note that we are importing every variant of both [`Rank`] and [`Suit`] directly into this module
//! to make things easier to read.
//!
//! Disclaimer: many of these tests were generated with AI, since a lot of it consists of repetitive
//! boilerplate code. However, the specific test scenarios were manually crafted.

use pokerlab_ref::card::{Card, Rank::*, Suit::*};
use pokerlab_ref::hand::{Hand, PokerHand};

fn assert_tie(hand1: Hand, hand2: Hand) {
    let solved_hand1 = PokerHand::solve(hand1);
    let solved_hand2 = PokerHand::solve(hand2);

    assert_eq!(solved_hand1, solved_hand2);
}

fn assert_winner(winner: Hand, loser: Hand) {
    let solved_winner = PokerHand::solve(winner);
    let solved_loser = PokerHand::solve(loser);

    assert!(solved_winner > solved_loser);
}

#[test]
fn test_high_card_vs_high_card() {
    // Higher high card hand: (A♠ K♠ Q♠ J♦ 9♣).
    let higher_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Spade, King),
        Card::new(Spade, Queen),
        Card::new(Diamond, Jack),
        Card::new(Club, Nine),
    ])
    .unwrap();

    // Lower high card hand: (K♠ Q♠ J♠ 9♦ 8♣).
    let lower_hand = Hand::new([
        Card::new(Spade, King),
        Card::new(Spade, Queen),
        Card::new(Spade, Jack),
        Card::new(Diamond, Nine),
        Card::new(Club, Eight),
    ])
    .unwrap();

    assert_winner(higher_hand, lower_hand);
}

#[test]
fn test_high_card_tie() {
    // First high card hand: (A♠ K♠ Q♠ J♦ 9♣).
    let first_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Spade, King),
        Card::new(Spade, Queen),
        Card::new(Diamond, Jack),
        Card::new(Club, Nine),
    ])
    .unwrap();

    // Second high card hand: (A♣ K♣ Q♣ J♥ 9♦).
    let second_hand = Hand::new([
        Card::new(Club, Ace),
        Card::new(Club, King),
        Card::new(Club, Queen),
        Card::new(Heart, Jack),
        Card::new(Diamond, Nine),
    ])
    .unwrap();

    assert_tie(first_hand, second_hand);
}

#[test]
fn test_one_pair_vs_high_card() {
    // One pair hand: (Q♠ Q♣ A♦ K♥ J♣).
    let pair_hand = Hand::new([
        Card::new(Spade, Queen),
        Card::new(Club, Queen),
        Card::new(Diamond, Ace),
        Card::new(Heart, King),
        Card::new(Club, Jack),
    ])
    .unwrap();

    // High card hand: (A♠ K♠ Q♠ J♦ 9♣).
    let high_card_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Spade, King),
        Card::new(Spade, Queen),
        Card::new(Diamond, Jack),
        Card::new(Club, Nine),
    ])
    .unwrap();

    assert_winner(pair_hand, high_card_hand);
}

#[test]
fn test_one_pair_vs_one_pair() {
    // Higher pair: (Q♠ Q♣ A♦ K♥ J♣).
    let higher_pair_hand = Hand::new([
        Card::new(Spade, Queen),
        Card::new(Club, Queen),
        Card::new(Diamond, Ace),
        Card::new(Heart, King),
        Card::new(Club, Jack),
    ])
    .unwrap();

    // Lower pair: (5♠ 5♣ A♦ K♥ Q♣).
    let lower_pair_hand = Hand::new([
        Card::new(Spade, Five),
        Card::new(Club, Five),
        Card::new(Diamond, Ace),
        Card::new(Heart, King),
        Card::new(Club, Queen),
    ])
    .unwrap();

    assert_winner(higher_pair_hand, lower_pair_hand);
}

#[test]
fn test_two_pair_vs_one_pair() {
    // Two pair hand: (K♠ K♣ Q♦ Q♥ 2♣).
    let two_pair_hand = Hand::new([
        Card::new(Spade, King),
        Card::new(Club, King),
        Card::new(Diamond, Queen),
        Card::new(Heart, Queen),
        Card::new(Club, Two),
    ])
    .unwrap();

    // One pair hand: (A♠ A♣ K♦ Q♥ J♣).
    let one_pair_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Club, Ace),
        Card::new(Diamond, King),
        Card::new(Heart, Queen),
        Card::new(Club, Jack),
    ])
    .unwrap();

    assert_winner(two_pair_hand, one_pair_hand);
}

#[test]
fn test_two_pair_vs_two_pair() {
    // Higher two pair hand: (K♠ K♣ Q♦ Q♥ 2♣).
    let higher_hand = Hand::new([
        Card::new(Spade, King),
        Card::new(Club, King),
        Card::new(Diamond, Queen),
        Card::new(Heart, Queen),
        Card::new(Club, Two),
    ])
    .unwrap();

    // Lower two pair hand: (Q♠ Q♣ J♦ J♥ A♣).
    let lower_hand = Hand::new([
        Card::new(Spade, Queen),
        Card::new(Club, Queen),
        Card::new(Diamond, Jack),
        Card::new(Heart, Jack),
        Card::new(Club, Ace),
    ])
    .unwrap();

    assert_winner(higher_hand, lower_hand);
}

#[test]
fn test_two_pair_tie() {
    // First two pair hand: (K♠ K♣ Q♦ Q♥ A♣).
    let first_hand = Hand::new([
        Card::new(Spade, King),
        Card::new(Club, King),
        Card::new(Diamond, Queen),
        Card::new(Heart, Queen),
        Card::new(Club, Ace),
    ])
    .unwrap();

    // Second two pair hand: (K♦ K♥ Q♠ Q♣ A♦).
    let second_hand = Hand::new([
        Card::new(Diamond, King),
        Card::new(Heart, King),
        Card::new(Spade, Queen),
        Card::new(Club, Queen),
        Card::new(Diamond, Ace),
    ])
    .unwrap();

    assert_tie(first_hand, second_hand);
}

#[test]
fn test_three_of_a_kind_vs_three_of_a_kind() {
    // Higher three of a kind hand: (K♠ K♣ K♦ Q♥ J♣).
    let higher_hand = Hand::new([
        Card::new(Spade, King),
        Card::new(Club, King),
        Card::new(Diamond, King),
        Card::new(Heart, Queen),
        Card::new(Club, Jack),
    ])
    .unwrap();

    // Lower three of a kind hand: (Q♠ Q♣ Q♦ K♥ A♣).
    let lower_hand = Hand::new([
        Card::new(Spade, Queen),
        Card::new(Club, Queen),
        Card::new(Diamond, Queen),
        Card::new(Heart, King),
        Card::new(Club, Ace),
    ])
    .unwrap();

    assert_winner(higher_hand, lower_hand);
}

#[test]
fn test_straight_vs_two_pair() {
    // Straight hand: (9♠ 8♣ 7♦ 6♥ 5♣).
    let straight_hand = Hand::new([
        Card::new(Spade, Nine),
        Card::new(Club, Eight),
        Card::new(Diamond, Seven),
        Card::new(Heart, Six),
        Card::new(Club, Five),
    ])
    .unwrap();

    // Two pair hand: (K♠ K♣ Q♦ Q♥ A♣).
    let two_pair_hand = Hand::new([
        Card::new(Spade, King),
        Card::new(Club, King),
        Card::new(Diamond, Queen),
        Card::new(Heart, Queen),
        Card::new(Club, Ace),
    ])
    .unwrap();

    assert_winner(straight_hand, two_pair_hand);
}

#[test]
fn test_wheel_straight_vs_three_of_a_kind() {
    // Wheel straight hand: (5♠ 4♣ 3♦ 2♥ A♣).
    let wheel_straight_hand = Hand::new([
        Card::new(Spade, Five),
        Card::new(Club, Four),
        Card::new(Diamond, Three),
        Card::new(Heart, Two),
        Card::new(Club, Ace),
    ])
    .unwrap();

    // Three of a kind hand: (Q♠ Q♣ Q♦ K♥ A♣).
    let three_of_a_kind_hand = Hand::new([
        Card::new(Spade, Queen),
        Card::new(Club, Queen),
        Card::new(Diamond, Queen),
        Card::new(Heart, King),
        Card::new(Club, Ace),
    ])
    .unwrap();

    assert_winner(wheel_straight_hand, three_of_a_kind_hand);
}

#[test]
fn test_straight_vs_straight() {
    // Higher straight hand: (T♠ 9♣ 8♦ 7♥ 6♣).
    let higher_straight = Hand::new([
        Card::new(Spade, Ten),
        Card::new(Club, Nine),
        Card::new(Diamond, Eight),
        Card::new(Heart, Seven),
        Card::new(Club, Six),
    ])
    .unwrap();

    // Lower straight hand: (9♠ 8♣ 7♦ 6♥ 5♣).
    let lower_straight = Hand::new([
        Card::new(Spade, Nine),
        Card::new(Club, Eight),
        Card::new(Diamond, Seven),
        Card::new(Heart, Six),
        Card::new(Club, Five),
    ])
    .unwrap();

    assert_winner(higher_straight, lower_straight);
}

#[test]
fn test_straight_vs_straight_tie() {
    // First straight hand: (9♠ 8♣ 7♦ 6♥ 5♣).
    let first_straight = Hand::new([
        Card::new(Spade, Nine),
        Card::new(Club, Eight),
        Card::new(Diamond, Seven),
        Card::new(Heart, Six),
        Card::new(Club, Five),
    ])
    .unwrap();

    // Second straight hand: (9♥ 8♦ 7♣ 6♠ 5♦).
    let second_straight = Hand::new([
        Card::new(Heart, Nine),
        Card::new(Diamond, Eight),
        Card::new(Club, Seven),
        Card::new(Spade, Six),
        Card::new(Diamond, Five),
    ])
    .unwrap();

    assert_tie(first_straight, second_straight);
}

#[test]
fn test_flush_vs_one_pair() {
    // Flush hand: (J♠ T♠ 8♠ 7♠ 5♠).
    let flush_hand = Hand::new([
        Card::new(Spade, Jack),
        Card::new(Spade, Ten),
        Card::new(Spade, Eight),
        Card::new(Spade, Seven),
        Card::new(Spade, Five),
    ])
    .unwrap();

    // One pair hand: (A♠ A♣ K♦ Q♥ J♣).
    let one_pair_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Club, Ace),
        Card::new(Diamond, King),
        Card::new(Heart, Queen),
        Card::new(Club, Jack),
    ])
    .unwrap();

    assert_winner(flush_hand, one_pair_hand);
}

#[test]
fn test_flush_vs_two_pair() {
    // Flush hand: (J♠ T♠ 8♠ 7♠ 5♠).
    let flush_hand = Hand::new([
        Card::new(Spade, Jack),
        Card::new(Spade, Ten),
        Card::new(Spade, Eight),
        Card::new(Spade, Seven),
        Card::new(Spade, Five),
    ])
    .unwrap();

    // Two pair hand: (A♠ A♣ K♦ K♥ Q♣).
    let two_pair_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Club, Ace),
        Card::new(Diamond, King),
        Card::new(Heart, King),
        Card::new(Club, Queen),
    ])
    .unwrap();

    assert_winner(flush_hand, two_pair_hand);
}

#[test]
fn test_flush_vs_three_of_a_kind() {
    // Flush hand: (J♠ T♠ 8♠ 7♠ 5♠).
    let flush_hand = Hand::new([
        Card::new(Spade, Jack),
        Card::new(Spade, Ten),
        Card::new(Spade, Eight),
        Card::new(Spade, Seven),
        Card::new(Spade, Five),
    ])
    .unwrap();

    // Three of a kind hand: (A♠ A♣ A♦ K♥ Q♣).
    let three_of_a_kind_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Club, Ace),
        Card::new(Diamond, Ace),
        Card::new(Heart, King),
        Card::new(Club, Queen),
    ])
    .unwrap();

    assert_winner(flush_hand, three_of_a_kind_hand);
}

#[test]
fn test_flush_vs_straight() {
    // Flush hand: (A♠ K♠ Q♠ J♠ 9♠).
    let flush_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Spade, King),
        Card::new(Spade, Queen),
        Card::new(Spade, Jack),
        Card::new(Spade, Nine),
    ])
    .unwrap();

    // Straight hand: (T♠ 9♣ 8♦ 7♥ 6♣).
    let straight_hand = Hand::new([
        Card::new(Spade, Ten),
        Card::new(Club, Nine),
        Card::new(Diamond, Eight),
        Card::new(Heart, Seven),
        Card::new(Club, Six),
    ])
    .unwrap();

    assert_winner(flush_hand, straight_hand);
}

#[test]
fn test_flush_vs_flush_different_ranks() {
    // Higher flush hand: (A♠ K♠ Q♠ J♠ 9♠).
    let higher_flush = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Spade, King),
        Card::new(Spade, Queen),
        Card::new(Spade, Jack),
        Card::new(Spade, Nine),
    ])
    .unwrap();

    // Lower flush hand: (K♥ Q♥ J♥ T♥ 8♥).
    let lower_flush = Hand::new([
        Card::new(Heart, King),
        Card::new(Heart, Queen),
        Card::new(Heart, Jack),
        Card::new(Heart, Ten),
        Card::new(Heart, Eight),
    ])
    .unwrap();

    assert_winner(higher_flush, lower_flush);
}

#[test]
fn test_flush_vs_flush_tie() {
    // First flush hand: (A♠ K♠ Q♠ J♠ 9♠).
    let first_flush = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Spade, King),
        Card::new(Spade, Queen),
        Card::new(Spade, Jack),
        Card::new(Spade, Nine),
    ])
    .unwrap();

    // Second flush hand: (A♥ K♥ Q♥ J♥ 9♥).
    let second_flush = Hand::new([
        Card::new(Heart, Ace),
        Card::new(Heart, King),
        Card::new(Heart, Queen),
        Card::new(Heart, Jack),
        Card::new(Heart, Nine),
    ])
    .unwrap();

    assert_tie(first_flush, second_flush);
}

#[test]
fn test_full_house_vs_three_of_a_kind() {
    // Full house hand: (T♠ T♣ T♦ 9♥ 9♣).
    let full_house_hand = Hand::new([
        Card::new(Spade, Ten),
        Card::new(Club, Ten),
        Card::new(Diamond, Ten),
        Card::new(Heart, Nine),
        Card::new(Club, Nine),
    ])
    .unwrap();

    // Three of a kind hand: (A♠ A♣ A♦ K♥ Q♣).
    let three_of_a_kind_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Club, Ace),
        Card::new(Diamond, Ace),
        Card::new(Heart, King),
        Card::new(Club, Queen),
    ])
    .unwrap();

    assert_winner(full_house_hand, three_of_a_kind_hand);
}

#[test]
fn test_full_house_vs_flush() {
    // Full house hand: (K♠ K♣ K♦ Q♥ Q♣).
    let full_house_hand = Hand::new([
        Card::new(Spade, King),
        Card::new(Club, King),
        Card::new(Diamond, King),
        Card::new(Heart, Queen),
        Card::new(Club, Queen),
    ])
    .unwrap();

    // Flush hand: (A♠ K♠ Q♠ J♠ 9♠).
    let flush_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Spade, King),
        Card::new(Spade, Queen),
        Card::new(Spade, Jack),
        Card::new(Spade, Nine),
    ])
    .unwrap();

    assert_winner(full_house_hand, flush_hand);
}

#[test]
fn test_full_house_vs_full_house() {
    // Higher full house hand: (K♠ K♣ K♦ Q♥ Q♣).
    let higher_full_house = Hand::new([
        Card::new(Spade, King),
        Card::new(Club, King),
        Card::new(Diamond, King),
        Card::new(Heart, Queen),
        Card::new(Club, Queen),
    ])
    .unwrap();

    // Lower full house hand: (Q♠ Q♣ Q♦ A♥ A♣).
    let lower_full_house = Hand::new([
        Card::new(Spade, Queen),
        Card::new(Club, Queen),
        Card::new(Diamond, Queen),
        Card::new(Heart, Ace),
        Card::new(Club, Ace),
    ])
    .unwrap();

    assert_winner(higher_full_house, lower_full_house);
}

#[test]
fn test_four_of_a_kind_vs_straight() {
    // Four of a kind hand: (Q♠ Q♣ Q♦ Q♥ K♣).
    let four_of_a_kind_hand = Hand::new([
        Card::new(Spade, Queen),
        Card::new(Club, Queen),
        Card::new(Diamond, Queen),
        Card::new(Heart, Queen),
        Card::new(Club, King),
    ])
    .unwrap();

    // Straight hand: (A♠ K♣ Q♦ J♥ T♣).
    let straight_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Club, King),
        Card::new(Diamond, Queen),
        Card::new(Heart, Jack),
        Card::new(Club, Ten),
    ])
    .unwrap();

    assert_winner(four_of_a_kind_hand, straight_hand);
}

#[test]
fn test_four_of_a_kind_vs_full_house() {
    // Four of a kind hand: (K♠ K♣ K♦ K♥ Q♣).
    let four_of_a_kind_hand = Hand::new([
        Card::new(Spade, King),
        Card::new(Club, King),
        Card::new(Diamond, King),
        Card::new(Heart, King),
        Card::new(Club, Queen),
    ])
    .unwrap();

    // Full house hand: (A♠ A♣ A♦ K♥ K♣).
    let full_house_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Club, Ace),
        Card::new(Diamond, Ace),
        Card::new(Heart, King),
        Card::new(Club, King),
    ])
    .unwrap();

    assert_winner(four_of_a_kind_hand, full_house_hand);
}

#[test]
fn test_four_of_a_kind_vs_flush() {
    // Four of a kind hand: (7♠ 7♣ 7♦ 7♥ 8♣).
    let four_of_a_kind_hand = Hand::new([
        Card::new(Spade, Seven),
        Card::new(Club, Seven),
        Card::new(Diamond, Seven),
        Card::new(Heart, Seven),
        Card::new(Club, Eight),
    ])
    .unwrap();

    // Flush hand: (A♥ K♥ Q♥ J♥ 9♥).
    let flush_hand = Hand::new([
        Card::new(Heart, Ace),
        Card::new(Heart, King),
        Card::new(Heart, Queen),
        Card::new(Heart, Jack),
        Card::new(Heart, Nine),
    ])
    .unwrap();

    assert_winner(four_of_a_kind_hand, flush_hand);
}

#[test]
fn test_straight_flush_vs_high_card() {
    // Straight flush hand: (6♠ 5♠ 4♠ 3♠ 2♠).
    let straight_flush_hand = Hand::new([
        Card::new(Spade, Six),
        Card::new(Spade, Five),
        Card::new(Spade, Four),
        Card::new(Spade, Three),
        Card::new(Spade, Two),
    ])
    .unwrap();

    // High card hand: (A♠ K♣ Q♦ J♥ 9♣).
    let high_card_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Club, King),
        Card::new(Diamond, Queen),
        Card::new(Heart, Jack),
        Card::new(Club, Nine),
    ])
    .unwrap();

    assert_winner(straight_flush_hand, high_card_hand);
}

#[test]
fn test_straight_flush_vs_straight() {
    // Straight flush hand: (T♠ 9♠ 8♠ 7♠ 6♠).
    let straight_flush_hand = Hand::new([
        Card::new(Spade, Ten),
        Card::new(Spade, Nine),
        Card::new(Spade, Eight),
        Card::new(Spade, Seven),
        Card::new(Spade, Six),
    ])
    .unwrap();

    // Straight hand: (K♠ Q♣ J♦ T♥ 9♣).
    let straight_hand = Hand::new([
        Card::new(Spade, King),
        Card::new(Club, Queen),
        Card::new(Diamond, Jack),
        Card::new(Heart, Ten),
        Card::new(Club, Nine),
    ])
    .unwrap();

    assert_winner(straight_flush_hand, straight_hand);
}

#[test]
fn test_straight_flush_vs_flush() {
    // Straight flush hand: (T♠ 9♠ 8♠ 7♠ 6♠).
    let straight_flush_hand = Hand::new([
        Card::new(Spade, Ten),
        Card::new(Spade, Nine),
        Card::new(Spade, Eight),
        Card::new(Spade, Seven),
        Card::new(Spade, Six),
    ])
    .unwrap();

    // Flush hand: (A♥ K♥ Q♥ J♥ 9♥).
    let flush_hand = Hand::new([
        Card::new(Heart, Ace),
        Card::new(Heart, King),
        Card::new(Heart, Queen),
        Card::new(Heart, Jack),
        Card::new(Heart, Nine),
    ])
    .unwrap();

    assert_winner(straight_flush_hand, flush_hand);
}

#[test]
fn test_straight_flush_vs_full_house() {
    // Straight flush hand: (T♠ 9♠ 8♠ 7♠ 6♠).
    let straight_flush_hand = Hand::new([
        Card::new(Spade, Ten),
        Card::new(Spade, Nine),
        Card::new(Spade, Eight),
        Card::new(Spade, Seven),
        Card::new(Spade, Six),
    ])
    .unwrap();

    // Full house hand: (A♠ A♣ A♦ K♥ K♣).
    let full_house_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Club, Ace),
        Card::new(Diamond, Ace),
        Card::new(Heart, King),
        Card::new(Club, King),
    ])
    .unwrap();

    assert_winner(straight_flush_hand, full_house_hand);
}

#[test]
fn test_straight_flush_vs_four_of_a_kind() {
    // Straight flush hand: (T♠ 9♠ 8♠ 7♠ 6♠).
    let straight_flush_hand = Hand::new([
        Card::new(Spade, Ten),
        Card::new(Spade, Nine),
        Card::new(Spade, Eight),
        Card::new(Spade, Seven),
        Card::new(Spade, Six),
    ])
    .unwrap();

    // Four of a kind hand: (A♠ A♣ A♦ A♥ K♣).
    let four_of_a_kind_hand = Hand::new([
        Card::new(Spade, Ace),
        Card::new(Club, Ace),
        Card::new(Diamond, Ace),
        Card::new(Heart, Ace),
        Card::new(Club, King),
    ])
    .unwrap();

    assert_winner(straight_flush_hand, four_of_a_kind_hand);
}

#[test]
fn test_straight_flush_vs_straight_flush() {
    // Higher straight flush hand: (J♠ T♠ 9♠ 8♠ 7♠).
    let higher_straight_flush = Hand::new([
        Card::new(Spade, Jack),
        Card::new(Spade, Ten),
        Card::new(Spade, Nine),
        Card::new(Spade, Eight),
        Card::new(Spade, Seven),
    ])
    .unwrap();

    // Lower straight flush hand: (T♥ 9♥ 8♥ 7♥ 6♥).
    let lower_straight_flush = Hand::new([
        Card::new(Heart, Ten),
        Card::new(Heart, Nine),
        Card::new(Heart, Eight),
        Card::new(Heart, Seven),
        Card::new(Heart, Six),
    ])
    .unwrap();

    assert_winner(higher_straight_flush, lower_straight_flush);
}
