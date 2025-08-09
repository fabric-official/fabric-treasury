
use axum::{routing::get, Router, response::IntoResponse, extract::Path};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/healthz", get(health))
        .route("/", get(root))
        .route("/wallets/:id/balances", get(balances))
        .route("/royalties", get(royalties))
        .route("/claims", get(claims));

    let addr = SocketAddr::from(([0,0,0,0], 8080));
    println!("listening on {}", addr);
    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();
}

async fn health() -> impl IntoResponse { axum::Json(serde_json::json!({"status":"ok"})) }
async fn root() -> impl IntoResponse { axum::Json(serde_json::json!({"service":"treasury"})) }
async fn balances(Path(id): Path<String>) -> impl IntoResponse { axum::Json(serde_json::json!({"wallet":id,"balances":[{"asset":"FAB","amount":1234.56}]})) }
async fn royalties() -> impl IntoResponse { axum::Json(vec![serde_json::json!({"agent":"edge-vision-v2","amount":12.34})]) }
async fn claims() -> impl IntoResponse { axum::Json(vec![] ) }
