use crate::climblib::models::{ClimbingSession, WorkoutSession, ClimbMetricsEntry};
use crate::climblib::io::{save_log};
use axum::{
    http::StatusCode,
    response::{IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use tracing::{info, error};
use tower_http::cors::{CorsLayer, Any};


async fn create_climb(Json(session): Json<ClimbingSession>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let filename = session.date.clone() + "-climb.json";
    match save_log(&session, &filename){
        Ok(_) => { info!("Saved {}", &filename);
        return Ok((StatusCode::CREATED, Json(json!({ "status": "ok" }))));
        } ,
        Err(e) => { error!("Error saving {} with {}", &filename, e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not save log".to_string()));
        },
    };
}

async fn create_workout(Json(session): Json<WorkoutSession>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let filename = session.date.clone() + "-workout.json";
    match save_log(&session, &filename){
        Ok(_) => { info!("Saved {}", &filename);
        return Ok((StatusCode::CREATED, Json(json!({ "status": "ok" }))));
        } ,
        Err(e) => { error!("Error saving {} with {}", &filename, e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not save log".to_string()));
        },
    };
}

async fn create_metrics(Json(session): Json<ClimbMetricsEntry>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let filename = session.date.clone() + "-metrics.json";
    match save_log(&session, &filename){
        Ok(_) => { info!("Saved {}", &filename);
        return Ok((StatusCode::CREATED, Json(json!({ "status": "ok" }))));
        } ,
        Err(e) => { error!("Error saving {} with {}", &filename, e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not save log".to_string()));
        },
    };
}

pub async fn start_server() {
    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([axum::http::Method::POST])
    .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
    .route("/api/logs/climb", post(create_climb))
    .route("/api/logs/workout", post(create_workout))
    .route("/api/logs/metrics", post(create_metrics));

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}