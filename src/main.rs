mod application;
mod data;
mod domain;
mod errors;
mod infrastructure;
mod presentation;

use crate::data::bank_repo::InMemoryBankService;
use crate::infrastructure::config;
use crate::infrastructure::logging::init_logging;
use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpResponse, HttpServer};
use std::io::Result as IoResult;

/// Общие данные и параметры конфигурации для передачи в приложение.
struct AppState {
    /// Реестр клиентов банка и их данных.
    bank: InMemoryBankService,
}

#[actix_web::main]
async fn main() -> IoResult<()> {
    init_logging();

    tracing::info!("Настройка сервера перед запуском...");

    dotenvy::dotenv().ok();

    let cfg = config::AppConfig::init().unwrap_or_else(|err| panic!("{}", err));
    tracing::info!("Подгружена конфигурация");

    let addr = format!("{}:{}", cfg.server.host, cfg.server.port);
    let app_state = web::Data::new(AppState {
        bank: InMemoryBankService::new(),
    });

    tracing::info!("Успешное подключение к базе данных");
    tracing::info!("Сервер запущен на адресе: {}", addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cfg.security.cors_origin)
            .allowed_methods(vec!["GET", "POST", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(cfg.security.cors_max_age);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(app_state.clone())
            .service(web::scope("/api/v1").configure(presentation::api_routes::configurate))
            .service(web::scope("").configure(presentation::routes::configurate))
            .default_service(web::to(|| async { HttpResponse::NotFound().finish() }))
    })
    .bind(addr)?
    .run()
    .await
    .unwrap_or_else(|err| {
        tracing::error!("HTTP сервер остановился с ошибкой: {err}");
        panic!("{err}");
    });

    tracing::info!("Сервер остановлен");
    Ok(())
}
