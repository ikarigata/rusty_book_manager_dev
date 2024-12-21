use axum::{extract::State, http::StatusCode};

use registry::AppRegistry;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
// axumにregistry事態はStateで登録しておいてそこから取り出す
pub async fn health_check_db(State(registry): State<AppRegistry>) -> StatusCode {
    if registry.health_check_repository().check_db().await {
        StatusCode::OK
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}
