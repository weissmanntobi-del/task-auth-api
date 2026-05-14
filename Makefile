.PHONY: fmt lint test check run docker-up docker-down migrate

fmt:
	cargo fmt --all

lint:
	cargo clippy --all-targets --all-features -- -D warnings

test:
	cargo test --all

check: fmt lint test

run:
	cargo run

docker-up:
	docker compose up --build

docker-down:
	docker compose down -v

migrate:
	sqlx migrate run
