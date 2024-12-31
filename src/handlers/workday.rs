use crate::handlers::team_member::get_team_member;
use crate::models::{WorkDay, WorkDayInput};
use axum::extract::Query;
use axum::{extract::Path, http::StatusCode, Extension, Json};
use chrono::NaiveDate;
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct DateRange {
    pub start_date: String,
    pub end_date: String,
}
pub async fn get_work_days(
    Query(params): Query<DateRange>,
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<WorkDay>>, StatusCode> {
    let start_date = NaiveDate::parse_from_str(&params.start_date, "%Y-%m-%d")
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let end_date = NaiveDate::parse_from_str(&params.end_date, "%Y-%m-%d")
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let members =
        sqlx::query_as::<_, WorkDay>("SELECT * FROM work_day where date between $1 and $2")
            .bind(start_date)
            .bind(end_date)
            .fetch_all(&pool)
            .await
            .map_err(|e| {
                println!("Database error: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    Ok(Json(members))
}

pub async fn create_work_day(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<WorkDayInput>,
) -> Result<Json<WorkDay>, (StatusCode, Json<serde_json::Value>)> {
    if let Err(status) = get_team_member(payload.member_id, Extension(pool.clone())).await {
        if status != StatusCode::OK {
            println!("member not found");
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": "member_id is not found" })),
            ));
        }
    }
    let date = NaiveDate::parse_from_str(&payload.date, "%Y-%m-%d").map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "Invalid date format, expected YYYY-MM-DD" })),
        )
    })?;

    let work_day = sqlx::query_as::<_, WorkDay>(
        "INSERT INTO work_day (member_id, date, utilisation)
         VALUES ($1, $2, $3)
         RETURNING id, member_id, date, utilisation",
    )
    .bind(payload.member_id)
    .bind(date)
    .bind(payload.utilisation)
    .fetch_one(&pool)
    .await
    .map_err(|e| {
        println!("Database error: {:?}", e); // Log the error
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error" })),
        )
    })?;

    Ok(Json(work_day))
}

pub async fn delete_work_day(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM work_day WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}
