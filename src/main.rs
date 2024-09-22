use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use sub_provider::{
    config::Config,
    provider::{clash::Clash, Provider},
    proxy::from_raw_proxies,
};

#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/clash", get(clash))
        .route("/clash-meta", get(clash_meta));

    // read the PORT environment variable
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3000);

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let lisenter = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(
        lisenter,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn clash() -> impl IntoResponse {
    let config_path = std::env::var("CONFIG_PATH").unwrap_or("config.toml".to_string());
    let cfg = Config::from_file(&config_path).unwrap();

    let proxies = cfg
        .groups
        .iter()
        .map(|(key, item)| (key.clone(), from_raw_proxies(item.to_vec())))
        .collect();

    let clash = Clash::new().with_proxies(proxies);

    clash.provide()
}

async fn clash_meta() -> impl IntoResponse {
    let config_path = std::env::var("CONFIG_PATH").unwrap_or("config.toml".to_string());
    let cfg = Config::from_file(&config_path).unwrap();

    let proxies = cfg
        .groups
        .iter()
        .map(|(key, item)| (key.clone(), from_raw_proxies(item.to_vec())))
        .collect();

    let clash = Clash::new().with_proxies(proxies);

    clash.provide()
}
