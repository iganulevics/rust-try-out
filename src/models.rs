use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct TeamMember {
    pub id: Option<i32>,
    pub name: String,
    pub work_hours_per_day: f64,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkDay {
    pub id: Option<i32>,
    pub member_id: i32,
    pub date: NaiveDate,
    pub utilisation: i32,
}

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct WorkDayInput {
    pub id: Option<i32>,
    pub member_id: i32,
    pub date: String,
    pub utilisation: i32,
}
