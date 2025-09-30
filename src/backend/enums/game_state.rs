use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};

/// Represents the current state of the game.
///
/// This enum defines the possible states a game can be in, such as:
///
/// - `InProgress`: The game is currently being played.
///
/// - `Ended`: The game has concluded.
/// - `WaitingForPlayers`: The game is waiting for players to join.
/// - `Starting`: The game is in the process of starting, preparing for the first turn.
///
/// Each variant represents a distinct phase in the lifecycle of a game, allowing for clear
/// management and transitions between states.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum GameState {
    /// The game is currently in progress.
    InProgress,
    /// The game has ended.
    Ended,
    /// The game is waiting for players to join.
    WaitingForPlayers,
    /// The game is starting, preparing for the first turn.
    Starting,
}

impl GameState {
    /// Returns a string representation of the game state.
    ///
    /// # Returns
    /// A string slice representing the current game state.
    pub fn as_str(&self) -> &str {
        match self {
            GameState::InProgress => "In Progress",
            GameState::Ended => "Ended",
            GameState::WaitingForPlayers => "Waiting for Players",
            GameState::Starting => "Starting",
        }
    }

    /// Returns the index of the game state.
    ///
    /// # Returns
    /// A `usize` representing the index of the game state.
    ///
    /// # Index Mapping
    ///
    /// - `InProgress` is mapped to index `0`.
    /// - `Ended` is mapped to index `1`.
    /// - `WaitingForPlayers` is mapped to index `2`.
    /// - `Starting` is mapped to index `3`.
    ///
    pub fn index(&self) -> usize {
        match self {
            GameState::InProgress => 0,
            GameState::Ended => 1,
            GameState::WaitingForPlayers => 2,
            GameState::Starting => 3,
        }
    }

    /// Simply returns the number of all enum variants of the `GameState` enum as a *usize*.
    ///
    /// Needs to be updated if the number of variants is modified!
    pub fn number_of_values() -> usize {
        4
    }
}

// Implementing the `Display` trait for `GameState` allows for easy printing of the game state.

impl Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
