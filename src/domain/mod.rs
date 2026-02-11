//! Чистые типы без привязки к HTTP и базе данных.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Тип данных для денежных сумм (в минимальных единицах валюты).
///
/// Например: 150 = 1.50 RUB, 250 = 2.50 USD
pub type MonetaryAmounts = u64;

/// Доступные валюты для счетов.
///
/// ISO 4217 код, например "USD", "EUR".
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Currency {
    /// Доллары США.
    Usd,
    /// Российские рубли.
    Rub,
    /// Евро.
    Eur,
}

/// Структура финансового аккаунта с балансом.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Account {
    /// Уникальный идентификатор счёта.
    pub id: Uuid,
    /// Баланс счёта.
    pub balance: MonetaryAmounts,
    /// Валюта счёта.
    pub currency: Currency,
    /// Владелец счёта.
    pub owner_name: String,
    /// Дата создания счёта (временная метка).
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// DTO для создания нового счёта.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateAccount {
    /// Начальный баланс счёта (может быть 0).
    pub initial_balance: MonetaryAmounts,
    /// Валюта счёта.
    pub currency: Currency,
    /// Имя владельца счёта.
    pub owner_name: String,
}

/// Структура для представления денежной суммы.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Amount {
    /// Числовое значение суммы.
    pub value: MonetaryAmounts,
    /// Валюта суммы.
    pub currency: Currency,
}

/// Структура для операции перевода между счетами.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Transfer {
    /// Сумма перевода.
    pub amount: Amount,
    /// Имя отправителя.
    pub from_account: String,
    /// Имя получателя.
    pub to_account: String,
}
