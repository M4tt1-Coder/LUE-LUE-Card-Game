// TODO: Implement the 'Game' endpoint functions

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {

use worker::Env;
use axum::{
    extract::Path,
    http::{self, StatusCode},
    Json,
    Extension
};
use leptos_axum::extract;
use uuid::Uuid;
use crate::backend::{types::game::Game, repositories::game_repository::GameRepository};
use std::sync::Arc;

// TODO: First try to implement the 'post_game' function -> access the database and insert a new
// game instance

// TODO: Try leptos server function -> register it in the router provider

pub async fn post_game(Json(body_data): Json<Game>, worker_env: Extension<Arc<Env>>) -> Result<Json<Game>, StatusCode> {
    // extract a new 'Game' object from the payload

    // get a database instance
    let db = match worker_env.d1("DB") {
        Ok(database) => database,
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR)
    };

    // execute the insertion
    let game_repo = GameRepository::new(db);
    match game_repo.add_game(body_data).await {
        Ok(inserted_game) => Ok(Json(inserted_game)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

// #[server]
// pub async fn post_game() -> Result<Game, ServerFnError> {


    }
}
