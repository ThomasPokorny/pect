use crate::platform::config::config;
use crate::platform::config::socket_address;
use crate::platform::router::handler::get_app_router;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod pect;
mod platform;

const PECT_LABEL: &str = "
___  ____ ____ ___
|__] |___ |     |
|    |___ |___  |
🗄... v.0.1 running";

type Cache<K, V> = Arc<Mutex<HashMap<K, V>>>;

#[tokio::main]
async fn main() {
    // Create the shared cache
    let cache: Cache<String, String> = Arc::new(Mutex::new(HashMap::new()));

    let config = config().await;
    let app = get_app_router(cache);

    println!("{}", PECT_LABEL);
    let listener = tokio::net::TcpListener::bind(&socket_address(config))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
