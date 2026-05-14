use axum::body::Body;
use axum::http::{header, Request, StatusCode};
use http_body_util::BodyExt;
use secrecy::SecretString;
use serde_json::{json, Value};
use sqlx::postgres::PgPoolOptions;
use task_auth_api::{
    app,
    config::{AppConfig, AuthConfig, DatabaseConfig, ServerConfig, TracingConfig},
    db::pool::run_migrations,
    state::AppState,
};
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
async fn register_login_and_me_flow_works() {
    let Ok(database_url) = std::env::var("TEST_DATABASE_URL") else {
        eprintln!(
            "skipping integration test: set TEST_DATABASE_URL to a disposable Postgres database"
        );
        return;
    };

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("connect test database");

    run_migrations(&pool).await.expect("run migrations");

    let state = build_test_state(pool);
    let app = app::router(state);

    let email = format!("user-{}@example.com", Uuid::new_v4());
    let password = "correct-horse-battery-staple";

    let register = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/auth/register",
            json!({ "email": email, "password": password }),
        ))
        .await
        .expect("register request");

    assert_eq!(register.status(), StatusCode::CREATED);

    let login = app
        .clone()
        .oneshot(json_request(
            "POST",
            "/auth/login",
            json!({ "email": email, "password": password }),
        ))
        .await
        .expect("login request");

    assert_eq!(login.status(), StatusCode::OK);
    let login_body = read_json(login).await;
    let access_token = login_body["access_token"]
        .as_str()
        .expect("access_token in login response");

    let me = app
        .oneshot(
            Request::get("/me")
                .header(header::AUTHORIZATION, format!("Bearer {access_token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("profile request");

    assert_eq!(me.status(), StatusCode::OK);
}

fn build_test_state(pool: sqlx::PgPool) -> AppState {
    AppState::new(
        AppConfig {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 0,
            },
            database: DatabaseConfig {
                url: SecretString::from("postgres://unused".to_string()),
                max_connections: 5,
                connect_timeout_secs: 5,
            },
            auth: AuthConfig {
                jwt_secret: SecretString::from(
                    "test-secret-change-in-real-deployments".to_string(),
                ),
                access_token_ttl_secs: 900,
                refresh_token_ttl_days: 30,
            },
            tracing: TracingConfig {
                filter: "info".to_string(),
            },
        },
        pool,
    )
}

fn json_request(method: &str, path: &str, body: Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(path)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

async fn read_json(response: axum::response::Response) -> Value {
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}
