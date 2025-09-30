use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {


use axum::Json;
use wasm_bindgen::JsValue;
use worker::D1Database;

use crate::backend::{
    errors::database_query_error::DatabaseQueryError,
    repositories::card_repository::CardRepository,
    types::player::{Player, UpdatePlayerDTO},
};

/// Represents a repository for managing player data in the D1 database.
///
/// This repository provides methods to interact with player data stored in the D1 database,
/// including creating, updating, and retrieving player instances.
///
/// # Properties
///
/// `db`: An instance of `D1Database` that provides access to the D1 database.
pub struct PlayerRepository {
    /// The D1 database instance used for accessing player data.
    db: D1Database,
}

// ----- Implementation of 'PlayerRepository' -----

impl PlayerRepository {
    /// Creates a new `PlayerRepository` instance with the provided D1 database.
    ///
    /// # Arguments
    ///
    /// * `db` - An instance of `D1Database` to be used for database operations.
    ///
    /// # Returns
    ///
    /// A new `PlayerRepository` instance.
    pub fn new(db: D1Database) -> Self {
        PlayerRepository { db }
    }

    /// Adds a new player to the D1 database.
    ///
    /// # Arguments
    ///
    /// * `player` - A reference to the `Player` instance to be added to the database.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation, containing the added `Player`
    /// instance on success.
    ///
    /// # Errors
    ///
    /// If the database query fails, it returns a `DatabaseQueryError` containing the error
    /// details.
    pub async fn add_player(&self, player: Player) -> Result<Player, DatabaseQueryError<Player>> {
        let added_player = self
            .db
            .prepare(
                "INSERT INTO players (id, name, game_id, joined_at)
                    VALUES (1?, 2?, 3?, 4?) RETURNING *;",
            )
            .bind(&[
                JsValue::from(player.id.clone()),
                JsValue::from(player.name.clone()),
                JsValue::from(player.game_id.clone()),
                JsValue::from(player.joined_at.clone()),
            ])
            .unwrap()
            .first::<Player>(None)
            .await;

        match added_player {
            Ok(good_query_result) => match good_query_result {
                Some(result_player) => Ok(result_player),
                None => Err(DatabaseQueryError::new(
                    "Failed to add player to the database".to_string(),
                    Some(axum::Json(player)),
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                )),
            },
            Err(e) => Err(DatabaseQueryError::new(
                e.to_string(),
                Some(axum::Json(player)),
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Updates an existing player in the D1 database.
    ///
    /// # Arguments
    ///
    /// * `player` - A reference to the `Player` instance containing updated information.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation, containing the updated `Player`
    /// instance on success.
    ///
    /// # Errors
    ///
    /// If the database query fails, it returns a `DatabaseQueryError` containing the error
    /// details.
    pub async fn update_player(
        &self,
        player: UpdatePlayerDTO,
    ) -> Result<Player, DatabaseQueryError<UpdatePlayerDTO>> {
        // Prepare the SQL statement to update the player
        // Note: The SQL statement uses positional parameters (1?, 2?, etc.) for binding values.
        // This is a common practice to prevent SQL injection attacks.

        // get the bindings for the SQL statement
        // get the query string depending on what new data was provided

        let (query, bindings) = self.get_update_query_string_and_bindings(&player);

        let updated_player = self
            .db
            .prepare(&query)
            .bind(&bindings)
            .unwrap()
            .first::<Player>(None)
            .await;

        match updated_player {
            Ok(good_query_result) => match good_query_result {
                Some(result_player) => Ok(result_player),
                None => Err(DatabaseQueryError::new(
                    "Failed to update player in the database".to_string(),
                    Some(axum::Json(player)),
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                )),
            },
            Err(e) => Err(DatabaseQueryError::new(
                e.to_string(),
                Some(axum::Json(player)),
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Prepare the SQL statement to update the player
    ///
    /// # Arguments
    ///
    /// * `player` - A reference to the `UpdatePlayerDTO` instance containing updated information.
    ///
    /// # Returns
    ///
    /// A tuple containing the SQL query string and a vector of bindings for the query.
    ///
    /// The SQL query string is constructed based on the fields that are provided in the `player`
    /// instance. If a field is `None`, it is not included in the query.
    ///
    /// The bindings vector contains the values to be bound to the query parameters in the
    /// order they appear in the query string.
    fn get_update_query_string_and_bindings(
        &self,
        player: &UpdatePlayerDTO,
    ) -> (String, Vec<JsValue>) {
        let mut query = "UPDATE players SET ".to_string();
        let mut bindings = vec![];

        if let Some(name) = &player.name {
            query.push_str("name = ?, ");
            bindings.push(JsValue::from(name));
        }
        if let Some(score) = player.score {
            query.push_str("score = ?, ");
            bindings.push(JsValue::from(score));
        }

        if let Some(last_time_update_requested) = &player.last_time_update_requested {
            query.push_str("last_time_update_requested = ?, ");
            bindings.push(JsValue::from(last_time_update_requested));
        }

        // Remove the trailing comma and space
        query.truncate(query.len() - 2);
        query.push_str(" WHERE id = ? RETURNING *;");
        bindings.push(JsValue::from(player.id.clone()));

        (query, bindings)
    }

    /// Deletes a player from the D1 database.
    ///
    /// # Arguments
    ///
    /// * `player_id` - A string slice representing the ID of the player to be deleted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation.
    ///
    /// # Errors
    ///
    /// If the database query fails, it returns a `DatabaseQueryError` containing the error
    /// details.
    pub async fn delete_player(&self, player_id: &str) -> Result<(), DatabaseQueryError<Player>> {
        let deleted_player = self
            .db
            .prepare("DELETE FROM players WHERE id = ?;")
            .bind(&[JsValue::from(player_id)])
            .unwrap()
            .run()
            .await;

        match deleted_player {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseQueryError::new(
                e.to_string(),
                None,
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Retrieves a player by their ID from the D1 database.
    ///
    /// # Arguments
    ///
    /// * `player_id` - A string slice representing the ID of the player to be retrieved.
    ///
    /// # Returns
    ///
    /// A `Result` containing the retrieved `Player` instance on success, or a `DatabaseQueryError`
    /// on failure.
    ///
    pub async fn get_player(&self, player_id: &str) -> Result<Player, DatabaseQueryError<Player>> {
        let player = self
            .db
            .prepare("SELECT * FROM players WHERE id = ?;")
            .bind(&[JsValue::from(player_id)])
            .unwrap()
            .first::<Player>(None)
            .await;

        match player {
            Ok(good_query_result) => match good_query_result {
                Some(result_player) => Ok(result_player),
                None => Err(DatabaseQueryError::new(
                    "Player not found".to_string(),
                    None,
                    axum::http::StatusCode::NOT_FOUND,
                )),
            },
            Err(e) => Err(DatabaseQueryError::new(
                e.to_string(),
                None,
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Retrieves all players from the D1 database.
    ///
    /// # Arguments
    ///
    /// - `game_id` -> Optional game id after which either all players are return or just all
    /// players in a game.
    /// - `card_repository` -> Reference to the `CardRepository` to fetch cards associated with
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Player` instances on success, or a `DatabaseQueryError`
    /// on failure.
    pub async fn get_all_players(
        &self,
        game_id: Option<&str>,
        card_repository: &CardRepository,
    ) -> Result<Vec<Player>, DatabaseQueryError<Player>> {
        // depending on if a game id was passed to the function -> filter for the players of a
        // game
        let query_result = match game_id {
            None => {
                self.db
                    .prepare("SELECT * FROM players;")
                    .bind(&[])
                    .unwrap()
                    .all()
                    .await
            }
            Some(_game_id) => {
                self.db
                    .prepare("SELECT * FROM players WHERE game_id = ?;")
                    .bind(&[JsValue::from(_game_id)])
                    .unwrap()
                    .all()
                    .await
            }
        };
        match query_result {
            Ok(collect_players) => {
                let mut players: Vec<Player> = match collect_players.results::<Player>() {
                    Ok(results) => results,
                    Err(e) => {
                        return Err(DatabaseQueryError::new(
                            e.to_string(),
                            None,
                            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                        ));
                    }
                };
                // for each player, fetch their assigned cards
                for player in players.iter_mut() {
                    player.assigned_cards = match card_repository
                        .get_all_cards(None, Some(player.id.clone()))
                        .await
                    {
                        Ok(cards) => cards,
                        Err(err) => {
                            return Err(DatabaseQueryError::new(
                                err.to_string(),
                                Some(Json(player.clone())),
                                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            ));
                        }
                    };
                }

                if players.is_empty() {
                    Err(DatabaseQueryError::new(
                        "No players found".to_string(),
                        None,
                        axum::http::StatusCode::NOT_FOUND,
                    ))
                } else {
                    Ok(players)
                }
            }
            Err(e) => Err(DatabaseQueryError::new(
                e.to_string(),
                None,
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }
}

}}
