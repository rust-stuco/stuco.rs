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
/// # use cardlab_ref::Card;
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
pub struct Card {
    rank: Rank,
    suit: Suit,
}

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
    /// # use cardlab_ref::Card;
    /// #
    /// let ace_of_spades = Card::new("spade", 14);
    /// let two_of_hearts = Card::new("heart", 2);
    /// let three_of_clubs = Card::new("club", 3);
    /// ```
    pub fn new(suit: &str, rank: u8) -> Self {
        // Convert the input `suit` integer into a `Suit` enum.
        // Note that this new `suit` "shadows" the input `suit`, becoming a new type.
        let suit = match suit {
            "diamond" => Suit::Diamond,
            "club" => Suit::Club,
            "heart" => Suit::Heart,
            "spade" => Suit::Spade,
            s => panic!("invalid suit {s}"),
        };

        // Convert the input `rank` integer into a `Rank` enum.
        let rank = match rank {
            2 => Rank::Number(Number::Two),
            3 => Rank::Number(Number::Three),
            4 => Rank::Number(Number::Four),
            5 => Rank::Number(Number::Five),
            6 => Rank::Number(Number::Six),
            7 => Rank::Number(Number::Seven),
            8 => Rank::Number(Number::Eight),
            9 => Rank::Number(Number::Nine),
            10 => Rank::Number(Number::Ten),
            11 => Rank::Face(Face::Jack),
            12 => Rank::Face(Face::Queen),
            13 => Rank::Face(Face::King),
            14 => Rank::Face(Face::Ace),
            n => panic!("invalid rank number {n}, expected a rank in the range [2, 14]"),
        };

        Card { suit, rank }
    }

    /// Returns a string representation of this card's suit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cardlab_ref::Card;
    /// #
    /// let card = Card::new("club", 3);
    /// assert_eq!(card.suit_name(), "club");
    /// ```
    pub fn suit_name(&self) -> &'static str {
        match self.suit {
            Suit::Diamond => "diamond",
            Suit::Club => "club",
            Suit::Heart => "heart",
            Suit::Spade => "spade",
        }
    }

    /// Returns the numeric rank of this card, where number cards = 2-10, and Jack = 11, Queen = 12,
    /// King = 13, and Ace = 14.
    ///
    /// # Examples
    ///
    /// ```
    /// # use cardlab_ref::Card;
    /// #
    /// let card = Card::new("club", 3);
    /// assert_eq!(card.rank_value(), 3);
    /// ```
    pub fn rank_value(&self) -> u8 {
        self.rank.rank_value()
    }
}

/// Implements partial equality comparison between [`Card`]s. Two [`Card`]s are equal if they have
/// the same suit and rank.
impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.suit == other.suit && self.rank == other.rank
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
        match self.rank.cmp(&other.rank) {
            std::cmp::Ordering::Equal => self.suit.cmp(&other.suit),
            ord => ord,
        }
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
/// # use cardlab_ref::Card;
/// #
/// let spade_card = Card::new("spade", 2);
/// let diamond_card = Card::new("diamond", 2);
/// assert!(spade_card > diamond_card);
/// ```
enum Suit {
    /// The Diamond suit, typically represented by a red ♦ symbol.
    Diamond,
    /// The Club suit, typically represented by a black ♣ symbol.
    Club,
    /// The Heart suit, typically represented by a red ♥ symbol.
    Heart,
    /// The Spade suit, typically represented by a black ♠ symbol.
    Spade,
}

impl Suit {
    /// Gives unique values to the suits to make ordering of suits easier.
    fn suit_value(&self) -> u8 {
        match self {
            Self::Diamond => 0,
            Self::Club => 1,
            Self::Heart => 2,
            Self::Spade => 3,
        }
    }
}

/// Represents the rank of a playing card, which can be either a number card (2-10) or a face card
/// (Ace, King, Queen, Jack).
///
/// # Examples
///
/// ```
/// # use cardlab_ref::Card;
/// #
/// let five_spades = Card::new("spade", 5);
/// let seven_spades = Card::new("spade", 7);
/// assert!(seven_spades > five_spades);
///
/// let queen_hearts = Card::new("heart", 12);
/// let seven_clubs = Card::new("club", 7);
/// assert!(queen_hearts > seven_clubs);
///
/// let king_diamonds = Card::new("diamond", 13);
/// let ace_clubs = Card::new("club", 14);
/// assert!(ace_clubs > king_diamonds);
/// ```
enum Rank {
    /// A number card (2-10).
    Number(Number),
    /// A face card (Ace, King, Queen, Jack).
    Face(Face),
}

