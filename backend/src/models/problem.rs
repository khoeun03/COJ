use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "checker_type", rename_all = "snake_case")]
pub enum CheckerType {
    Exact,
    Epsilon,
    SpecialJudge,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Problem {
    pub id: i64,
    pub title: String,
    pub statement: String,
    pub time_limit_ms: i32,
    pub memory_limit_kb: i32,
    pub checker_type: CheckerType,
    pub checker_config: Option<serde_json::Value>,
    pub dataset_uri: String,
    pub dataset_hash: String,
    pub allowed_languages: Option<Vec<String>>,
    pub max_files: i32,
    pub max_total_code_bytes: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub revision: i32,
}
