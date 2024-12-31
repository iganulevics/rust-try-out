use crate::handlers::team_member;
use crate::handlers::workday;
use axum::routing::delete;
use axum::{
    routing::{get, put},
    Router,
};

pub fn app_routes(pool: sqlx::PgPool) -> Router {
    Router::new()
        .route(
            "/team_members",
            get(team_member::get_team_members).post(team_member::create_team_member),
        )
        .route(
            "/team_members/{id}",
            put(team_member::update_team_member).delete(team_member::delete_team_member),
        )
        .route(
            "/work_days",
            get(workday::get_work_days).post(workday::create_work_day),
        )
        .route("/work_days/{id}", delete(workday::delete_work_day))
        .layer(axum::Extension(pool))
}
