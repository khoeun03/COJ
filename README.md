https://github.com/nulta/deps-protocol

Problem
- id
- title
- statement (text, markdown)
- time_limit_ms
- memory_limit_kb
- checker_type
- checker_uri
- dataset_uri
- dataset_hash
- allowed_languages (text[] | null)
- max_files
- max_total_code_bytes
- created_at
- updated_at
- revision

Submission
- id
- problem_id
- user_public_key
- submitted_at
- signature

SubmissionFile
- id
- submission_id
- filename
- language
- code

Verdict
- id
- submission_id
- result
- time_ms
- memory_kb
- problem_revision
- judged_at
- dateset_hash
- judge_signature
- invalidated_at
- invalidated_reason