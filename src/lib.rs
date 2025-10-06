// TODO: Read the book for leptos -> https://book.leptos.dev/
// TODO: Watch Axum tutoial -> https://www.youtube.com/watch?v=XZtlD_m59sM

// https://github.com/cloudflare/workers-rs/tree/main/templates/leptos
// https://github.com/bakcxoj/leptos-workers
// https://github.com/DylanRJohnston/leptos-cloudflare-example

mod app;
mod backend;
mod ui;

use leptos::*;

#[cfg(feature = "ssr")]
use worker::*;

#[event(fetch)]
#[cfg(feature = "ssr")]
async fn fetch(
    req: HttpRequest,
    env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    use crate::app::App;
    use crate::backend::router::router_provider;
    use leptos::config::LeptosOptions;
    use log::debug;
    use std::env;
    use std::sync::Arc;
    use tower_service::Service;
    // use worker::*;

    // TODO: Try to important all options from .env
    // Define the leptos options
    let leptos_options_builder = LeptosOptions::builder();

    //    output_name: env!("CARGO_CRATE_NAME").into(),
    //    site_root: "target/site".into(),
    //    site_pkg_dir: "pkg".into(),
    //    env: (&env::var("LEPTOS_ENV")).into(),
    //    site_addr: "127.0.0.1:3000".parse().unwrap(),
    //    reload_port: 3001,
    //    reload_external_port: None,
    //    reload_ws_protocol: ReloadWSProtocol::WS,
    //    not_found_path: "target/site/404.html".into(),
    //    hash_file: "hash.txt".into(),
    //    hash_files: false,
    //    server_fn_prefix: None,
    //    disable_server_fn_hash: false,
    //    server_fn_mod_path: false,

    // register leptos server functions
    // TODO: Register leptos functions later

    // Get the database binding -> access to D1 database
    // let database = env.d1("DB").map_err(|err| {
    //     warn!("{err}");
    //     worker::Error::RustError("DB binding not found".to_string())
    // })?;

    debug!("Server is running on port http://localhost:3000/");
    Ok(router_provider::router(
        env,
        leptos_options_builder
            .output_name(*Arc::new("lue_lue_game"))
            .build(),
    )
    .await
    .call(req)
    .await?)
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    mount::mount_to_body(|| view! { <App/> });
}