impl Rank {
    /// Converts a rank to its numeric value (2-14), where the face cards are 11-14.
    fn rank_value(&self) -> u8 {
        match self {
            Self::Number(number) => number.number_value(),
            Self::Face(face) => face.face_value(),
        }
    }
}

/// Represents the possible numbers for cards in a deck.
///
/// # Examples
///
/// ```
/// # use cardlab_ref::Card;
/// #
/// let two_hearts = Card::new("heart", 2);
/// let seven_diamonds = Card::new("diamond", 7);
/// assert!(two_hearts < seven_diamonds);
/// ```
enum Number {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

impl Number {
    // Converts a `Number` into a proper integer.
    fn number_value(&self) -> u8 {
        match self {
            Self::Two => 2,
            Self::Three => 3,
            Self::Four => 4,
            Self::Five => 5,
            Self::Six => 6,
            Self::Seven => 7,
            Self::Eight => 8,
            Self::Nine => 9,
            Self::Ten => 10,
        }
    }
}

/// Represents the face cards in a deck, including Ace.
///
/// Note that even though Ace could be considered a number card in some games, we're grouping it
/// with the face cards for this assignment.
///
/// # Examples
///
/// ```
/// # use cardlab_ref::Card;
/// #
/// let queen_hearts = Card::new("heart", 12);
/// let jack_clubs = Card::new("club", 11);
/// assert!(queen_hearts > jack_clubs);
///
/// let ace_spades = Card::new("spade", 14);
/// let king_diamonds = Card::new("diamond", 13);
/// assert!(ace_spades > king_diamonds);
/// ```
enum Face {
    /// The Jack card, traditionally representing a royal servant or knight.
    Jack,
    /// The Queen card, traditionally representing a female royal figure.
    Queen,
    /// The King card, traditionally representing a male royal figure.
    King,
    /// The Ace card, which often has special rules in many card games.
    Ace,
}

impl Face {
    // Converts a `Face` value to an integer from 11-14.
    fn face_value(&self) -> u8 {
        match self {
            Self::Jack => 11,
            Self::Queen => 12,
            Self::King => 13,
            Self::Ace => 14,
        }
    }
}

/// Compares two suits for equality by comparing their discriminants.
impl PartialEq for Suit {
    fn eq(&self, other: &Self) -> bool {
        self.suit_value() == other.suit_value()
    }
}

/// Implements total equality for [`Suit`]. See the `PartialEq` implementation for more information.
impl Eq for Suit {}

/// Provides partial ordering for suits by delegating to the total ordering implementation.
impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Compares suits according to their standard order: Diamond < Club < Heart < Spade.
impl Ord for Suit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.suit_value().cmp(&other.suit_value())
    }
}

/// Compares two ranks for equality by comparing their numeric values.
impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        self.rank_value() == other.rank_value()
    }
}

/// Implements total equality for [`Rank`]. See the `PartialEq` implementation for more information.
impl Eq for Rank {}

/// Provides partial ordering for ranks by delegating to the total ordering implementation.
impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Compares ranks based on their numeric values from 2-14.
impl Ord for Rank {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.rank_value().cmp(&other.rank_value())
    }
}

/// Compares two number cards for equality by comparing their discriminants.
impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        self.number_value() == other.number_value()
    }
}

/// Implements total equality for [`Number`]. See the `PartialEq` implementation for more
/// information.
impl Eq for Number {}

/// Provides partial ordering for number cards by delegating to the total ordering implementation.
impl PartialOrd for Number {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Compares number cards based on their numeric values from 2-10.
impl Ord for Number {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.number_value().cmp(&other.number_value())
    }
}

/// Compares two face cards for equality by comparing their discriminants.
impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        self.face_value() == other.face_value()
    }
}

/// Implements total equality for [`Face`]. See the `PartialEq` implementation for more information.
impl Eq for Face {}

/// Provides partial ordering for face cards by delegating to the total ordering implementation.
impl PartialOrd for Face {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Compares face cards based on their values (Jack = 11, Queen = 12, King = 13, Ace = 14).
impl Ord for Face {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.face_value().cmp(&other.face_value())
    }
}
