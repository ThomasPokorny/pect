use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use axum::{Extension, Json, Router};
use axum::extract::Path;
use axum::routing::{delete, get, post};
use serde::Deserialize;
use serde_json::json;

pub fn router() -> Router {
    Router::new()
        .route("/pect/store", post(set_entry))
        .route("/pect/store/:key", get(get_entry))
        .route("/pect/store/:key", delete(delete_entry))
        .route("/pect/keys", get(get_keys))
}

#[derive(Deserialize)]
struct CreateEntryDTO {
    pub key: String,
    pub value: String
}
async fn set_entry(
    Extension(cache): Extension<Arc<Mutex<HashMap<String, String>>>>,
    Json(set_entry): Json<CreateEntryDTO>,
)  {
    let mut cache = cache.lock().unwrap();
    cache.insert(set_entry.key, set_entry.value);
}

async fn get_entry(
    Extension(cache): Extension<Arc<Mutex<HashMap<String, String>>>>,
    Path(key): Path<String>,
) -> Json<String> {
    let cache = cache.lock().unwrap();
    let value = cache.get(&key).unwrap_or(&"Not Found".to_string()).clone();
    Json(value)
}

async fn delete_entry(
    Extension(cache): Extension<Arc<Mutex<HashMap<String, String>>>>,
    Path(key): Path<String>,
)  {
    let mut cache = cache.lock().unwrap();
    if  cache.contains_key(&key) {
        cache.remove(&key);
    }
}

async fn get_keys(
    Extension(cache): Extension<Arc<Mutex<HashMap<String, String>>>>
) -> Json<serde_json::Value> {
    let cache = cache.lock().unwrap();
    let keys: Vec<String> = cache.keys().cloned().collect();
    Json(json!({ "keys": keys.len() }))
}