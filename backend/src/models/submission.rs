use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct Submission {
    pub id: i64,
    pub problem_id: i64,
    pub user_public_key: String,
    pub submitted_at: DateTime<Utc>,
    pub signature: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SubmissionFile {
    pub id: i64,
    pub submission_id: i64,
    pub filename: String,
    pub language: Option<String>,
    pub code: String,
}

/// 클라이언트가 보내는 제출 요청
#[derive(Debug, Deserialize)]
pub struct SubmitRequest {
    pub problem_id: i64,
    pub user_public_key: String,
    pub signature: String,
    pub files: Vec<SubmitFile>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitFile {
    pub filename: String,
    pub language: Option<String>,
    pub code: String,
}

/// 제출 성공 응답
#[derive(Debug, Serialize)]
pub struct SubmitResponse {
    pub submission_id: i64,
}
