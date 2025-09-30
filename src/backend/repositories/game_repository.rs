use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {


use crate::backend::{
    errors::{application_error::ApplicationError, database_query_error::DatabaseQueryError},
    repositories::{card_repository::CardRepository, chat::{chat_message_repository::ChatMessageRepository, chat_repository::ChatRepository}, claim_repository::ClaimsRepository, player_repository::PlayerRepository},
    types::{
        chat::{Chat, ChatMessage},
        claim::Claim,
        game::{Game, UpdateGameDTO},
        player::Player,
    },
};
use axum::{http::StatusCode, Json};
use wasm_bindgen::JsValue;
use worker::D1Database;

/// Represents a repository for managing game data in the D1 database.
///
/// This repository provides methods to interact with the game data stored in the D1 database,
/// including creating, updating, and retrieving game instances.
///
/// # Properties
///
/// `db`: An instance of `D1Database` that provides access to the D1 database.
pub struct GameRepository {
    /// The D1 database instance used for accessing game data.
    db: D1Database,
}

impl GameRepository {
    /// Creates a new `GameRepository` instance with the provided D1 database.
    ///
    /// # Arguments
    ///
    /// * `db` - An instance of `D1Database` to be used for database operations.
    ///
    /// # Returns
    ///
    /// A new `GameRepository` instance.
    pub fn new(db: D1Database) -> Self {
        GameRepository { db }
    }

    // pub fn db(&self) -> &D1Database {
    //    &self.db
    // }

    /// Adds a new game to the D1 database.
    ///
    /// # Arguments
    ///
    /// * `game` - A reference to the `Game` instance to be added to the database.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation.
    pub async fn add_game(&self, game: Game) -> Result<Game, DatabaseQueryError<Game>> {
        let added_game = self
            .db
            .prepare(
                "INSERT INTO games (id, started_at, round_number, state, which_players_turn, card_to_play)
                    VALUES (1?, 2?, 3?, 4?, 5?, 6?) RETURNING *;",
            )
            .bind(&[
                JsValue::from(game.id),
                JsValue::from(game.started_at),
                JsValue::from(game.round_number),
                JsValue::from(game.state.index()),
                JsValue::from(game.which_player_turn),
                JsValue::from(game.card_to_play.index()),
            ]).unwrap().first::<Game>(None).await;

        match added_game {
            Ok(game) => match game {
                Some(game) => Ok(game),
                None => Err(DatabaseQueryError::new(
                    "Failed to add game to the database".to_string(),
                    None,
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                )),
            },
            Err(err) => Err(DatabaseQueryError::new(
                err.to_string(),
                None,
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    /// Updates an existing game in the D1 database.
    ///
    /// # Arguments
    ///
    /// - `game` - A reference to the `Game` instance containing updated information.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation.
    pub async fn update_game(
        &self,
        game_data: UpdateGameDTO,
        player_repo: &PlayerRepository,
        claims_repo: &ClaimsRepository,
        card_repo: &CardRepository,
        chat_repo: &ChatRepository,
        chat_message_repo: &ChatMessageRepository
    ) -> Result<Game, Box<dyn ApplicationError>> {
        let (query, bindings) = self.get_update_query_string_and_bindings(&game_data);

        let query_result = self
            .db
            .prepare(&query)
            .bind(&bindings)
            .unwrap()
            .first::<Game>(None)
            .await;

        match query_result {
            Ok(game) => match game {
                Some(mut updated_game) => {
                    updated_game.players = match self.update_players_in_game(&game_data, player_repo, card_repo).await {
                        Ok(players) => players,
                        Err(err) => return Err(Box::new(err))
                    };

                    updated_game.claims = match self.update_claims_of_game(&game_data, claims_repo, card_repo).await {
                        Ok(claims) => claims,
                        Err(err) => return Err(Box::new(err))
                    };

                    updated_game.chat = match self.update_chat_of_game(&game_data, chat_repo, chat_message_repo).await {
                        Ok(chat) => chat,
                        Err(err) => return Err(err)
                    };

                    return Ok(updated_game);
                },
                None => Err(Box::new(DatabaseQueryError::<Game>::new(
                    "Failed to update game in the database".to_string(),
                    None,
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                ))),
            },
            Err(err) => Err(Box::new(DatabaseQueryError::<Game>::new(
                err.to_string(),
                None,
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    /// Retrieves a game by its ID from the D1 database.
    ///
    /// # Arguments
    ///
    /// * `game_id` - A string slice representing the ID of the game to be retrieved.
    ///
    /// # Returns
    ///
    /// A `Result` containing an `Game` struct object if the game is found, or a `DatabaseQueryError` if
    /// an error occurs.
    pub async fn get_game_by_id(
        &self,
        game_id: &str,
        chat_repo: &ChatRepository,
        player_repo: &PlayerRepository,
        claim_repo: &ClaimsRepository,
        chat_message_repo: &ChatMessageRepository,
        card_repo: &CardRepository
    ) -> Result<Game, Box<dyn ApplicationError>> {
        let query_result = self
            .db
            .prepare("SELECT * FROM games WHERE id = ?;")
            .bind(&[JsValue::from(game_id)])
            .unwrap()
            .first::<Game>(None)
            .await;

        match query_result {
            Ok(game) => match game {
                Some(mut game) => {
                    game.chat = match chat_repo.get_chat(None, Some(&game.id), chat_message_repo).await {
                        Ok(chat) => chat,
                        Err(err) => return Err(err)
                    };

                    game.players = match player_repo.get_all_players(Some(&game.id), card_repo).await {
                        Ok(players) => players,
                        Err(err) => return Err(Box::new(err))
                    };
                    game.claims = match claim_repo.get_all_claims(Some(&game.id), None, card_repo).await {
                        Ok(claims) => claims,
                        Err(err) => return Err(Box::new(err))
                    };
                    Ok(game)
                },
                None => Err(Box::new(DatabaseQueryError::<Game>::new(
                    "Game not found".to_string(),
                    None,
                    axum::http::StatusCode::NOT_FOUND,
                ))),
            },
            Err(err) => Err(Box::new(DatabaseQueryError::<Game>::new(
                err.to_string(),
                None,
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    /// Retrieves all games from the D1 database.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of `Game` instances if successful, or a `DatabaseQueryError`
    /// if an error occurs.
    pub async fn get_all_games(&self, player_repo: &PlayerRepository, card_repo: &CardRepository, claims_repo: &ClaimsRepository, chat_repo: &ChatRepository, chat_message_repo: &ChatMessageRepository) -> Result<Vec<Game>, Box<dyn ApplicationError>> {
        let query_result = self
            .db
            .prepare("SELECT * FROM games;")
            .bind(&[])
            .unwrap()
            .all()
            .await;

        match query_result {
            Ok(collected_games) => {
                let mut output: Vec<Game> = collected_games.results::<Game>().unwrap();

                if output.is_empty() {
                    Err(Box::new(DatabaseQueryError::<Game>::new(
                        "No games found".to_string(),
                        None,
                        axum::http::StatusCode::NOT_FOUND,
                    )))
                } else {

                    // Retrieve all other necessary game data (players, claims, chat) here
                    for game in &mut output {
                        // players
                        let players = match player_repo.get_all_players(Some(&game.id), card_repo).await {
                            Ok(players) => players,
                            Err(err) => return Err(Box::new(err))
                        };
                        // Assign players to the game
                        game.players = players;

                        // claims
                        let claims = match claims_repo.get_all_claims(Some(&game.id), None, card_repo).await {
                            Ok(claims) => claims,
                            Err(err) => return Err(Box::new(err))
                        };

                        // Assign claims to the game
                        game.claims = claims;

                        // Retrieve chat for the game
                        let chat = match chat_repo.get_chat(None, Some(&game.id), chat_message_repo).await {
                            Ok(chat) => chat,
                            Err(err) => return Err(err)
                        };
                        // Assign chat to the game
                        game.chat = chat;
                    }
                    Ok(output)
                }
            }
            Err(err) => Err(Box::new(DatabaseQueryError::<Game>::new(
                err.to_string(),
                None,
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            ))),
        }
    }

    /// Deletes a game by its ID from the D1 database.
    ///
    /// # Arguments
    ///
    /// * `game_id` - A string slice representing the ID of the game to be deleted.
    ///
    /// # Returns
    ///
    /// A `Result` indicating success or failure of the operation.
    pub async fn delete_game(&self, game_id: &str) -> Result<(), DatabaseQueryError<Game>> {
        let query_result = self
            .db
            .prepare("DELETE FROM games WHERE id = ?;")
            .bind(&[JsValue::from(game_id)])
            .unwrap()
            .run()
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(err) => Err(DatabaseQueryError::new(
                err.to_string(),
                None,
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }

    // ----- utility functions of the 'GameRepository' struct -----

    /// Combines all properties together that are directly stored in the 'games' table.
    ///
    /// Fields that weren't supposed to be updated aren't included.
    ///
    /// # Arguments
    ///
    /// - `game_data` -> DTO object which holds new data stored in the `games` table
    fn get_update_query_string_and_bindings(
        &self,
        game_data: &UpdateGameDTO,
    ) -> (String, Vec<JsValue>) {
        let mut output_query = "UPDATE games SET ".to_string();
        let mut output_bindings = vec![];

        // game state
        if let Some(state) = &game_data.state {
            output_query.push_str("state = ?, ");
            output_bindings.push(JsValue::from(state.index()));
        }

        // round number
        if let Some(round) = game_data.round_number {
            output_query.push_str("round_number = ?, ");
            output_bindings.push(JsValue::from(round));
        }

        // card to play
        if let Some(card) = &game_data.card_to_play {
            output_query.push_str("card_to_play = ?, ");
            output_bindings.push(JsValue::from(card.index()));
        }

        // which players turn it is
        if let Some(player) = &game_data.which_player_turn {
            output_query.push_str("which_player_turn = ?, ");
            output_bindings.push(JsValue::from(player));
        }

        output_query.truncate(output_query.len() - 2);
        output_query.push_str(" WHERE id = ? RETURNING *;");
        output_bindings.push(JsValue::from(game_data.id.clone()));

        (output_query, output_bindings)
    }

    /// Fetches all curent players of the game stored in the database and then determines which
    /// entities to delete or add.
    ///
    /// # Returns
    ///
    /// - List of `Player`, which was passed to the function.
    ///
    /// # Arguments
    ///
    /// - `game_data` -> DTO object containing the list players
    /// - `player_repo` -> Player database repository passed from the handler function
    async fn update_players_in_game(
        &self,
        game_data: &UpdateGameDTO,
        player_repo: &PlayerRepository,
        card_repo: &CardRepository
    ) -> Result<Vec<Player>, DatabaseQueryError<UpdateGameDTO>> {
        // just to make sure that the needed data was provided
        let new_players = match &game_data.players {
            None => {
                return Err(DatabaseQueryError {
                    message: "Function was called with invalid data passed to it! A new list of players is mandatory!".to_string(),
                    received_data: None,
                    status_code: StatusCode::INTERNAL_SERVER_ERROR
                });
            },
            Some(players) => {
                if players.len() == 0 {
                    return Err(DatabaseQueryError {
                        message: "An empty list of players was provided! That's an invalid data input!".to_string(),
                        received_data: None,
                        status_code: StatusCode::BAD_REQUEST
                    });
                }
                players
            }
        };

        // get all players first
        let all_current_players: Vec<Player> = match player_repo.get_all_players(Some(&game_data.id), card_repo).await {
            Ok(players) => players,
            Err(err) => {
                return Err(DatabaseQueryError::new(
                    err.message,
                    match err.received_data {
                        None => None,
                        Some(_) => Some(Json(game_data.clone())),
                    },
                    err.status_code,
                ))
            }
        };

        // -> leave all entities that haven't changed
        // delete all players that are not in the updated list
        for player in all_current_players.clone() {
            match new_players.iter().find(|&p| p.id == player.id) {
                None => {
                    // delete the player
                    match player_repo.delete_player(&player.id).await {
                        Ok(_) => continue,
                        Err(err) => return Err(DatabaseQueryError {
                            message: err.message,
                            received_data: match err.received_data {
                                None => None,
                                Some(_) => Some(Json(game_data.clone()))
                            },
                            status_code: err.status_code
                        })
                    };
                }
                Some(_) => continue
            }
        }

        // add new entries
        for player in new_players {
            match all_current_players.iter().find(|&p| p.id == player.id) {
                None => {
                    match player_repo.add_player(player.clone()).await {
                        Ok(_) => continue,
                        Err(err) => return Err(DatabaseQueryError {
                            message: err.message,
                            received_data: match err.received_data {
                                None => None,
                                Some(_) => Some(Json(game_data.clone()))
                            },
                            status_code: err.status_code
                        })
                    }
                }
                Some(_) => continue
            }
        }


        // return modified list of players
        Ok(all_current_players)
    }

    /// Updates the claims of a game based on the provided `UpdateGameDTO`.
    ///
    /// # Arguments
    ///
    /// - `game_data` -> DTO object containing the list of claims
    /// - `claims_repo` -> Claim database repository passed from the handler function
    /// - `card_repo` -> Card database repository passed from the handler function
    ///
    /// # Returns
    ///
    /// - A vector of `Claim` if a new claim was added or when all claims were deleted.
    ///
    /// # Errors
    ///
    /// - Returns a `DatabaseQueryError` if the `claims` field in `game_data` is `None`.
    /// - Returns a `DatabaseQueryError` if there is an error while deleting or adding claims
    async fn update_claims_of_game(&self, game_data: &UpdateGameDTO, claims_repo: &ClaimsRepository, card_repo: &CardRepository) -> Result<Vec<Claim>, DatabaseQueryError<UpdateGameDTO>> {
        // first check if the needed data was provided
        if let None = &game_data.claims {
            return Err(DatabaseQueryError {
                message: "Function was called with invalid data passed to it! A new list of claims is mandatory!".to_string(),
                received_data: None,
                status_code: StatusCode::INTERNAL_SERVER_ERROR
            });
        }

        // when the array is empty, all claims in the list of the game will be deleted.
        if game_data.claims.iter().len() == 0 {
            // delete all claims of the game
            match claims_repo.delete_all_claims_of_game(&game_data.id).await {
                Ok(_) => {},
                Err(err) => return Err(DatabaseQueryError {
                    message: err.message,
                    received_data: match err.received_data {
                        None => None,
                        Some(_) => Some(Json(game_data.clone()))
                    },
                    status_code: err.status_code
                })
            };
        } else {
            // when there is one element, it will be added to the claims list of a game.
            // add the claim to the database
            match claims_repo.create_claim(game_data.claims.clone().unwrap()[1].clone(), card_repo).await {
                Ok(_) => {},
                Err(err) => return Err(DatabaseQueryError {
                    message: err.message,
                    received_data: match err.received_data {
                        None => None,
                        Some(_) => Some(Json(game_data.clone()))
                    },
                    status_code: err.status_code
                })
            };
        }

        Ok(match claims_repo.get_all_claims(Some(&game_data.id), None, card_repo).await {
            Ok(claims) => claims,
            Err(err) => return Err(DatabaseQueryError {
                message: err.message,
                received_data: match err.received_data {
                    None => None,
                    Some(_) => Some(Json(game_data.clone()))
                },
                status_code: err.status_code
            })
        })
    }

    /// This a ***`modification`*** method which modifies the `Chat` instance of a `Game`.
    ///
    /// `ChatMessage` entries which should be deleted or added are determined separatly.
    ///
    /// # Arguments
    ///
    /// - `game_data` => Reference to necessary the `UpdateGameDTO` data with a new `Chat` object.
    /// - `chat_repo` => The repository for a `Chat` struct.
    /// - `chat_message_repo` => The repository for a `ChatMessage`.
    ///
    /// # Returns
    ///
    /// 1.) Ok(Chat), WHEN the data could be updated without an incident.
    /// 2.) Err(Box<dyn ApplicationError>), WHEN an error occurs.
    pub async fn update_chat_of_game(&self, game_data: &UpdateGameDTO, chat_repo: &ChatRepository, chat_message_repo: &ChatMessageRepository) -> Result<Chat, Box<dyn ApplicationError>> {
        // first check if the needed data was provided
        if let None = &game_data.chat {
            return Err(Box::new(DatabaseQueryError::<Chat> {
                message: "Function was called with invalid data passed to it! A new chat object is mandatory!".to_string(),
                received_data: None,
                status_code: StatusCode::INTERNAL_SERVER_ERROR
            }));
        } else {
            let chat = game_data.chat.as_ref().unwrap();
            // when the number of the updated messages is 0 then immediately remove all messages
            if chat.number_of_messages == 0 {
                match chat_message_repo.delete_all_messages_in_chat(&chat.id).await {
                    Ok(_) => match chat_repo.update_number_of_messages_of_chat(0, Some(&chat.id), None).await {
                        Ok(_) => return Ok(chat.clone()),
                        Err(err) => return Err(err)
                    },
                    Err(err) => return Err(err)
                }
            }

            // delete all messages that aren't in the new message list and add these that were
            // added to the queue
            let all_current_messages = match chat_message_repo.get_all_messages_in_chat(&chat.id).await {
                Ok(messages) => messages,
                Err(err) => return Err(err)
            };

            // retrieve all ids of the messages that can be removed
            let mut id_list_of_removed_messages: Vec<&str> = vec![];
            for cur_mes in &all_current_messages {
                let mut was_removed = true;
                if let Some(_) = &chat.messages.iter().find(|m| m.id == cur_mes.id) {
                    was_removed = false;
                }
                if was_removed {
                    id_list_of_removed_messages.push(&cur_mes.id);
                }
            }
            // also determine the new messages to be added
            let mut new_messages: Vec<&ChatMessage> = vec![];
            for passed_message in &chat.messages {
                let mut is_new_mes = true;
                for cur_mes in &all_current_messages {
                    if cur_mes.id == passed_message.id {
                        is_new_mes = false;
                    }
                }
                if is_new_mes {
                    new_messages.push(passed_message);
                }
            }

            // finally remove & add corresponding messages
            for id in id_list_of_removed_messages {
                match chat_message_repo.delete_message_by_id(id).await {
                    Ok(_) => (),
                    Err(err) => return Err(err)
                }
            }

            for message in new_messages {
                match chat_message_repo.save_message(message).await {
                    Ok(_) => (),
                    Err(err) => return Err(err)
                }
            }

            Ok(chat.clone())
        }
    }
}
    }
}
