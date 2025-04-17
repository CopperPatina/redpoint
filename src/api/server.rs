use crate::climblib::models::{ClimbingSession, WorkoutSession, ClimbMetricsEntry};
use crate::climblib::io::{save_log};
use axum::{
    http::StatusCode,
    response::{IntoResponse},
    routing::{get, post},
    Json, Router,
};
use tracing::{info, error};


async fn create_climb(Json(session): Json<ClimbingSession>) -> impl IntoResponse {
    let filename = session.date.clone() + "-climb.json";
    match save_log(&session, &filename){
        Ok(_) => info!("Saved {}", &filename),
        Err(e) => error!("Error saving {} with {}", &filename, e),
    };
}

async fn create_workout(Json(session): Json<WorkoutSession>) -> impl IntoResponse {
    let filename = session.date.clone() + "-workout.json";
    match save_log(&session, &filename){
        Ok(_) => info!("Saved {}", &filename),
        Err(e) => error!("Error saving {} with {}", &filename, e),
    };
}

async fn create_metrics(Json(session): Json<ClimbMetricsEntry>) -> impl IntoResponse {
    let filename = session.date.clone() + "-metrics.json";
    match save_log(&session, &filename){
        Ok(_) => info!("Saved {}", &filename),
        Err(e) => error!("Error saving {} with {}", &filename, e),
    };
}

pub async fn start_server() {
    let app = Router::new()
    .route("/api/logs/climb", post(create_climb))
    .route("/api/logs/workout", post(create_workout))
    .route("/api/logs/metrics", post(create_metrics));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}