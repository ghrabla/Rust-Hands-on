use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};
use tracing::info;

type Db = Arc<Mutex<HashMap<String, Job>>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Job {
    id: String,
    name: String,
    status: String,
}

#[derive(Debug, Deserialize)]
struct CreateJobRequest {
    name: String,
}

#[derive(Clone)]
struct AppState {
    db: Db,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        db: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/jobs", post(create_job))
        .route("/jobs/:id", get(get_job))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(serde_json::json!({ "ok": true })))
}

async fn create_job(
    State(state): State<AppState>,
    Json(payload): Json<CreateJobRequest>,
) -> impl IntoResponse {
    let id = format!("job-{}", uuid_like());

    let job = Job {
        id: id.clone(),
        name: payload.name,
        status: "queued".to_string(),
    };

    state.db.lock().unwrap().insert(id.clone(), job.clone());

    info!(job_id = %id, "job created");

    (StatusCode::CREATED, Json(job))
}

async fn get_job(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let db = state.db.lock().unwrap();

    if let Some(job) = db.get(&id) {
        (StatusCode::OK, Json(job.clone())).into_response()
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "job not found" })),
        )
            .into_response()
    }
}

fn uuid_like() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    ms.to_string()
}
