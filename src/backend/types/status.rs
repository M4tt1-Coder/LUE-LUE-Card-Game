use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use axum::response::IntoResponse;
    }
}
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::backend::types::{game::Game, player::Player};

/// A simple request sent by a user for a status update.
///
/// An update should be sent in frequent delays or the player will be execluded in the game
/// session.
///
/// # Properties
///
/// - player_id: The ID of the player requesting the status update.
/// - game_id: The ID of the game for which the status update is requested.
#[derive(Deserialize, Serialize)]
pub struct StatusUpdateRequest {
    /// The ID of the player requesting the status update.
    pub player_id: String,
    /// The ID of the game for which the status update is requested.
    pub game_id: String,
}

/// Represents a requested update answer of a user.
///
/// Contains info about a new game state or own new data changes.
///
/// # Properties
/// - game_data: Optional game data that has been updated or changed.
/// - player_data: Optional player data that has been updated or changed.
/// - player_execluded_from_game: Indicates whether the player has been execluded from the game
///   session.
#[derive(Deserialize, Serialize)]
pub struct StatusUpdate {
    /// The game data that has been updated or changed.
    pub game_data: Option<Game>,
    /// The player data that has been updated or changed.
    pub player_data: Option<Player>,
    /// Indicates whether the player has been execluded from the game session.
    pub player_execluded_from_game: bool,
}

// ----- Implementation 'StatusUpdateRequest' -----

impl StatusUpdateRequest {
    /// Creates a new `StatusUpdateRequest` instance with the specified player ID and game ID.
    ///
    /// # Arguments
    /// - `player_id`: A string representing the ID of the player requesting the status update.
    /// - `game_id`: A string representing the ID of the game for which the status update is requested.
    ///
    /// # Returns
    /// A new `StatusUpdateRequest` instance.
    pub fn new(player_id: String, game_id: String) -> Self {
        StatusUpdateRequest { player_id, game_id }
    }
}

impl Default for StatusUpdateRequest {
    /// Creates a default `StatusUpdateRequest` instance with empty player ID and game ID.
    ///
    /// The 'player_id' and 'game_id' are initialized with new UUIds but should be changed
    /// immidiately.
    ///
    /// # Returns
    /// A new `StatusUpdateRequest` instance with default values.
    fn default() -> Self {
        StatusUpdateRequest {
            player_id: Uuid::new_v4().to_string(),
            game_id: Uuid::new_v4().to_string(),
        }
    }
}

// ----- Implementation 'StatusRequest' -----

impl StatusUpdate {
    /// Creates a new `StatusUpdate` instance with the specified game data, player data, and
    /// player execlusion status.
    ///
    /// # Arguments
    /// - `game_data`: An optional `Game` instance representing the updated game data.
    /// - `player_data`: An optional `Player` instance representing the updated player data.
    /// - `player_execluded_from_game`: A boolean indicating whether the player has been execluded
    ///   from the game session.
    ///
    /// # Returns
    /// A new `StatusUpdate` instance.
    pub fn new(
        game_data: Option<Game>,
        player_data: Option<Player>,
        player_execluded_from_game: bool,
    ) -> Self {
        StatusUpdate {
            game_data,
            player_data,
            player_execluded_from_game,
        }
    }
}

impl Default for StatusUpdate {
    /// Creates a default `StatusUpdate` instance with no game data, no player data, and the player
    /// not execluded from the game session.
    ///
    /// # Returns
    /// A new `StatusUpdate` instance with default values.
    fn default() -> Self {
        StatusUpdate {
            game_data: None,
            player_data: None,
            player_execluded_from_game: false,
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        // ----- Implementation 'IntoResponse' for 'StatusUpdate' -----

 impl IntoResponse for StatusUpdate {
    /// Converts the `StatusUpdate` instance into an HTTP response.
    ///
    /// # Returns
    /// An HTTP response containing the serialized `StatusUpdate` data.
    fn into_response(self) -> axum::response::Response {
        axum::Json(self).into_response()
    }
}
    }
}
