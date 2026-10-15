use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
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
use tokio::{sync::mpsc, time};
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

#[derive(Clone)]
struct AppState {
    jobs: Arc<Mutex<HashMap<Uuid, Job>>>,
    tx: mpsc::Sender<Uuid>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let (tx, rx) = mpsc::channel::<Uuid>(100);

    let jobs = Arc::new(Mutex::new(HashMap::new()));
    tokio::spawn(worker(rx, jobs.clone()));

    let state = AppState { jobs, tx };

    let app = Router::new()
        .route("/", get(handler))
        .route("/health", get(health_handler))
        .route("/jobs", post(create_jobs))
        .route("/jobs/{id}", get(get_single_job))
        .with_state(state);

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
    tracing::info!(job_id = %id, "job queued");
    state.jobs.lock().unwrap().insert(id, new_job);
    state.tx.send(id).await.unwrap();

    (StatusCode::ACCEPTED, Json(new_job))
}

async fn get_single_job(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Job>, StatusCode> {
    let jobs = state.jobs.lock().unwrap();
    match jobs.get(&id) {
        Some(job) => Ok(Json(*job)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn worker(mut rx: mpsc::Receiver<Uuid>, jobs: Arc<Mutex<HashMap<Uuid, Job>>>) {
    while let Some(id) = rx.recv().await {
        tracing::info!(job_id = %id, "worker received job");
        {
            let mut jobs_map = jobs.lock().unwrap();
            match jobs_map.get_mut(&id) {
                Some(job) => {
                    job.status = JobStatus::Processing;
                    tracing::info!(job_id = %id, "worker is processing job");
                }
                None => {
                    tracing::error!(job_id = %id, "job not found");
                    continue;
                }
            }
        }
        time::sleep(Duration::from_secs(30)).await;
        {
            let mut jobs_map = jobs.lock().unwrap();
            match jobs_map.get_mut(&id) {
                Some(job) => {
                    job.status = JobStatus::Completed;
                    tracing::info!(job_id = %id, "job completed");
                }
                None => {
                    tracing::error!(job_id = %id, "job not found");
                    continue;
                }
            }
        }
    }
}
