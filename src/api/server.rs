use crate::climblib::models::{ClimbingSession, WorkoutSession, ClimbMetricsEntry};
use crate::climblib::io::{save_log, log_index};
use crate::db::queries::{insert_climb_db, insert_workout_db, insert_metrics_db};
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde_json::json;
use sqlx::postgres::{PgPool, PgPoolOptions};
use tracing::{info, error};
use tower_http::cors::{CorsLayer, Any};

async fn create_climb(Json(session): Json<ClimbingSession>) -> Result<impl IntoResponse, (StatusCode, String)> {
    println!("{:?}", session);
    let filename = "climb-".to_owned() + &session.date + ".json";
    match save_log(&session, &filename){
        Ok(_) => { info!("Saved {}", &filename);
        return Ok((StatusCode::CREATED, Json(json!({ "status": "ok" }))));
        } ,
        Err(e) => { error!("Error saving {} with {}", &filename, e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not save log".to_string()));
        },
    };
}

async fn get_climb() -> Result<impl IntoResponse, (StatusCode, Json<Vec<String>>)> {
    match log_index() {
        Ok(paths) => { info!("got paths");
        let string_paths: Vec<String> = paths.into_iter()
        .filter_map(|p| p.to_str().map(|s| s.to_string()))
        .collect();
        return Ok((StatusCode::OK, Json(string_paths)));
        } ,
        Err(e) => { error!("Error finding logs with {}", e);
        return Err((StatusCode::NOT_FOUND, Json(vec![] as Vec<String>)));
        },
    };
}

async fn create_workout(Json(session): Json<WorkoutSession>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let filename = "workout-".to_owned() + &session.date + ".json";
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
    let filename = "metrics-".to_owned() + &session.date + ".json";
    match save_log(&session, &filename){
        Ok(_) => { info!("Saved {}", &filename);
        return Ok((StatusCode::CREATED, Json(json!({ "status": "ok" }))));
        } ,
        Err(e) => { error!("Error saving {} with {}", &filename, e);
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Could not save log".to_string()));
        },
    };
}

pub async fn create_climb_db_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<ClimbingSession>,
) -> (StatusCode, String) {
    if let Err(e) = insert_climb_db(&pool, payload).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to insert climb: {}", e));
    }
    (StatusCode::OK, "Climb session inserted into database".to_string())
}

pub async fn create_workout_db_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<WorkoutSession>,
) -> (StatusCode, String) {
    if let Err(e) = insert_workout_db(&pool, payload).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to insert workout: {}", e));
    }
    (StatusCode::OK, "Workout session inserted into database".to_string())
}

pub async fn create_metrics_db_handler(
    State(pool): State<PgPool>,
    Json(payload): Json<ClimbMetricsEntry>,
) -> (StatusCode, String) {
    if let Err(e) = insert_metrics_db(&pool, payload).await {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to insert metrics: {}", e));
    }
    (StatusCode::OK, "Climb metrics inserted into database".to_string())
}

pub async fn start_server(db_connection_str: &str) {
    let pool = PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(tokio::time::Duration::from_secs(3))
    .connect(&db_connection_str)
    .await
    .expect("can't connect to database");

    if let Err(e) = sqlx::migrate!().run(&pool).await {
        panic!("Failed to run migrations: {:?}", e);
    }

    let cors = CorsLayer::new()
    .allow_origin(Any)
    .allow_methods([axum::http::Method::POST])
    .allow_methods([axum::http::Method::GET])
    .allow_headers([axum::http::header::CONTENT_TYPE]);

    let app = Router::new()
    .route("/api/logs/climb", post(create_climb))
    .route("/api/logs/workout", post(create_workout))
    .route("/api/logs/metrics", post(create_metrics))
    .route("/api/logs/get-climb", get(get_climb))
    .route("/api/db/climb", post(create_climb_db_handler))
    .route("/api/db/workout", post(create_workout_db_handler))
    .route("/api/db/metrics", post(create_metrics_db_handler))
    .layer(cors)
    .with_state(pool.clone());

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
    .serve(app.into_make_service())
    .await
    .unwrap();
}