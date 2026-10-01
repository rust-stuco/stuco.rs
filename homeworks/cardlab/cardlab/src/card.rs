//! This module contains the definition and implementation of [`Card`], which represents a card in
//! a standard deck of 52 cards. [`Card`]s are uniquely identified by their rank (which can be the
//! numbers 2-10 or Jack, Queen, King, or Ace) and their suit (which can be Diamond, Club, Heart,
//! or Spade).
//!
//! Note to students: we would like to emphasize that you _do not_ have to represent / model playing
//! cards in the same way that we do.
//!
//! Additionally, we are aware that there is a _very easy_ way to implement the comparison traits
//! for all of the relevant subtypes (`Rank`, `Suit`, `Number`, etc.), and we have purposefully not
//! implemented them in that way in the reference solution.
//!
//! If you would like to use derived traits, you are allowed to. However, we felt that it would be
//! more instructive to manually write out the implementations in the reference solution.

/// Represents a standard playing card with a suit and a rank.
///
/// Each card consists of one of the four suits (Diamond, Club, Heart, Spade), as well as a rank
/// that can be either a number card (2-10) or a face card (Jack, Queen, King, Ace).
///
/// This type implements the traits [`PartialEq`], [`Eq`], [`PartialOrd`], and [`Ord`].
///
/// # Examples
///
/// ```
/// # use cardlab::Card;
/// #
/// let ace_spades = Card::new("spade", 14);
/// let king_spades = Card::new("spade", 13);
/// let ten_spades = Card::new("spade", 10);
/// let five_spades = Card::new("spade", 5);
/// let ace_hearts = Card::new("heart", 14);
/// let ace_clubs = Card::new("club", 14);
///
/// // Compare cards of same suit but different rank.
/// assert!(ace_spades > king_spades);
/// assert!(king_spades > ten_spades);
/// assert!(ten_spades > five_spades);
///
/// // Compare cards of same rank but different suit.
/// assert!(ace_spades > ace_hearts);
/// assert!(ace_hearts > ace_clubs);
///
/// // Test equality between identical cards.
/// assert!(ace_spades == Card::new("spade", 14));
/// ```
pub struct Card;

impl Card {
    /// Creates a new Card instance from a suit name and numeric rank.
    ///
    /// # Parameters
    ///
    /// * `suit` - A string representing the suit:
    ///   * `"diamond"`
    ///   * `"club"`
    ///   * `"heart"`
    ///   * `"spade"`
    /// * `rank` - A number from 1-13 representing the rank:
    ///   * 2-10: Number cards
    ///   * 11: Jack
    ///   * 12: Queen
    ///   * 13: King
    ///   * 14: Ace
    ///
    /// # Panics
    ///
    /// * If suit is not one of: `"spade"`, `"heart"`, `"club"`, `"diamond"`.
    /// * If rank is not in the range [2, 14].
    ///
    /// # Examples
    ///
    /// ```
    /// # use cardlab::Card;
    /// #
    /// let ace_of_spades = Card::new("spade", 14);
    /// let two_of_hearts = Card::new("heart", 2);
    /// let three_of_clubs = Card::new("club", 3);
    /// ```
    pub fn new(suit: &str, rank: u8) -> Self {
        todo!("implement me once you've finished modeling `Card`!")
    }

    /// Returns a string representation of this card's suit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cardlab::Card;
    /// #
    /// let card = Card::new("club", 3);
    /// assert_eq!(card.suit_name(), "club");
    /// ```
    pub fn suit_name(&self) -> &'static str {
        todo!("implement me once you've finished modeling `Card`!")
    }

    /// Returns the numeric rank of this card, where number cards = 2-10, and Jack = 11, Queen = 12,
    /// King = 13, and Ace = 14.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cardlab::Card;
    /// #
    /// let card = Card::new("club", 3);
    /// assert_eq!(card.rank_value(), 3);
    /// ```
    pub fn rank_value(&self) -> u8 {
        todo!("implement me once you've finished modeling `Card`!")
    }
}

/// Implements partial equality comparison between [`Card`]s. Two [`Card`]s are equal if they have
/// the same suit and rank.
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        todo!("implement me once you've finished modeling `Card`!")
    }
}

/// Implements total equality for [`Card`]. See the `PartialEq` implementation for more information.
///
/// Since equality is reflexive for [`Card`]s (`a == a`), we implement [`Eq`] for [`Card`].
///
/// For more information on why [`Eq`] does not have any methods, see the documentation for both
/// [`PartialEq`] and [`Eq`].
impl Eq for Card {}

/// Implements total ordering between [`Card`]s. [`Card`]s are compared first by rank, then by suit
/// (Diamond < Club < Heart < Spade) if ranks are equal.
impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!("implement me once you've finished modeling `Card`!")
    }
}

/// Implements partial ordering between [`Card`]s.
///
/// Since [`Card`]s can always be compared, this is just a wrapper around the `Ord` implementation.
/// See the `Ord` implementation for more information.
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
