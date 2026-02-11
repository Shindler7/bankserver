//! Инфраструктура логирования.

use tracing_subscriber::{
    EnvFilter, fmt::time::ChronoUtc, layer::SubscriberExt, util::SubscriberInitExt,
};

/// Базовая инициализация логера.
pub fn init_logging() {
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "actix_web=info,server=debug".into()),
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .with_timer(ChronoUtc::rfc_3339()),
        )
        .init();

    tracing::info!("Логирование инициализировано");
}
