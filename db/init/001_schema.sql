CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- ============================================================
-- Problem
-- ============================================================
CREATE TYPE checker_type AS ENUM ('exact', 'epsilon', 'special_judge');

CREATE TABLE problem (
    id              BIGSERIAL PRIMARY KEY,
    title           TEXT        NOT NULL,
    statement       TEXT        NOT NULL,
    time_limit_ms   INT         NOT NULL,
    memory_limit_kb INT         NOT NULL,
    checker_type    checker_type NOT NULL DEFAULT 'exact',
    checker_path    TEXT,
    dataset_path    TEXT        NOT NULL,
    dataset_hash    TEXT        NOT NULL,
    allowed_languages TEXT[],
    max_files           INT     NOT NULL DEFAULT 1,
    max_total_code_bytes INT    NOT NULL DEFAULT 65536,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    revision        INT         NOT NULL DEFAULT 1
);

-- ============================================================
-- Submission
-- ============================================================
CREATE TABLE submission (
    id              BIGSERIAL PRIMARY KEY,
    problem_id      BIGSERIAL   NOT NULL REFERENCES problem(id),
    user_public_key TEXT        NOT NULL,
    submitted_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    signature       TEXT        NOT NULL
);

CREATE INDEX idx_submission_problem   ON submission(problem_id);
CREATE INDEX idx_submission_user      ON submission(user_public_key);
CREATE INDEX idx_submission_submitted ON submission(submitted_at DESC);

-- ============================================================
-- Submission File
-- ============================================================
CREATE TABLE submission_file (
    id              BIGSERIAL PRIMARY KEY,
    submission_id   BIGSERIAL   NOT NULL REFERENCES submission(id) ON DELETE CASCADE,
    filename        TEXT        NOT NULL,
    language        TEXT,
    code            TEXT        NOT NULL,

    UNIQUE (submission_id, filename)
);

-- ============================================================
-- Verdict
-- ============================================================
CREATE TYPE verdict_result AS ENUM (
    'AC',   -- Accepted
    'WA',   -- Wrong Answer
    'TLE',  -- Time Limit Exceeded
    'MLE',  -- Memory Limit Exceeded
    'RE',   -- Runtime Error
    'CE',   -- Compilation Error
    'IE'    -- Internal Error
);

CREATE TABLE verdict (
    id                  BIGSERIAL PRIMARY KEY,
    submission_id       BIGSERIAL       NOT NULL REFERENCES submission(id),
    result              verdict_result  NOT NULL,
    time_ms             INT,
    memory_kb           INT,
    problem_revision    INT             NOT NULL,
    dataset_hash        TEXT            NOT NULL,
    judged_at           TIMESTAMPTZ     NOT NULL DEFAULT now(),
    judge_signature     TEXT            NOT NULL,
    invalidated_at      TIMESTAMPTZ,
    invalidated_reason  TEXT
);

CREATE INDEX idx_verdict_submission ON verdict(submission_id);
CREATE INDEX idx_verdict_result     ON verdict(result);
CREATE INDEX idx_verdict_judged     ON verdict(judged_at DESC);
