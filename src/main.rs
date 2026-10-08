use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    response::{Html, Json},
    routing::{get, post},
};
use serde::Serialize;
use serde_json::{Value, json};
use uuid::Uuid;

#[derive(Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum JobStatus {
    Queued,
    Processing,
    Completed,
    Failed,
}

#[derive(Clone, Copy, Serialize)]
struct Job {
    id: Uuid,
    status: JobStatus,
}

type AppState = Arc<Mutex<HashMap<Uuid, Job>>>;

#[tokio::main]
async fn main() {
    let jobs: AppState = Arc::new(Mutex::new(HashMap::new()));
    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health_handler))
        .route("/jobs", post(create_jobs))
        .route("/jobs/{id}", get(get_single_job))
        .with_state(jobs);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}

async fn health_handler() -> Json<Value> {
    Json(json!({"message":"ok"}))
}

async fn create_jobs(State(state): State<AppState>) -> (StatusCode, Json<Job>) {
    let id = Uuid::new_v4();
    let new_job = Job {
        id,
        status: JobStatus::Queued,
    };
    state.lock().unwrap().insert(id, new_job);

    (StatusCode::ACCEPTED, Json(new_job))
}

async fn get_single_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Job>, StatusCode> {
    let jobs = state.lock().unwrap();
    match jobs.get(&id) {
        Some(job) => Ok(Json(*job)),
        None => Err(StatusCode::NOT_FOUND),
    }
}
