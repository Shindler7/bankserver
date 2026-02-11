//! Ошибки приложения.

use thiserror::Error;

/// Базовые ошибки приложения.
///
/// Используются при системных ошибках, при которых продложение работы
/// программы нецелесообразно.
#[derive(Error, Debug)]
pub enum AppError {
    /// Ошибка формирования конфигурации.
    #[error("Ошибка параметров: {0} ({1})")]
    Configuration(String, String),
    /// Ошибка доступа к базам данных.
    #[error("Ошибка доступа к БД: {0}")]
    Database(String),
}

impl AppError {
    /// Конструктор для ошибки [`AppError::Configuration`].
    ///
    /// ## Args:
    /// - `name` — имя параметра
    /// - `err_msg` — текстовое описание ошибки
    pub fn cfg_err(name: impl Into<String>, err_msg: impl Into<String>) -> AppError {
        let name = {
            let name = name.into();
            if name.is_empty() {
                "ОБЩАЯ".to_string()
            } else {
                name
            }
        };

        AppError::Configuration(name, err_msg.into())
    }

    /// Конструктор для ошибки [`AppError::db_err`].
    pub fn db_err(err_msg: impl Into<String>) -> AppError {
        AppError::Database(err_msg.into())
    }
}

/// Ошибки для представлений.
#[derive(Error, Debug)]
pub enum BankError {
    /// Ошибка валидации предоставленных данных.
    #[error("Некорректные данные: {0}")]
    Validation(String),
    /// Запрошенная информация не найдена.
    #[error("Данные не найдены: {0}")]
    NotFound(String),
    /// Ошибка недостаточности средств.
    #[error("Недостаточно средств для проведения операции")]
    InsufficientFunds,
    /// Ошибка авторизации.
    #[error("Неверные данные для авторизации")]
    Unauthorized,
    /// Ошибка обработки данных, связанная с БД (не технологическая).
    #[error("Ошибка данных: {0}")]
    Database(String),
}

impl BankError {
    /// Конструктор для ошибки [`BankError::Validation`].
    pub fn validation(msg: impl Into<String>) -> BankError {
        BankError::Validation(msg.into())
    }

    /// Конструктор для ошибки [`BankError::NotFound`].
    pub fn not_found(msg: impl Into<String>) -> BankError {
        BankError::NotFound(msg.into())
    }

    /// Конструктор для ошибки [`BankError::Database`].
    pub fn db_err(msg: impl Into<String>) -> BankError {
        BankError::Database(msg.into())
    }
}
