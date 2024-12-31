use crate::models::TeamMember;
use axum::{extract::Path, http::StatusCode, Extension, Json};
use serde_json::json;
use sqlx::PgPool;

pub async fn get_team_members(
    Extension(pool): Extension<PgPool>,
) -> Result<Json<Vec<TeamMember>>, StatusCode> {
    let members = sqlx::query_as::<_, TeamMember>("SELECT * FROM team_member")
        .fetch_all(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(members))
}

pub async fn get_team_member(
    id: i32,
    Extension(pool): Extension<PgPool>,
) -> Result<TeamMember, StatusCode> {
    let member = sqlx::query_as::<_, TeamMember>("SELECT * FROM team_member where id=$1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(member)
}
pub async fn create_team_member(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<TeamMember>,
) -> Result<Json<TeamMember>, (StatusCode, Json<serde_json::Value>)> {
    if payload.work_hours_per_day > 12.0 || payload.work_hours_per_day < 0.0 {
        println!("Work hours per day have to be less than 12 and more than 0");
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "work_hours_per_day must be between 1.0 and 12.0" })),
        ));
    }
    let team_member = sqlx::query_as::<_, TeamMember>(
        "INSERT INTO team_member (name, work_hours_per_day)
         VALUES ($1, $2)
         RETURNING id, name, work_hours_per_day",
    )
    .bind(payload.name)
    .bind(payload.work_hours_per_day)
    .fetch_one(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error" })),
        )
    })?;

    Ok(Json(team_member))
}

pub async fn update_team_member(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<TeamMember>,
) -> Result<Json<TeamMember>, StatusCode> {
    let updated_member = sqlx::query_as::<_, TeamMember>(
        "UPDATE team_member
         SET name = $2, work_hours_per_day = $3
         WHERE id = $1
         RETURNING *",
    )
    .bind(id)
    .bind(payload.name)
    .bind(payload.work_hours_per_day)
    .fetch_one(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(updated_member))
}

pub async fn delete_team_member(
    Path(id): Path<i32>,
    Extension(pool): Extension<PgPool>,
) -> Result<StatusCode, StatusCode> {
    sqlx::query("DELETE FROM team_member WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::NO_CONTENT)
}
