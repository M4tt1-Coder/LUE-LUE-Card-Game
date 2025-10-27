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
use crate::backend::{types::game::{Game, UpdateGameDTO}, repositories::game_repository::GameRepository};
use std::sync::Arc;

// TODO: First try to implement the 'post_game' function -> access the database and insert a new
// game instance

// TODO: Add a functional error handling mechanism instead of just returning StatusCode

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
    let game_repo = GameRepository::new(db);
    match game_repo.add_game(new_game).await {
        Ok(inserted_game) => Ok(Json(inserted_game)),
        Err(err) => {
            console_error!("Failed to insert new game into the database! Error: {:?}", err);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

    }
}
