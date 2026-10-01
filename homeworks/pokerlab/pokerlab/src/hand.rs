//! Module for poker hand evaluation and comparison.
//!
//! There is a bit of Poker lingo that this file uses. If you find yourself confused by anything,
//! please reach out to us and ask! The term that will probably be most confusing is a "kicker".
//! A "kicker", also known as a "side card", is simply a card that does not take part in determine
//! the rank of a hand. For example, if we want to compare 2 four-of-a-kinds, the fifth card in each
//! hand would be considered the kicker. Note that kickers are still important in breaking ties.
//!
//! A good way to approach this assignment is to first read the code in `src/card.rs`, and then
//! read all of the starter code in this file (`src/hand.rs`). If you are unfamiliar with Poker, you
//! may also want to read up on the hands you can make on [Wikipedia]. Finally (as a good rule of
//! thumb for any programming course), make sure to go over the test cases to make sure you have a
//! correct idea of what you should implement.
//!
//! You are also allowed to change _any_ of the private fields of structs in this file, as long as
//! you are able to implement the all of the [`Hand`] methods without issue. You are also allowed to
//! modify the [`PokerHand::solve`] method, but be aware that the test cases are _solely_ using that
//! method to check the correctness of your implementation, so make sure not to play around with it
//! too much.
//!
//! [Wikipedia]: https://en.wikipedia.org/wiki/List_of_poker_hands

use crate::card::{Card, Rank};

/// Represents different poker hand rankings with their respective cards. Each variant contains the
/// relevant cards that make up the hand.
///
/// Note to students: When you auto derive [`PartialOrd`] and [`Ord`], the enum variants are ordered
/// by declaration order. For example, [`HighCard`] will always come before [`OnePair`].
///
/// This type has easily derivable comparison traits, as later [`PokerHand`] variants always beat
/// earlier ones, and each of the variants are able to be compared with themselves.
///
/// See the integration tests in `tests/poker_tests.rs` for examples.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PokerHand {
    HighCard(HighCard),
    OnePair(OnePair),
    TwoPair(TwoPair),
    ThreeOfAKind(ThreeOfAKind),
    Straight(Straight),
    Flush(Flush),
    FullHouse(FullHouse),
    FourOfAKind(FourOfAKind),
    StraightFlush(StraightFlush),
}

/// Represents a high card hand, consisting of five unpaired cards.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct HighCard {
    /// The highest card rank in the hand.
    high_card: Rank,
    /// The remaining four card ranks in descending order.
    kickers: [Rank; 4],
}

/// Represents a hand containing one pair and three kickers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct OnePair {
    /// The rank of the matched pair of cards.
    pair: Rank,
    /// The remaining three card ranks in descending order.
    kickers: [Rank; 3],
}

/// Represents a hand containing two pairs and one kicker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TwoPair {
    /// The higher ranked pair.
    first_pair: Rank,
    /// The lower ranked pair.
    second_pair: Rank,
    /// The remaining unpaired card.
    kicker: Rank,
}

/// Represents a hand containing three cards of the same rank and two kickers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ThreeOfAKind {
    /// The rank of the three matched cards.
    triple: Rank,
    /// The remaining two card ranks in descending order.
    kickers: [Rank; 2],
}

/// Represents five consecutive cards of different suits, storing only the highest card rank.
///
/// Straights can range from Ace-high straights to wheels. A wheel refers to the lowest possible
/// 5-high straight (Ace, 2, 3, 4, 5). In this case, even though the Ace is normally the highest
/// card, it counts as a low card (like a 1), making a wheel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Straight {
    /// The highest rank in the straight (highest can be an Ace, lowest is a 5 for a wheel).
    high_card: Rank,
}

/// Represents five cards of the same suit. The suit of the cards is ignored since we do not need it
/// for comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Flush {
    /// The ranks of the five cards in descending order.
    ranks: [Rank; 5],
}

/// Represents a hand containing three cards of one rank and two cards of another rank.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FullHouse {
    /// The rank of the three matched cards.
    triple: Rank,
    /// The rank of the two matched cards.
    pair: Rank,
}

/// Represents a hand containing four cards of the same rank and one kicker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FourOfAKind {
    /// The rank shared by all four cards.
    quad: Rank,
    /// The remaining unpaired card rank.
    kicker: Rank,
}

/// Represents five consecutive cards of the same suit. The suit of the cards is ignored since we do
/// not need it for comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct StraightFlush {
    /// The highest rank in the straight (highest can be an Ace, lowest is a 5 for a wheel).
    high_card: Rank,
}

