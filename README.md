# rust-queue-worker

A production-oriented asynchronous job processing service built with Rust, Axum, Tokio, Postgres, and background workers.

> **Status: early development.** The API and worker are being built in the open, one vertical slice at a time. See the roadmap for what exists today and what is planned.

## Why this project exists

Many services must accept work quickly and finish it later. Examples are image conversion, report generation, and webhook delivery. This project is a small, honest implementation of that pattern. It focuses on the parts that make such a system reliable: durable job state, retries, failure handling, and observability.

## Architecture

```text
Client
  |
  v
Axum API  ------>  PostgreSQL (jobs table)
  |                      ^
  v                      |
Queue / dispatch         |
  |                      |
  +--------+--------+    |
  v        v        v    |
Worker   Worker   Worker-+
  |
  v
Job handler (first target: image processing)
```

- The API accepts a job, stores it, and returns a job ID at once.
- Workers take queued jobs and run them asynchronously.
- Every state change is saved to Postgres, so a restart does not lose work.

## Job lifecycle

```text
queued -> processing -> completed
                    \-> failed (retry with backoff, then stop)
```

## Planned API

| Method | Path | Purpose |
|---|---|---|
| GET | `/health` | Liveness check |
| POST | `/jobs` | Submit a job, receive a job ID |
| GET | `/jobs/{id}` | Read job status and result |

## Tech stack

- Rust
- Axum (HTTP API)
- Tokio (async runtime, tasks, channels)
- SQLx and PostgreSQL (persistence and migrations)
- tracing (structured logs)
- Docker

## Roadmap

- [ ] `GET /health` with Axum
- [ ] `POST /jobs` and `GET /jobs/{id}` with in-memory state
- [ ] Worker task fed by a Tokio channel
- [ ] PostgreSQL persistence with SQLx migrations
- [ ] Job state transitions saved to the database
- [ ] Failure state and error storage
- [ ] Retries with exponential backoff
- [ ] Idempotent job submission
- [ ] Multiple workers with concurrency limits
- [ ] Structured logging and request IDs with `tracing`
- [ ] Graceful shutdown
- [ ] Docker Compose setup for API and database
- [ ] Integration tests
- [ ] First real job type: image conversion

## Local setup

Setup steps will be added when the API runs. The target flow is:

```bash
# Start Postgres
docker compose up -d db

# Run migrations
# (command added with the SQLx step)

# Run the API
cargo run
```

## Design notes

Decisions and lessons learned will be added here as the project grows, including trade-offs in queue design, retry behavior, and worker concurrency.

## License

To be decided.
