//! Роутеры web-приложения.

use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

/// Роутер для предоставления данных о состоянии приложения.
///
/// `/health`
#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "ok"}))
}

/// Зарегистрировать HTTP-сервисы.
pub fn configurate(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(health);
}