impl PokerHand {
    /// Given 5 cards as input, creates a `PokerHand` with the correct ranking.
    ///
    /// Note that there is definitely a cleaner way to implement Poker hands, but by breaking it
    /// down into these specific types we can support partial implementation that don't handle every
    /// single kind of Poker hand!
    pub fn solve(hand: Hand) -> Self {
        // Check for a straight flush first (highest ranking hand).
        if let Some(straight_flush) = hand.get_straight_flush() {
            return PokerHand::StraightFlush(straight_flush);
        }

        // Check for a four of a kind.
        if let Some(four_of_a_kind) = hand.get_four_of_a_kind() {
            return PokerHand::FourOfAKind(four_of_a_kind);
        }

        // Check for a full house.
        if let Some(full_house) = hand.get_full_house() {
            return PokerHand::FullHouse(full_house);
        }

        // Check for a flush.
        if let Some(flush) = hand.get_flush() {
            return PokerHand::Flush(flush);
        }

        // Check for a straight.
        if let Some(straight) = hand.get_straight() {
            return PokerHand::Straight(straight);
        }

        // Check for a three of a kind.
        if let Some(three_of_a_kind) = hand.get_triple() {
            return PokerHand::ThreeOfAKind(three_of_a_kind);
        }

        // Check for a two pair.
        if let Some(two_pair) = hand.get_two_pair() {
            return PokerHand::TwoPair(two_pair);
        }

        // Check for a one pair.
        if let Some(one_pair) = hand.get_one_pair() {
            return PokerHand::OnePair(one_pair);
        }

        // If no other hand is found, it's a high card hand.
        PokerHand::HighCard(HighCard {
            high_card: hand.cards[0].rank(),
            kickers: [
                hand.cards[1].rank(),
                hand.cards[2].rank(),
                hand.cards[3].rank(),
                hand.cards[4].rank(),
            ],
        })
    }
}

/// Represents a standard `Hand` of 5 playing [`Card`]s.
///
/// Note that the cards will always be stored in sorted descending order (only by [`Rank`]).
#[derive(Debug)]
pub struct Hand {
    /// The cards in the hand.
    cards: [Card; 5],
}

impl Hand {
    /// Creates a new `Hand` of 5 [`Card`]s.
    ///
    /// Stores the cards in reverse (descending) sorted order.
    ///
    /// Returns [`None`] if any cards are duplicates.
    pub fn new(mut cards: [Card; 5]) -> Option<Self> {
        // Sort in reverse order.
        cards.sort_by(|a, b| b.cmp(a));

        // Check for any duplicate cards.
        for i in 0..4 {
            // Need to also check if the suits are equal since `Card` equality does not consider
            // suits when checking comparison.
            if cards[i] == cards[i + 1] && cards[i].suit() == cards[i + 1].suit() {
                return None;
            }
        }

        Some(Self { cards })
    }

    /// Returns a [`StraightFlush`] if the hand contains consecutive cards of the same suit,
    /// otherwise returns `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_straight_flush(&self) -> Option<StraightFlush> {
        todo!("check if the current hand is both a straight and a flush")
    }

    /// Returns a [`FourOfAKind`] if the hand contains four cards of the same rank, otherwise
    /// returns `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_four_of_a_kind(&self) -> Option<FourOfAKind> {
        todo!("check if the current hand contains all four cards of a specific rank")
    }

    /// Returns a [`FullHouse`] if the hand contains a triple and a pair, otherwise returns `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_full_house(&self) -> Option<FullHouse> {
        todo!("check if the current hand has both three of a kind and a pair")
    }

    /// Returns a [`Flush`] if all cards in the hand have the same suit, otherwise returns `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_flush(&self) -> Option<Flush> {
        todo!("check if all cards in the current hand have the same suit")
    }

    /// Returns a [`Straight`] if the cards form a straight (consecutive ranks), otherwise returns
    /// `None`.
    ///
    /// This function can also handle wheels / Ace-low straights (Ace, 2, 3, 4, 5).
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_straight(&self) -> Option<Straight> {
        todo!("check if the current hand contains contiguously-ranked cards")
    }

    /// Returns a [`ThreeOfAKind`] if the hand contains three cards of the same rank, otherwise
    /// returns `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_triple(&self) -> Option<ThreeOfAKind> {
        todo!("check if the current hand contains three cards with equal rank")
    }

    /// Returns a [`TwoPair`] if the hand contains two pairs, otherwise `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_two_pair(&self) -> Option<TwoPair> {
        todo!("check if the current hand contains two pairs of two cards with equal rank")
    }

    /// Returns a [`OnePair`] if the hand contains a pair of cards with matching rank, otherwise
    /// returns `None`.
    ///
    /// This function assumes that all higher-ranking hand checks have been called.
    pub fn get_one_pair(&self) -> Option<OnePair> {
        todo!("check if the current hand contains two cards with equal rank")
    }
}
