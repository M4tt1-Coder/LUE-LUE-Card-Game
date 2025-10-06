use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {


use axum::{http::StatusCode, Json};
use wasm_bindgen::JsValue;
use worker::D1Database;

use crate::backend::{
    errors::{database_query_error::DatabaseQueryError, process_error::ProcessError, application_error::ApplicationError},
    types::card::{Card, UpdateCardDTO},
};

/// A database repository for interacting with the `cards` table.
///
/// Contains the utility functions for the `Card` struct.
///
/// It will be accessible in the context element in the handler functions.
pub struct CardRepository {
    /// Database pointer to execute queries.
    db: D1Database,
}

impl CardRepository {
    /// Returns a fresh instance of `CardRepository` struct.
    ///
    /// # Arguments
    ///
    /// - `db` -> Database service pointer to execute queries.
    ///
    /// # Returns a `CardRepository` instance.
    pub fn new(db: D1Database) -> Self {
        CardRepository { db }
    }

    /// Gets a `Card` struct from the database by using its ID.
    ///
    /// # Arguments
    ///
    /// - `claim_id` -> Identifier of the `Claim` object.
    /// - `player_id` -> Identifier of the `Player` object.
    ///
    /// # Returns a `Card` instance
    ///
    /// If both `claim_id` and `player_id` are provided, it returns an error.
    pub async fn get_all_cards(
        &self,
        claim_id: Option<String>,
        player_id: Option<String>,
    ) -> Result<Vec<Card>, Box<dyn ApplicationError>> {
        if claim_id.is_some() && player_id.is_some() {
            return Err(Box::new(DatabaseQueryError::<Card>::new(
                "Either claim_id or player_id must be provided, but not both.".to_string(),
                None,
                StatusCode::BAD_REQUEST,
            )));
        }

        let mut query = "SELECT * FROM cards".to_string();
        let mut params: Vec<JsValue> = Vec::new();

        if let Some(claim_id) = claim_id {
            query.push_str(" WHERE claim_id = ?");
            params.push(JsValue::from(claim_id));
        } else if let Some(player_id) = player_id {
            query.push_str(" WHERE player_id = ?");
            params.push(JsValue::from(player_id));
        }

        query.push(';');

        let query_result = self.db.prepare(&query).bind(&params).unwrap().all().await;

        match query_result {
            Ok(fetched_cards) => {
                let output_cards: Vec<Card> = match fetched_cards.results::<Card>() {
                    Ok(cards) => cards,
                    Err(err) => {
                        return Err(Box::new(DatabaseQueryError::<Card>::new(
                            err.to_string(),
                            None,
                            StatusCode::INTERNAL_SERVER_ERROR,
                        )));
                    }
                };

                Ok(output_cards)
            }
            Err(err) => Err(Box::new(DatabaseQueryError::<Card>::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    /// Gets a `Card` struct from the database by its ID.
    ///
    /// # Arguments
    ///
    /// - `id` -> Identifier of the `Card` object.
    ///
    /// # Returns a `Card` instance if found, or an error if not found or if the query fails.
    pub async fn get_card_by_id(&self, id: String) -> Result<Card, Box<dyn ApplicationError>> {
        let query = "SELECT * FROM cards WHERE id = ?;";
        let params = vec![JsValue::from(id)];

        let query_result = match self
            .db
            .prepare(query)
            .bind(&params)
                    {
            Ok(recv_cards) => recv_cards.first::<Card>(None).await,
            Err(err) => return Err(Box::new(DatabaseQueryError::<Card>::new(
                        err.to_string(),
                        None,
                        StatusCode::INTERNAL_SERVER_ERROR
                    )))
                };


        match query_result {
            Ok(fetched_card) => match fetched_card {
                Some(card) => Ok(card),
                None => Err(Box::new(DatabaseQueryError::<Card>::new(
                    "Card not found".to_string(),
                    None,
                    StatusCode::NOT_FOUND,
                ))),
            },
            Err(err) => Err(Box::new(DatabaseQueryError::<Card>::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    /// Deletes a `Card` from the database by its ID.
    ///
    /// # Arguments
    ///
    /// - `id` -> Identifier of the `Card` object to be deleted.
    ///
    /// # Returns `Ok(())` if the deletion was successful, or an error if the query fails.
    pub async fn delete_card(&self, id: String) -> Result<(), Box<dyn ApplicationError>> {
        let query = "DELETE FROM cards WHERE id = ?;";
        let params = vec![JsValue::from(id)];

        let query_result = match self.db.prepare(query).bind(&params) {
            Ok(recv_data) => recv_data.run().await,
            Err(err) => return Err(Box::new(DatabaseQueryError::<Card>::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR
            )))
        };

        match query_result {
            Ok(_) => Ok(()),
            Err(err) => Err(Box::new(DatabaseQueryError::<Card>::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    /// Creates a new `Card` in the database.
    ///
    /// # Arguments
    ///
    /// - `card` -> The `Card` struct to be inserted into the database.
    /// - `player_id` -> Identifier of the `Player` object to which the card belongs.
    ///
    /// # Returns a `Card` instance if the insertion is successful, or an error if it fails.
    pub async fn create_card(
        &self,
        card: Card,
        player_id: String,
    ) -> Result<Card, Box<dyn ApplicationError>> {
        let query = "INSERT INTO cards (id, card_type, player_id) VALUES (1?, 2?, 3?) RETURN *;";
        let params = vec![
            JsValue::from(card.id.clone()),
            JsValue::from(card.card_type.index()),
            JsValue::from(player_id),
        ];

        let query_result = match self
            .db
            .prepare(query)
            .bind(&params)
            {
                Ok(recv_data) => recv_data.first::<Card>(None).await,
                Err(err) => return Err(Box::new(DatabaseQueryError::<Card>::new(
                    err.to_string(),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR
                )))
            };

        match query_result {
            Ok(card_result) => match card_result {
                Some(created_card) => Ok(created_card),
                None => Err(Box::new(DatabaseQueryError::<Card>::new(
                    "Failed to create card".to_string(),
                    Some(Json(card)),
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))),
            },
            Err(err) => Err(Box::new(DatabaseQueryError::<Card>::new(
                err.to_string(),
                Some(Json(card)),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    /// Updates an existing `Card` in the database.
    ///
    /// # Arguments
    ///
    /// - `card_data` -> The `UpdateCardDTO` struct containing the data to update the card.
    ///
    /// # Returns a `Card` instance if the update is successful, or an error if it fails.
    pub async fn update_card(
        &self,
        card_data: UpdateCardDTO,
    ) -> Result<Card, Box<dyn ApplicationError>> {
        let (query, params) = match self.determine_query_and_bindings_to_update_card(&card_data) {
            Ok(result) => result,
            Err(err) => {
                return Err(Box::new(DatabaseQueryError::<Card>::new(
                    err.to_string(),
                    Some(Json(card_data.as_card())),
                    StatusCode::BAD_REQUEST,
                )))
            }
        };

        let query_result = match self
            .db
            .prepare(&query)
            .bind(&params)
            {
                Ok(recv_data) => recv_data.first::<Card>(None).await,
                Err(err) => return Err(Box::new(DatabaseQueryError::<Card>::new(
                    err.to_string(),
                    None,
                    StatusCode::INTERNAL_SERVER_ERROR
                    )))
            };

        match query_result {
            Ok(updated_card) => match updated_card {
                Some(card) => Ok(card),
                None => Err(Box::new(DatabaseQueryError::<Card>::new(
                    "Card not found and couldn't be updated!".to_string(),
                    None,
                    StatusCode::NOT_FOUND,
                ))),
            },
            Err(err) => Err(Box::new(DatabaseQueryError::<Card>::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    // ----- Helper functions for the 'CardRepository' struct -----

    /// Determines the SQL query and bindings to update a card based on the provided
    /// `UpdateCardDTO`.
    ///
    /// # Arguments
    ///
    /// - `card_data` -> The `UpdateCardDTO` containing the data to update the card.
    ///
    /// # Returns a tuple containing the SQL query string and a vector of bindings.
    fn determine_query_and_bindings_to_update_card(
        &self,
        card_data: &UpdateCardDTO,
    ) -> Result<(String, Vec<JsValue>), Box<dyn ApplicationError>> {
        if card_data.player_id.is_none()
            && card_data.claim_id.is_none()
            && card_data.card_type.is_none()
        {
            return Err(Box::new(ProcessError::<UpdateCardDTO>::new(
                "No new data was provided! The modifying attempt was aborted!".to_string(),
                "CardRepository::update_card".to_string(),
                Some(card_data.clone()),
            )));
        }

        let mut query = "UPDATE cards SET ".to_string();
        let mut params: Vec<JsValue> = Vec::new();

        if let Some(card_type) = &card_data.card_type {
            query.push_str("card_type = ?, ");
            params.push(JsValue::from(card_type.index()));
        }

        if let Some(player_id) = &card_data.player_id {
            query.push_str("player_id = ?, ");
            params.push(JsValue::from(player_id));
        }

        if let Some(claim_id) = &card_data.claim_id {
            query.push_str("claim_id = ?, ");
            params.push(JsValue::from(claim_id));
        }

        query.truncate(query.len() - 2); // Remove the last comma and space
        query.push_str(" WHERE id = ? RETURNING *;");
        params.push(JsValue::from(card_data.id.clone()));

        Ok((query, params))
    }
}
}
}
