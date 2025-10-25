// the server endpoints can only tested in ssr mode
use cfg_if::cfg_if;
cfg_if! {
    if #[cfg(feature = "ssr")] {
        // TODO: Mock database interactions

        // define the testing server
        // const server = crate::backend::router::router_provider::router();

    }
}
