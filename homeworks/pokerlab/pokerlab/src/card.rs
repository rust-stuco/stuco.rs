//! This module contains the definition and implementation of [`Card`], which represents a card in
//! a standard deck of 52 cards. [`Card`]s are uniquely identified by their rank (which can be the
//! numbers 2-10 or Jack, Queen, King, or Ace) and their suit (which can be Diamond, Club, Heart,
//! or Spade).
//!
//! Note to students: You are allowed to modify this file and replace the implementation with
//! something similar to your own implementation in Card Lab.
//!
//! However, make sure you understand that the function signatures are different, and that the
//! `Rank` and `Suit` types are now public! Additionally, we now no longer make a distinction
//! between suits. For example, we will say that the Ace of Spades has equal value to the Ace of
//! Diamonds.

use derivative::Derivative;

/// Represents a standard playing card with a suit and a rank.
///
/// Each card consists of one of the four suits (Diamond, Club, Heart, Spade), as well as a rank
/// that can be either a number card (2-10) or a face card (Jack, Queen, King, Ace).
///
/// This type implements the traits [`PartialEq`], [`Eq`], [`PartialOrd`], and [`Ord`]. Note that
/// `Card` only considers the [`Rank`] of the `Card` when doing comparisons, so the Three of Clubs
/// is considered to have equal value to the Three of Hearts.
///
/// This is achieved using the [`derivative`] macro, where instead of using all of the fields of a
/// struct to auto-implement a trait, we are able to ignore specific fields. If you have questions
/// about this, please do not hesitate to ask!
///
/// # Examples
///
/// ```
/// # use pokerlab::card::{Card, Suit, Rank};
/// #
/// let ace_spades = Card::new(Suit::Spade, Rank::Ace);
/// let king_spades = Card::new(Suit::Spade, Rank::King);
/// let ten_spades = Card::new(Suit::Spade, Rank::Ten);
/// let five_spades = Card::new(Suit::Spade, Rank::Five);
/// let ace_hearts = Card::new(Suit::Heart, Rank::Ace);
/// let ace_clubs = Card::new(Suit::Club, Rank::Ace);
///
/// // Compare cards of same suit but different rank.
/// assert!(ace_spades > king_spades);
/// assert!(king_spades > ten_spades);
/// assert!(ten_spades > five_spades);
///
/// // Compare cards with same rank
/// assert_eq!(ace_spades, ace_hearts);
/// assert_eq!(ace_hearts, ace_clubs);
///
/// // Test equality between identical cards.
/// assert_eq!(ace_spades, Card::new(Suit::Spade, Rank::Ace));
/// ```
#[derive(Debug, Clone, Copy, Derivative)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
pub struct Card {
    rank: Rank,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore", Ord = "ignore")]
    suit: Suit,
}

impl Card {
    /// Creates a new Card instance from a Suit and Rank.
    ///
    /// # Parameters
    ///
    /// * `suit` - The suit of the card as a [`Suit`] enum value
    /// * `rank` - The rank of the card as a [`Rank`] enum value
    ///
    /// # Examples
    ///
    /// ```
    /// # use pokerlab::card::{Card, Suit, Rank};
    /// #
    /// let ace_of_spades = Card::new(Suit::Spade, Rank::Ace);
    /// let two_of_hearts = Card::new(Suit::Heart, Rank::Two);
    /// let three_of_clubs = Card::new(Suit::Club, Rank::Three);
    /// ```
    pub fn new(suit: Suit, rank: Rank) -> Self {
        Card { suit, rank }
    }

    /// Returns a reference to this card's suit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pokerlab::card::{Card, Suit, Rank};
    /// #
    /// let card = Card::new(Suit::Club, Rank::Three);
    /// assert_eq!(Suit::Club, card.suit());
    /// ```
    pub fn suit(&self) -> Suit {
        self.suit
    }

    /// Returns a reference to this card's rank.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pokerlab::card::{Card, Suit, Rank};
    /// #
    /// let card = Card::new(Suit::Club, Rank::Three);
    /// assert_eq!(Rank::Three, card.rank());
    /// ```
    pub fn rank(&self) -> Rank {
        self.rank
    }
}

/// Represents the four possible suits in a standard deck of playing cards.
///
/// The suits are ordered in the traditional manner:
/// - Diamonds (lowest)
/// - Clubs
/// - Hearts
/// - Spades (highest)
///
/// # Examples
///
/// ```
/// # use pokerlab::card::{Card, Suit, Rank};
/// #
/// let spade_card = Card::new(Suit::Spade, Rank::Two);
/// let diamond_card = Card::new(Suit::Diamond, Rank::Two);
/// assert_eq!(spade_card, diamond_card);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    /// The Diamond suit, typically represented by a red ♦ symbol.
    Diamond,
    /// The Club suit, typically represented by a black ♣ symbol.
    Club,
    /// The Heart suit, typically represented by a red ♥ symbol.
    Heart,
    /// The Spade suit, typically represented by a black ♠ symbol.
    Spade,
}

/// Represents the rank of a playing card, which can be either a number card (2-10) or a face card
/// (Ace, King, Queen, Jack).
///
/// # Examples
///
/// ```
/// # use pokerlab::card::{Card, Suit, Rank};
/// #
/// let five_spades = Card::new(Suit::Spade, Rank::Five);
/// let seven_spades = Card::new(Suit::Spade, Rank::Seven);
/// assert!(seven_spades > five_spades);
///
/// let queen_hearts = Card::new(Suit::Heart, Rank::Queen);
/// let seven_clubs = Card::new(Suit::Club, Rank::Seven);
/// assert!(queen_hearts > seven_clubs);
///
/// let king_diamonds = Card::new(Suit::Diamond, Rank::King);
/// let ace_clubs = Card::new(Suit::Club, Rank::Ace);
/// assert!(ace_clubs > king_diamonds);
/// ```
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    /// The Jack card, traditionally representing a royal servant or knight.
    Jack = 11,
    /// The Queen card, traditionally representing a female royal figure.
    Queen = 12,
    /// The King card, traditionally representing a male royal figure.
    King = 13,
    /// The Ace card, which often has special rules in many card games.
    Ace = 14,
}
