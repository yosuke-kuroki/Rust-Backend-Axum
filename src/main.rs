use axum::{
    Json, Router,
    extract::State,
    http::{Method, StatusCode},
    routing::{get, post},
    serve,
};
use serde::{Deserialize, Serialize};

fn main() {
    println!("Hello, world!");
}
