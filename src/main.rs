use std::{borrow::BorrowMut, io::{Read, Write}, net::{TcpListener, TcpStream}};
use std::sync::Arc;
use queue::Queue;
use serde::{Deserialize, Serialize};
use axum::extract::{Path, Query, Json};
use axum::{
    routing::{get, post},
    Router,
};
use axum::extract::State;

mod queue;

async fn enqueue(Json(payload): Json<QueueRequest>) {
    println!("{},{}",payload.action, payload.id)
}

async fn dequeue(Json(payload): Json<QueueRequest>) {
    println!("{},{}",payload.action, payload.id)
}

async fn handler(
    State(state): State<Arc<Queue>>,
) {
   
}

#[derive(Serialize, Deserialize)]
pub struct QueueRequest {
    action: String,
    id: String
}

#[tokio::main]
async fn main() {
    let state = Arc::new(queue::Queue::new());

    let app = Router::new()
    .route("/enqueue", post(enqueue))
    .route("/dequeue", post(dequeue))
    .route("/", get(|| async { "Hello, World!" }))
    .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}