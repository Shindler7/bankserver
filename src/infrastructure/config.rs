//! Конфигурация приложения.

use crate::errors::AppError;
use serde::{Deserialize, Serialize};
use std::{env, fmt::Display, str::FromStr};

/// Унифицированный трейт для формирования конфигурации приложения.
trait Configuration {
    type OneAppConfig;

    /// Загрузить конфигурацию из внешних источников (.env и другие).
    fn load() -> Result<Self::OneAppConfig, AppError>;
}

/// Комплексная структура параметров приложения.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    /// Базовые настройки сервера.
    pub server: ServerConfig,
    /// Базовые настройки безопасности сервера.
    pub security: SecurityConfig,
    /// Настройки для базы данных.
    pub db: DBConfig,
}

impl AppConfig {
    /// Формирует конфигурацию приложения, с учётом его состояния.
    pub fn init() -> Result<Self, AppError> {
        let server = ServerConfig::load()?;
        let security = SecurityConfig::load()?;
        let db = DBConfig::default();

        Ok(Self {
            server,
            security,
            db,
        })
    }
}

/// Базовые настройки сервера.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Хост для запуска сервера.
    pub host: String,
    /// Порт для запуска сервера.
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".into(),
            port: 8080,
        }
    }
}

impl Configuration for ServerConfig {
    type OneAppConfig = ServerConfig;

    fn load() -> Result<Self, AppError> {
        let host: String = load_from_env("SERVER_HOST")?;
        let port: u16 = load_from_env("SERVER_PORT")?;

        Ok(Self { host, port })
    }
}

/// Базовые настройки безопасности сервера.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// Разрешённый origin для CORS (или "*" для всех).
    pub cors_origin: String,
    /// Таймаут запроса в секундах.
    pub cors_max_age: usize,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            cors_origin: "*".into(),
            cors_max_age: 600,
        }
    }
}

impl Configuration for SecurityConfig {
    type OneAppConfig = SecurityConfig;

    fn load() -> Result<Self, AppError> {
        let cors_origin = load_from_env("CORS_ORIGIN")?;

        Ok(Self {
            cors_origin,
            ..Default::default()
        })
    }
}

/// Настройки для базы данных.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DBConfig {
    /// Ссылка для доступа к базе данных.
    pub database_url: String,
    /// Максимальное число соединений, поддерживаемых пулом.
    /// Необходимо учитывать лимиты соединений выбранной базы данных.
    pub max_conn: u32,
}

impl Configuration for DBConfig {
    type OneAppConfig = DBConfig;

    fn load() -> Result<Self, AppError> {
        let database_url = load_from_env("DATABASE_URL")?;
        let max_conn = 10;

        Ok(Self {
            database_url,
            max_conn,
        })
    }
}

/// Загрузить указанный параметр из окружения.
///
/// Дженерик преобразует значение из файла в требуемый тип, если возможно.
fn load_from_env<T>(name: &str) -> Result<T, AppError>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let s = env::var(name).map_err(|err| AppError::cfg_err(name, err.to_string()))?;

    s.parse::<T>()
        .map_err(|err| AppError::cfg_err(name, err.to_string()))
}
