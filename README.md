제출자의 Private key / Public key 기반 인증

"프로필"에 관한 정보는 랭킹 서버에서?
"채점 서버"는 신원만

- Solution을 서명해서 제출 <--

- 통과 점수
- 채점 결과 점수

채점 서버는 User 정보를 알 필요가 없음!

- 문제: (서버식별자)/(문제번호)

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