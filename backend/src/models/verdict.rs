use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "verdict_result")]
pub enum VerdictResult {
    AC,
    WA,
    TLE,
    MLE,
    RE,
    CE,
    IE,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Verdict {
    pub id: i64,
    pub submission_id: i64,
    pub result: VerdictResult,
    pub time_ms: Option<i32>,
    pub memory_kb: Option<i32>,
    pub problem_revision: i32,
    pub dataset_hash: String,
    pub judged_at: DateTime<Utc>,
    pub judge_signature: String,
    pub invalidated_at: Option<DateTime<Utc>>,
    pub invalidated_reason: Option<String>,
}
