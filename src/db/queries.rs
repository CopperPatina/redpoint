use crate::climblib::models::{ClimbMetricsEntry, ClimbingSession, Grade, WorkoutSession};
use chrono::NaiveDate;
use sqlx::postgres::PgPool;
use uuid::Uuid;


pub async fn insert_climb_db(pool: &PgPool, session: ClimbingSession) -> Result<(), sqlx::Error> {
    let session_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO climbing_sessions (id, date, location, style, notes)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        session_id,
        NaiveDate::parse_from_str(&session.date, "%Y-%m-%d").unwrap(), //validated as date in ClimbingSession struct
        session.location,
        session.style.to_string(),
        session.notes
    )
    .execute(pool)
    .await?;

    for climb in session.climbs {
        let climb_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO climb_entries (id, session_id, name, grade, attempts, sent, reached_top, lead, rests)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            "#,
            climb_id,
            session_id,
            climb.name,
            match climb.grade {
                Grade::Boulder(b) => b.to_string(),
                Grade::Rope(r) => r.to_string(),
            },
            climb.attempts as i16,
            climb.sent,
            climb.reached_top,
            climb.lead,
            climb.rests.map(|r| r as i16)
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn insert_workout_db(pool: &PgPool, session: WorkoutSession) -> Result<(), sqlx::Error> {
    let session_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO workout_sessions (id, date, notes)
        VALUES ($1, $2, $3)
        "#,
        session_id,
        NaiveDate::parse_from_str(&session.date, "%Y-%m-%d").unwrap(), //validated as date in WorkoutSession struct
        session.notes
    )
    .execute(pool)
    .await?;

    for exercise in session.exercises {
        let exercise_id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO exercise_entries (id, workout_session_id, name, sets, reps, weight_lb, rpe, is_main_lift)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            exercise_id,
            session_id,
            exercise.name,
            exercise.sets as i16,
            exercise.reps as i16,
            exercise.weight_lb,
            exercise.rpe.map(|r| r as i16),
            exercise.is_main_lift
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}

pub async fn insert_metrics_db(pool: &PgPool, metrics: ClimbMetricsEntry) -> Result<(), sqlx::Error> {
    let metrics_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO climbing_metrics (id, date, finger_strength_percent_bw, max_pullup_percent_bw, notes)
        VALUES ($1, $2, $3, $4, $5)
        "#,
        metrics_id,
        NaiveDate::parse_from_str(&metrics.date, "%Y-%m-%d").unwrap(), //validated as date in ClimbMetricsEntry struct
        metrics.finger_strength_percent_bw,
        metrics.max_pullup_percent_bw,
        metrics.notes
    )
    .execute(pool)
    .await?;

    Ok(())
}
