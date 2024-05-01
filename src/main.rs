use std::sync::{Arc, Mutex};
use queue::Queue;
use serde::{Deserialize, Serialize};
use axum::extract::Json;
use axum::routing::{Router, post, get};
use axum::extract::State;

mod queue;

async fn enqueue_handler(State(state): State<Arc<Mutex<Queue>>>,Json(payload): Json<QueueRequest>) {
    let taco = state.clone();
    let mut state = taco.lock().unwrap();
    state.enqueue(5);
}

async fn dequeue_handler(State(state): State<Arc<Mutex<Queue>>>,Json(payload): Json<QueueRequest>) {
let taco = state.clone();
    let mut state = taco.lock().unwrap();
    state.dequeue();
}

async fn view_handler(State(state): State<Arc<Mutex<Queue>>>){
    let taco = state.clone();
    let state = taco.lock().unwrap().clone();

    for item in state.arr {
        println!("{}",item);
    }
}


#[derive(Serialize, Deserialize)]
pub struct QueueRequest {
    action: String,
    id: String
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(queue::Queue::new()));

    let app = Router::new()
    .route("/enqueue", post(enqueue_handler))
    .route("/dequeue", post(dequeue_handler))
    .route("/", get(view_handler))
    .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}