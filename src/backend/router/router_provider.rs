use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {


    // use statements

    use leptos::*;

    use axum::routing::{put, post};
    use axum::Router;
    use axum::Extension;
    use leptos_axum::{generate_route_list, LeptosRoutes};

    use crate::backend::handlers::game_handlers::update_game;
    use crate::app::*;
    use worker::Env;
    use leptos::prelude::LeptosOptions;

    /// Application state for the Axum application.
    ///
    /// This module defines the application state that will be shared across the Axum application.
    ///
    /// # Properties
    ///
    /// - `leptos_options` -> Redefined `Leptos` options
    #[derive(Clone)]
    pub struct AppState {
        /// The `LeptosOptions` that need to be declared before running the server.
        pub leptos_options: LeptosOptions
    }

    /// Router provider for the Axum application.
    ///
    /// This module defines the router for the Axum application, setting up the routes
    ///
    /// # Arguments
    ///
    /// - `env` -> Cloudflare Worker environment
    /// - `leptos_options` -> Redefined `Leptos` options
    pub async fn router(env: Env, leptos_options: LeptosOptions) -> Router {
        use std::sync::Arc;

         // retrieve all leptos routes
        let routes = generate_route_list(|| view! { <App />});


        Router::new()
        // Register all necessary endpoints
        // game instance endpoints
        .route("/api/game/update", put(update_game))
        .leptos_routes(&leptos_options, routes,{
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        // .fallback()  TODO: Add a fallback handler / page
        .with_state(leptos_options)
        .layer(Extension(Arc::new(env)))
    }

}}
