use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let shared_state = AppState {
        population: Arc::new(Mutex::new(500)),
    };

    let cors = CorsLayer::new().allow_methods(Any).allow_origin(Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/add", get(add))
        .route("/sub", get(sub))
        .layer(cors)
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug, Clone)]
struct AppState {
    population: Arc<Mutex<i32>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PopulationAnnouncement<'a> {
    population: i32,
    announcement: &'a str,
}

#[axum::debug_handler]
async fn root(State(state): State<AppState>) -> impl IntoResponse {
    let data = state.population.lock().expect("mutex was poisoned");

    (StatusCode::OK, Json(*data))
}

#[axum::debug_handler]
async fn add(State(state): State<AppState>) -> impl IntoResponse {
    println!("adding here");
    let mut data = state.population.lock().expect("mutex was poisoned");
    *data += 1;

    let popreturn = PopulationAnnouncement {
        population: *data,
        announcement: "Little timmy was born today.",
    };

    (StatusCode::OK, Json(popreturn))
}

#[axum::debug_handler]
async fn sub(State(state): State<AppState>) -> impl IntoResponse {
    println!("subbing here");
    let mut data = state.population.lock().expect("mutex was poisoned");
    *data -= 1;

    let popreturn = PopulationAnnouncement {
        population: *data,
        announcement: "Little timmy died in the well.",
    };

    (StatusCode::OK, Json(popreturn))
}
