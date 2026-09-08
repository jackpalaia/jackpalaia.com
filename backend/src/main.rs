use std::{env, io::ErrorKind, path::PathBuf, sync::Arc};

use axum::{extract::State, http::StatusCode, routing::get, Json, Router};
use serde_json::{json, Map, Value};
use tokio::sync::Mutex;
use tower_http::services::ServeDir;

struct JsonFile {
    path: PathBuf,
    write_lock: Mutex<()>,
}

impl JsonFile {
    fn new(path: PathBuf) -> Arc<Self> {
        Arc::new(Self {
            path,
            write_lock: Mutex::new(()),
        })
    }
}

async fn get_json(State(file): State<Arc<JsonFile>>) -> Result<Json<Value>, StatusCode> {
    let _guard = file.write_lock.lock().await;
    match tokio::fs::read(&file.path).await {
        Ok(bytes) => serde_json::from_slice(&bytes)
            .map(Json)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR),
        Err(e) if e.kind() == ErrorKind::NotFound => Ok(Json(Value::Object(Map::new()))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn put_json(
    State(file): State<Arc<JsonFile>>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    if !(body.is_object() || body.is_array()) {
        return Err(StatusCode::BAD_REQUEST);
    }
    let text = serde_json::to_string_pretty(&body).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _guard = file.write_lock.lock().await;
    if let Some(dir) = file.path.parent() {
        tokio::fs::create_dir_all(dir)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }
    let tmp = file.path.with_extension("json.tmp");
    tokio::fs::write(&tmp, text)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    tokio::fs::rename(&tmp, &file.path)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(json!({ "ok": true })))
}

#[tokio::main]
async fn main() {
    let scores_path = env::var("CHORD_SCORES_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("/website/data/chord_scores.json"));
    let selected_path = env::var("CHORD_SELECTED_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| scores_path.with_file_name("chord_selected.json"));

    let app = Router::new()
        .route(
            "/api/chords/scores",
            get(get_json).post(put_json).with_state(JsonFile::new(scores_path)),
        )
        .route(
            "/api/chords/selected",
            get(get_json).post(put_json).with_state(JsonFile::new(selected_path)),
        )
        .fallback_service(ServeDir::new("../frontend/out"));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
