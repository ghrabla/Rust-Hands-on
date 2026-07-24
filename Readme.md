# Distributed Task Runner

## 1) CLI (Rust)

- Submit jobs
- Check status
- Stream logs
- Cancel/retry

**Tech:** `clap`, `reqwest`, `tokio`, `tracing`

## 2) Desktop (Rust + Tauri)

- Visual dashboard for jobs/workers/queues
- Real-time updates
- Filters
- Retry buttons

## 3) Web Backend (Rust)

- API + scheduler + worker pool
- Retries
- Backoff
- Dead-letter queue
- Metrics

**Tech:** `axum`, `sqlx`, `redis`/`nats`, `tokio`, `prometheus`
