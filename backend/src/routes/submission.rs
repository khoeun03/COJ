use actix_web::{get, post, web, HttpResponse};
use sqlx::PgPool;

use crate::errors::ApiError;
use crate::models::submission::{Submission, SubmissionFile, SubmitRequest, SubmitResponse};

#[post("/api/submissions")]
async fn submit(
    pool: web::Data<PgPool>,
    body: web::Json<SubmitRequest>,
) -> Result<HttpResponse, ApiError> {
    let req = body.into_inner();

    // 문제 존재 여부 및 제약 조건 확인
    let problem = sqlx::query!(
        "SELECT id, max_files, max_total_code_bytes FROM problem WHERE id = $1",
        req.problem_id
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound("problem not found".into()))?;

    if req.files.is_empty() {
        return Err(ApiError::BadRequest("at least one file required".into()));
    }

    if req.files.len() as i32 > problem.max_files {
        return Err(ApiError::BadRequest(format!(
            "too many files: {} > {}",
            req.files.len(),
            problem.max_files
        )));
    }

    let total_bytes: usize = req.files.iter().map(|f| f.code.len()).sum();
    if total_bytes as i32 > problem.max_total_code_bytes {
        return Err(ApiError::BadRequest(format!(
            "total code size {total_bytes} exceeds limit {}",
            problem.max_total_code_bytes
        )));
    }

    // TODO: 서명 검증 (user_public_key + signature)

    // 트랜잭션으로 submission + files 삽입
    let mut tx = pool.begin().await?;

    let row = sqlx::query_scalar!(
        r#"
        INSERT INTO submission (problem_id, user_public_key, signature)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
        req.problem_id,
        req.user_public_key,
        req.signature,
    )
    .fetch_one(&mut *tx)
    .await?;

    for file in &req.files {
        sqlx::query!(
            r#"
            INSERT INTO submission_file (submission_id, filename, language, code)
            VALUES ($1, $2, $3, $4)
            "#,
            row,
            file.filename,
            file.language,
            file.code,
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(HttpResponse::Created().json(SubmitResponse { submission_id: row }))
}

#[get("/api/submissions/{id}")]
async fn get_submission(
    pool: web::Data<PgPool>,
    path: web::Path<i64>,
) -> Result<HttpResponse, ApiError> {
    let id = path.into_inner();

    let submission = sqlx::query_as!(
        Submission,
        "SELECT id, problem_id, user_public_key, submitted_at, signature FROM submission WHERE id = $1",
        id
    )
    .fetch_optional(pool.get_ref())
    .await?
    .ok_or_else(|| ApiError::NotFound("submission not found".into()))?;

    let files = sqlx::query_as!(
        SubmissionFile,
        "SELECT id, submission_id, filename, language, code FROM submission_file WHERE submission_id = $1",
        id
    )
    .fetch_all(pool.get_ref())
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "submission": submission,
        "files": files,
    })))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(submit).service(get_submission);
}
