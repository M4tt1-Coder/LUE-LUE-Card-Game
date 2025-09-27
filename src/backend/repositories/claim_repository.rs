use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {


use axum::{http::StatusCode, Json};
use wasm_bindgen::JsValue;
use worker::D1Database;

use crate::backend::{
    errors::database_query_error::DatabaseQueryError,
    repositories::card_repository::CardRepository,
    types::{card::UpdateCardDTO, claim::Claim},
};

/// A database repository for interacting with the `claims` table.
///
/// Contains the utility functions for the `Claims` struct.
///
/// It will be accessable in the context element in the handler functions.
pub struct ClaimsRepository {
    db: D1Database,
}

// ----- Implementation of the 'ClaimsRepository' struct -----

impl ClaimsRepository {
    /// Returns a fresh instance of `ClaimsRepository` struct.
    ///
    /// # Arguments
    ///
    /// - `db` -> Database service pointer to execute queries.
    pub fn new(db: D1Database) -> Self {
        ClaimsRepository { db }
    }

    /// Gets a `Claim` struct from the database by using its ID.
    ///
    /// # Arguments
    ///
    /// - `id` -> Identifier of the `Claim` object.
    ///
    /// # Returns a `Claim` instance
    pub async fn get_claim_by_id(&self, id: String) -> Result<Claim, DatabaseQueryError<Claim>> {
        let query_result = self
            .db
            .prepare("SELECT * FROM claims WHERE id = ?;")
            .bind(&[JsValue::from(id.clone())])
            .unwrap()
            .first::<Claim>(None)
            .await;

        match query_result {
            Ok(fetched_claim) => match fetched_claim {
                Some(claim) => Ok(claim),
                None => Err(DatabaseQueryError {
                    message: format!("The claim with the id {} couldn't be found!", id),
                    received_data: None,
                    status_code: StatusCode::NOT_FOUND,
                }),
            },
            Err(err) => Err(DatabaseQueryError::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Retrieves all claims from the database, optionally filtered by game ID or player ID.
    ///
    /// # Arguments
    ///
    /// - `card_repository` -> Reference to the `CardRepository` to fetch cards associated with
    /// claims.
    /// - `game_id` -> Optional game ID to filter claims by game.
    /// - `player_id` -> Optional player ID to filter claims by player.
    /// If both are `None`, all claims will be returned.
    ///
    /// # Returns a vector of `Claim` instances or an error if the query fails.

    pub async fn get_all_claims(
        &self,
        game_id: Option<&str>,
        player_id: Option<&str>,
        card_repository: &CardRepository,
    ) -> Result<Vec<Claim>, DatabaseQueryError<Claim>> {
        let mut query = "SELECT * FROM claims".to_string();
        let mut params: Vec<JsValue> = Vec::new();

        if let Some(game_id) = game_id {
            query.push_str(" WHERE game_id = ?");
            params.push(JsValue::from(game_id));
        } else if let Some(player_id) = player_id {
            query.push_str(" WHERE created_by = ?");
            params.push(JsValue::from(player_id));
        }

        query.push_str(";");

        let query_result = self.db.prepare(&query).bind(&params).unwrap().all().await;

        match query_result {
            Ok(fetched_claims) => {
                let mut extracted_claims = match fetched_claims.results::<Claim>() {
                    Ok(claims) => claims,
                    Err(err) => {
                        return Err(DatabaseQueryError::new(
                            err.to_string(),
                            None,
                            StatusCode::INTERNAL_SERVER_ERROR,
                        ));
                    }
                };

                // get all cards in the claim
                extracted_claims.iter_mut().map(async |claim| {
                    let query_result = card_repository
                        .get_all_cards(Some(claim.id.clone()), None)
                        .await;

                    claim.cards = match query_result {
                        Ok(cards) => cards,
                        Err(err) => {
                            return Err(DatabaseQueryError::new(
                                err.message,
                                Some(Json(claim.clone())),
                                err.status_code,
                            ));
                        }
                    };

                    Ok(())
                });

                Ok(extracted_claims)
            }
            Err(err) => Err(DatabaseQueryError::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Uses a `Claim` struct to create a new claim entry in the database.
    ///
    /// # Arguments
    ///
    /// - `claim` -> The `Claim` struct to be inserted into the database.
    /// - `card_repository` -> Reference to the `CardRepository` to handle cards associated with
    /// the claim.
    ///
    /// # Returns a `Claim` instance if the insertion is successful, or an error if it fails.
    pub async fn create_claim(
        &self,
        claim: Claim,
        card_repository: &CardRepository,
    ) -> Result<Claim, DatabaseQueryError<Claim>> {
        let query =
            "INSERT INTO claims (id, created_by, number_of_cards, cards) VALUES (?, ?, ?, ?);";
        let params = vec![
            JsValue::from(claim.id.clone()),
            JsValue::from(claim.created_by.clone()),
            JsValue::from(claim.number_of_cards as i32),
        ];

        let query_result = self.db.prepare(query).bind(&params).unwrap().run().await;

        // cards need to be stored separatly
        for card in &claim.cards {
            let res = card_repository
                .update_card(
                    match UpdateCardDTO::new(card.id.clone(), None, None, Some(claim.id.clone())) {
                        Ok(update_card) => update_card,
                        Err(err) => {
                            return Err(DatabaseQueryError::new(
                                err.message,
                                Some(Json(claim.clone())),
                                StatusCode::INTERNAL_SERVER_ERROR,
                            ));
                        }
                    },
                )
                .await;
            if let Err(err) = res {
                return Err(DatabaseQueryError::new(
                    err.message,
                    Some(Json(claim.clone())),
                    err.status_code,
                ));
            }
        }

        match query_result {
            Ok(_) => Ok(claim),
            Err(err) => Err(DatabaseQueryError::new(
                err.to_string(),
                Some(Json(claim)),
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Deletes a claim from the database by its ID.
    ///
    /// # Arguments
    ///
    /// - `id` -> Identifier of the `Claim` object to be deleted.
    ///
    /// # Returns `Ok(())` if the deletion is successful, or an error if it fails.
    pub async fn delete_claim(&self, claim_id: String) -> Result<(), DatabaseQueryError<Claim>> {
        let query_result = self
            .db
            .prepare("DELETE FROM claims WHERE id = ?;")
            .bind(&[JsValue::from(claim_id)])
            .unwrap()
            .run()
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(err) => Err(DatabaseQueryError::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Deletes all claims associated with a specific game ID from the database.
    ///
    /// # Arguments
    ///
    /// - `game_id` -> Identifier of the game whose claims are to be deleted.
    ///
    /// # Returns `Ok(())` if the deletion is successful, or an error if it fails.
    pub async fn delete_all_claims_of_game(
        &self,
        game_id: &str,
    ) -> Result<(), DatabaseQueryError<Claim>> {
        let query_result = self
            .db
            .prepare("DELETE FROM claims WHERE game_id = ?;")
            .bind(&[JsValue::from(game_id)])
            .unwrap()
            .run()
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(err) => Err(DatabaseQueryError::new(
                err.to_string(),
                None,
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }
}
}}
