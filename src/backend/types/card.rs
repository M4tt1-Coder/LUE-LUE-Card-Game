use std::{
    clone,
    fmt::{self, Display},
};

use serde::{Deserialize, Serialize};

// using statements
use crate::backend::enums::card_types::CardType;

use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use crate::backend::errors::{application_error::ErrorObject, process_error::ProcessError};
    }
}

// This module defines the `Card` struct, which represents a card in a card game.

/// The `Card` struct represents a card in a card game.
///
/// It contains a name and a type, which can be one of the predefined card types.
///
/// # Fields
/// - `card_type`: An enum representing the type of the card, such as King, Queen, Jack, Ace, or
#[derive(Deserialize, Serialize)]
pub struct Card {
    /// The unique identifier for the card, typically a string.
    pub id: String,
    /// The type of the card, represented by the `CardType` enum.
    pub card_type: CardType,
}

impl Card {
    /// Creates a new `Card` instance with the specified name and card type.
    ///
    /// # Arguments
    /// - `card_type`: The type of the card, represented by the `CardType` enum.
    ///
    /// # Returns
    /// A new `Card` instance.
    pub fn new(card_type: CardType) -> Self {
        Card {
            id: uuid::Uuid::new_v4().to_string(),
            card_type,
        }
    }
}

impl Default for Card {
    /// Provides a default implementation for the `Card` struct.
    ///
    /// # Returns
    /// A new `Card` instance with an empty name and a default card type (King).
    fn default() -> Self {
        Card::new(CardType::King)
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card Type: {}", self.card_type)
    }
}

impl clone::Clone for Card {
    fn clone(&self) -> Self {
        Card {
            id: self.id.clone(),
            card_type: match self.card_type {
                CardType::Ace => CardType::Ace,
                CardType::King => CardType::King,
                CardType::Joker => CardType::Jack,
                CardType::Queen => CardType::Queen,
                CardType::Jack => CardType::Jack,
            },
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card Type: {}, ID: {}", self.card_type, self.id)
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        impl<'a> ErrorObject<'a> for Card {}
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {


// ----- Implementation of the 'UpdateCardDTO' struct -----

/// The `UpdateCardDTO` struct is used to represent the data transfer object for updating a card.
///
/// It contains the necessary fields to identify the card and specify any updates to its
/// properties.
///
/// # Fields
///
/// - `id`: The unique identifier for the card to be updated.
/// - `card_type`: The new type of the card, if it is being updated.
/// - `player_id`: The ID of the player associated with the card, if applicable.
/// - `claim_id`: The ID of the claim associated with the card, if applicable.
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCardDTO {
    /// The unique identifier for the card to be updated.
    pub id: String,
    /// The new type of the card, if it is being updated.
    pub card_type: Option<CardType>,
    /// The ID of the player associated with the card, if applicable.
    pub player_id: Option<String>,
    /// The ID of the claim associated with the card, if applicable.
    pub claim_id: Option<String>,
}

impl UpdateCardDTO {
    /// Creates a new instance of `UpdateCardDTO`.
    ///
    /// # Arguments
    ///
    /// - `id`: The unique identifier for the card to be updated.
    /// - `card_type`: The new type of the card, if it is being updated.
    /// - `player_id`: The ID of the player associated with the card, if applicable.
    /// - `claim_id`: The ID of the claim associated with the card, if applicable.
    ///
    /// # Returns
    ///
    /// A new instance of `UpdateCardDTO`.
    pub fn new(
        id: String,
        card_type: Option<CardType>,
        player_id: Option<String>,
        claim_id: Option<String>,
    ) -> Result<Self, ProcessError<UpdateCardDTO>> {
        if id.is_empty() {
            return Err(ProcessError::new(
                "Card ID cannot be empty.".to_string(),
                "UpdateCardDTO::new".to_string(),
                None,
            ));
        }

        Ok(UpdateCardDTO {
            id,
            card_type,
            player_id,
            claim_id,
        })
    }

    /// Converts the `UpdateCardDTO` into a `Card` instance.
    ///
    /// # Returns
    ///
    /// A `Card` instance with the ID and card type from the DTO.
    pub fn as_card(&self) -> Card {
        Card {
            id: self.id.clone(),
            card_type: self.card_type.as_ref().unwrap_or(&CardType::King).clone(), // Default to King if not specified
        }
    }
}

impl Display for UpdateCardDTO {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "UpdateCardDTO {{ id: {}, card_type: {:?}, player_id: {:?}, claim_id: {:?} }}",
            self.id, self.card_type, self.player_id, self.claim_id
        )
    }
}

impl<'a> ErrorObject<'a> for UpdateCardDTO {}
}}
