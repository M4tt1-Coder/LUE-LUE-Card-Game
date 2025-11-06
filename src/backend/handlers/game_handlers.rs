// TODO: Implement the 'Game' endpoint functions

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {

use worker::{Env, console_error};
use axum::{
    extract::Path,
    http::{self, StatusCode},
    Json,
    Extension
};
use leptos_axum::extract;
use uuid::Uuid;
use crate::backend::{
    types::game::{ Game, UpdateGameDTO },
    repositories::{
        game_repository::GameRepository,
        chat::{
            chat_repository::ChatRepository,
            chat_message_repository::ChatMessageRepository
        },
        card_repository::CardRepository,
        claim_repository::ClaimsRepository,
        player_repository::PlayerRepository
    }
};
use std::sync::Arc;

#[axum::debug_handler]
pub async fn post_game(Extension(worker_env): Extension<Arc<Env>>) -> Result<Json<Game>, StatusCode> {
    // get the database instance
    let db = match worker_env.d1("DB") {
         Ok(database) => database,
         Err(err) => {
            console_error!("Failed to get database instance from worker environment! Error: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
         }
    };

    // a new 'Game' object
    let new_game = Game::default();

    // execute the insertion
    let game_repo = GameRepository::new(&db);
    match game_repo.add_game(new_game).await {
        Ok(inserted_game) => Ok(Json(inserted_game)),
        Err(err) => {
            console_error!("Failed to insert new game into the database! Error: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

// TODO: Implement the other game endpoint handlers (GET, PUT, DELETE)

#[axum::debug_handler]
pub async fn get_game(Extension(worker_env): Extension<Arc<Env>>, Path(game_id): Path<String>) -> Result<Json<Game>, StatusCode> {
    // get the database instance
    let db = match worker_env.d1("DB") {
        Ok(database) => database,
        Err(err) => {
            console_error!("Failed to get database instance from worker environment! Error: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // TODO: Adjust all remaining repositories methods to accept the "#[worker::send]" macro

    // get all necessary repositories
    let claims_repo = ClaimsRepository::new(&db);
    let card_repo = CardRepository::new(&db);
    let player_repo = PlayerRepository::new(&db);
    let chat_repo = ChatRepository::new(&db);
    let chat_message_repo = ChatMessageRepository::new(&db);

    // fetch the game by ID
    let game_repo = GameRepository::new(&db);
    match game_repo.get_game_by_id(&game_id, &chat_repo, &player_repo, &claims_repo, &chat_message_repo, &card_repo).await {
        Ok(game) => Ok(Json(game)),
        Err(err) => {
            console_error!("Failed to fetch game from the database! Error: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

    }
}
