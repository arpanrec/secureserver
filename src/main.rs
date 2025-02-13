use axum::extract::State;
use axum::{
    body::Body,
    extract,
    extract::Path,
    http,
    http::{Response, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::get,
    Router,
};
use secretsquirrel::physical::{delete, read, write};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone, Debug)]
struct MainState {
    data: Arc<RwLock<HashMap<String, u32>>>,
}
#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let shared_state = Arc::new(MainState {
        data: Arc::new(RwLock::new(HashMap::new())),
    });
    let app = Router::new()
        .route(
            "/secret/{*key}",
            get(handle_get).post(handle_post).delete(handle_delete),
        )
        .route("/{*key}", axum::routing::any(handle_any))
        .layer(middleware::from_fn_with_state(
            shared_state.clone(),
            print_request_response,
        ))
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_get(
    Path(path): Path<String>,
    State(state): State<Arc<MainState>>,
) -> Response<String> {
    let map = state.data.read().await;
    let counter = map.get("counter").unwrap();
    println!("Counter in get: {}", counter);
    drop(map);
    println!("Received request for key: {}", path);
    let value = read(&path).await;
    println!("Read value: {:?}", value);
    let builder = Response::builder();
    if let Some(value) = value {
        println!("Found value: {}", value);
        builder
            .status(StatusCode::OK)
            .header("Content-Type", "text/plain")
            .body(value)
            .expect("Failed to send response")
    } else {
        println!("Not Found");
        builder
            .status(StatusCode::NOT_FOUND)
            .header("Content-Type", "text/plain")
            .body("".to_string())
            .expect("Failed to send response")
    }
}

async fn handle_post(Path(path): Path<String>, body: String) -> Response<String> {
    println!("Received request for key: {}", path);
    println!("Received body: {}", body);
    write(&path, &body).await;
    Response::new("".to_string())
}

async fn handle_delete(Path(path): Path<String>) -> Response<String> {
    println!("Received request for key: {}", path);
    delete(&path).await;
    Response::new("".to_string())
}

async fn handle_any(Path(path): Path<String>, request: http::Request<Body>) -> Response<String> {
    println!(
        "Received request for lock with method: {:?}",
        request.method()
    );
    println!("Received request for key: {}", path);
    Response::new("".to_string())
}

async fn print_request_response(
    State(state): State<Arc<MainState>>,
    request: extract::Request,
    next: Next,
) -> impl IntoResponse {
    let mut data = state.data.write().await;
    let counter = data.entry("counter".to_string()).or_insert(0);
    *counter += 1;
    let is_counter_even = *counter % 2 == 0;
    drop(data);
    if is_counter_even {
        Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .header("Content-Type", "text/plain")
            .body("Unauthorized".into())
            .unwrap()
    } else {
        next.run(request).await
    }
}
