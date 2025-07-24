use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::Query,
    routing::get,
    Router,
};
use serde::Deserialize;
use std::env;
use std::fs;
use tower_http::services::ServeDir;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    namespace: String,
    pod_name: String,
    node_name: String,
    service_account: String,
    custom_vars: Vec<(String, String)>,
}

#[derive(Template)]
#[template(path = "testing.html")]
struct TestingTemplate {
    content: String,
    file_path: String,
}

#[derive(Deserialize)]
struct InfoQuery {
    debug: Option<String>,
}

#[derive(Deserialize)]
struct Config {
    mock_tests: MockTestsConfig,
}

#[derive(Deserialize)]
struct MockTestsConfig {
    number: i32,
}

fn load_config() -> Config {
    let config_content = fs::read_to_string("config.toml")
        .expect("Failed to read config.toml");
    toml::from_str(&config_content)
        .expect("Failed to parse config.toml")
}

async fn index(Query(params): Query<InfoQuery>) -> impl IntoResponse {
    let namespace = env::var("NAMESPACE").unwrap_or_else(|_| "default".to_string());
    let pod_name = env::var("POD_NAME").unwrap_or_else(|_| "unknown-pod".to_string());
    let node_name = env::var("NODE_NAME").unwrap_or_else(|_| "unknown-node".to_string());
    let service_account = env::var("SERVICE_ACCOUNT").unwrap_or_else(|_| "default".to_string());

    let mut custom_vars = Vec::new();
    
    // Collect custom environment variables starting with APP_
    for (key, value) in env::vars() {
        if key.starts_with("APP_") {
            custom_vars.push((key, value));
        }
    }

    // Add debug info if requested
    if params.debug.is_some() {
        custom_vars.push(("HOSTNAME".to_string(), env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_string())));
        custom_vars.push(("USER".to_string(), env::var("USER").unwrap_or_else(|_| "unknown".to_string())));
    }

    let template = IndexTemplate {
        namespace,
        pod_name,
        node_name,
        service_account,
        custom_vars,
    };

    template
}

async fn testing() -> impl IntoResponse {
    let file_path = "content.txt";
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => format!("Error reading file: {}\n\nPlease create a 'content.txt' file in the project root.", err),
    };

    let template = TestingTemplate {
        content,
        file_path: file_path.to_string(),
    };

    template
}

async fn health() -> impl IntoResponse {
    "OK"
}

async fn mock_tests() -> impl IntoResponse {
    let config = load_config();
    config.mock_tests.number.to_string()
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);

    println!("🚀 Starting ArgoFlux Sample Server on {}", bind_addr);

    let app = Router::new()
        .route("/", get(index))
        .route("/testing", get(testing))
        .route("/health", get(health))
        .route("/mock_tests", get(mock_tests))
        .nest_service("/static", ServeDir::new("static"));

    let listener = tokio::net::TcpListener::bind(&bind_addr).await.unwrap();
    
    println!("🌐 Server running at http://{}", bind_addr);
    
    axum::serve(listener, app).await.unwrap();
}