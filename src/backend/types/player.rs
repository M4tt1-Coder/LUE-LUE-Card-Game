use cfg_if::cfg_if;

use std::fmt::Display;

cfg_if! {
    if #[cfg(feature = "ssr")] {
 use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::backend::{errors::application_error::ErrorObject};
    }
}
use crate::backend::types::card::Card;
use serde::{Deserialize, Serialize};

/// Player struct representing a player in the game system.
///
/// He / she can be identified by a unique ID.
///
/// Contains data set by the user like the name, etc. ...
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Player {
    /// Unique identifier of the player.
    pub id: String,

    /// Name of the player.
    pub name: String,

    /// Score of the player in the game.
    pub score: usize,

    /// The date and time when the player joined the game.
    pub joined_at: String,

    /// The cards assigned to the player.
    pub assigned_cards: Vec<Card>,

    /// The ID of the game the player is currently in.
    ///
    /// This field is used to associate the player with a specific game instance.
    pub game_id: String,

    /// The last time a player requested a status updated.
    ///
    /// If the time exceeds 5 minutes the player will be deleted from the gaming session.
    pub last_time_update_requested: String,
}

impl Player {
    /// Creates a new `Player` instance with the specified name and an empty card list.
    ///
    /// # Arguments
    /// - `name`: A string representing the name of the player.
    ///
    /// # Returns
    /// A new `Player` instance with a unique ID, the provided name, and an empty card list.
    pub fn new(name: String, game_id: String) -> Self {
        Player {
            id: uuid::Uuid::new_v4().to_string(),
            name,
            game_id,
            score: 0,
            joined_at: chrono::Utc::now().to_string(),
            assigned_cards: Vec::new(),
            last_time_update_requested: chrono::Utc::now().to_string(),
        }
    }

    // ----- Implementation for 'Vec<Player>' to be serialized to JSON -----

    /// Converts a vector of `Player` instances into a JSON string.
    ///     
    /// # Arguments
    ///
    /// - `players`: A vector of `Player` instances to be serialized.
    ///
    /// # Returns
    /// A `Result` containing the serialized JSON string on success, or an error if serialization
    /// fails.
    pub fn list_to_json(players: Vec<Player>) -> Result<String, serde_json::Error> {
        serde_json::to_string(&players)
    }
}

// ----- Implementation of 'Display' trait for Player -----

impl Display for Player {
    /// Formats the `Player` instance as a string.
    ///
    /// # Returns
    /// A string representation of the `Player` instance, including the player's ID, name, score,
    /// and joined date.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Player ID: {}, Name: {}, Score: {}, Joined At: {}, Game ID: {}",
            self.id, self.name, self.score, self.joined_at, self.game_id
        )
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {


impl<'a> ErrorObject<'a> for Player {}

// ----- DTO for updating a player entity -----

/// Data Transfer Object (DTO) for updating a player's information.
///
/// This struct is used to encapsulate the data required to update a player's
/// information in the system. It includes optional fields for the player's name,
/// score, and assigned cards, allowing for partial updates.
///
/// # Fields
///
/// - `id`: The unique identifier of the player to be updated.
/// - `name`: An optional new name for the player.
/// - `score`: An optional new score for the player.
/// - `assigned_cards`: An optional list of new cards assigned to the player.
#[derive(Deserialize, Debug)]
pub struct UpdatePlayerDTO {
    /// The unique identifier of the player to be updated.
    pub id: String,

    /// The new name for the player.
    pub name: Option<String>,

    /// The new score for the player.
    pub score: Option<usize>,

    /// The new game ID for the player.
    pub assigned_cards: Option<Vec<Card>>,

    /// The last time when the client requested a status update
    pub last_time_update_requested: Option<String>,
}

impl UpdatePlayerDTO {
    /// Creates a new `UpdatePlayerDTO` instance with the specified player ID.
    ///
    /// # Arguments
    /// - `id`: The unique identifier of the player to be updated.
    /// - `name`: An optional new name for the player.
    /// - `score`: An optional new score for the player.
    /// - `assigned_cards`: An optional list of new cards assigned to the player.
    ///
    /// # Returns
    /// A new `UpdatePlayerDTO` instance with the provided player ID and default values for other fields.
    pub fn new(
        id: String,
        name: Option<String>,
        score: Option<usize>,
        assigned_cards: Option<Vec<Card>>,
        last_time_update_requested: Option<String>,
    ) -> Self {
        UpdatePlayerDTO {
            id,
            name,
            score,
            assigned_cards,
            last_time_update_requested,
        }
    }
}

// ----- Implementation of 'ErrorObject' trait for 'UpdatePlayerDTO' -----

impl Display for UpdatePlayerDTO {
    /// Formats the `UpdatePlayerDTO` instance as a string.
    ///
    /// # Returns
    /// A string representation of the `UpdatePlayerDTO` instance, including the player's ID,
    /// name, score, and last time updated.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "UpdatePlayerDTO ID: {}, Name: {:?}, Score: {:?}, Last time when update requested: {:?}",
            self.id, self.name, self.score, self.last_time_update_requested
        )
    }
}

impl<'a> ErrorObject<'a> for UpdatePlayerDTO {}

// ----- Implementation of 'IntoResponse' trait for 'Player' -----
impl IntoResponse for Player {
    /// Converts the `Player` instance into a response.
    ///
    /// # Returns
    /// A `Response` containing the serialized `Player` instance.
    fn into_response(self) -> Response {
        (StatusCode::OK, self).into_response()
    }
}
    }
}
