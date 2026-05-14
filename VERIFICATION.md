# Verification Checklist

Run these commands before publishing a release:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
```

For the database-backed integration test:

```bash
docker compose up -d db
export TEST_DATABASE_URL=postgres://app:app@localhost:5432/task_api
cargo test register_login_and_me_flow_works -- --nocapture
```

For container smoke testing:

```bash
docker compose up --build
curl -f http://localhost:3000/health/live
curl -f http://localhost:3000/health/ready
```

A production-grade claim should be attached to verified behavior, not just code snippets in a book.
