use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use std::net::SocketAddr;
use sub_provider::{
    config::Config,
    provider::{clash::Clash, Provider},
    proxy::Proxy,
};

#[tokio::main]
async fn main() {
    // build our application with a route
    let provider = Router::new()
        .route("/clash", get(clash))
        .route("/clash-meta", get(clash_meta));

    // read the path prefix environment variable
    let path_prefix = std::env::var("PATH_PREFIX").unwrap_or("/".to_string());
    let app = Router::new()
        .route("/", get(handler))
        .nest(&path_prefix, provider);

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
        .into_iter()
        .map(|(key, items)| {
            let proxies: Vec<Proxy> = items
                .into_iter()
                .filter_map(|item| Proxy::try_from(item).ok())
                .collect();
            (key, proxies)
        })
        .collect();

    let clash = Clash::new().with_proxies(proxies);

    clash.provide()
}

async fn clash_meta() -> impl IntoResponse {
    let config_path = std::env::var("CONFIG_PATH").unwrap_or("config.toml".to_string());
    let cfg = Config::from_file(&config_path).unwrap();

    let proxies = cfg
        .groups
        .into_iter()
        .map(|(key, items)| {
            let proxies: Vec<Proxy> = items
                .into_iter()
                .filter_map(|item| Proxy::try_from(item).ok())
                .collect();
            (key, proxies)
        })
        .collect();

    let clash = Clash::new().with_proxies(proxies);

    clash.provide()
}
