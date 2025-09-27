use std::fmt;

use log::warn;
use serde::{Deserialize, Serialize};

/// Card types for a card game.
///
/// This module defines the different types of cards that can be used in the game.
///
/// Each card type is represented by an enum variant, allowing for easy identification and handling
/// of different card types.
///
/// # Example usage:
/// ```rust
/// use your_crate::card_types::CardType;
/// let card = CardType::King;
/// match card {
///     CardType::King => println!("This is a King card."),
///     CardType::Queen => println!("This is a Queen card."),
///     CardType::Jack => println!("This is a Jack card."),
///     CardType::Ace => println!("This is an Ace card."),
///     CardType::Joker => println!("This is a Joker card."),
///     _ => println!("Unknown card type."),
/// }
/// ```
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum CardType {
    /// King card type.
    King,
    /// Queen card type.
    Queen,
    /// Jack card type.
    Jack,
    /// Ace card type.
    Ace,
    /// Joker card type.
    ///
    /// The Joker can be used as a wild card in the game.
    Joker,
}

impl CardType {
    /// Returns a string representation of the card type.
    ///
    /// # Returns
    /// A string slice representing the card type.
    pub fn as_str(&self) -> &str {
        match self {
            CardType::King => "King",
            CardType::Queen => "Queen",
            CardType::Jack => "Jack",
            CardType::Ace => "Ace",
            CardType::Joker => "Joker",
        }
    }

    /// Returns the index of the card type.
    ///
    /// # Returns
    ///
    /// A `usize` representing the index of the card type.
    ///
    /// # Index Mapping
    ///
    /// - `King` is mapped to index `0`.
    /// - `Queen` is mapped to index `1`.
    /// - `Jack` is mapped to index `2`.
    /// - `Ace` is mapped to index `3`.
    /// - `Joker` is mapped to index `4`.
    ///
    pub fn index(&self) -> usize {
        match self {
            CardType::King => 0,
            CardType::Queen => 1,
            CardType::Jack => 2,
            CardType::Ace => 3,
            CardType::Joker => 4,
        }
    }

    /// Simply returns the number of all enum variants of the `CardType` enum as a *usize*.
    ///
    /// Needs to be updated if the number of variants is modified!
    pub fn number_of_values() -> usize {
        5
    }

    /// Creates a new instance of `CardType` from a ***usize***.
    ///
    /// Makes sure that if an invalid number was provided that calculations still work properly.
    ///
    /// Covers all cases!
    pub fn from_usize(num: usize) -> Self {
        // make sure a valid number in the prefered range is used
        let used_num = num % Self::number_of_values();

        return match used_num {
            0 => CardType::King,
            1 => CardType::Queen,
            2 => CardType::Jack,
            3 => CardType::Ace,
            4 => CardType::Joker,
            5_usize.. => {
                warn!("When creating an instance of 'CardType' a provided was out of range of the allowed scope!");

                CardType::King
            }
        };
    }
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CardType::King => "King",
                CardType::Queen => "Queen",
                CardType::Jack => "Jack",
                CardType::Ace => "Ace",
                CardType::Joker => "Joker",
            }
        )
    }
}
