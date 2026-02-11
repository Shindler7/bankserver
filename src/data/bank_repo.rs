//! Управление счетами пользователей.

use crate::{
    domain::{Account, Currency, MonetaryAmounts, Transfer},
    errors::BankError,
};
use async_trait::async_trait;
use chrono::Utc;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tracing::{info, instrument, warn};
use uuid::Uuid;

#[async_trait]
pub trait AccountRepository: Send + Sync {
    /// Создать запись с новым аккаунтом.
    fn create(&self, account: Account) -> Result<(), BankError>;
    /// Выгрузить данные об аккаунте.
    fn get(&self, id: u32) -> Result<Account, BankError>;
    /// Обновить данные об аккаунте.
    fn update(&self, account: Account) -> Result<(), BankError>;
}

/// Структура агрегированния данных об аккаунтах.
///
/// Для режима разработки без базы данных.
#[derive(Default, Clone)]
pub struct InMemoryBankService {
    /// Созданные аккаунты клиентов Банка.
    accounts: Arc<RwLock<HashMap<Uuid, Account>>>,
}

impl InMemoryBankService {
    /// Создать нового клиента и добавить в базу.
    pub async fn new_client(
        &self,
        owner: String,
        balance: MonetaryAmounts,
        currency: Currency,
    ) -> Result<Account, BankError> {
        if self.is_owner_exists(&owner).await {
            return Err(BankError::db_err(format!(
                "{} уже имеет аккаунт в системе",
                owner
            )));
        }

        let account_id = Uuid::new_v4();

        let client = Account {
            id: account_id,
            balance,
            currency,
            owner_name: owner,
            created_at: Utc::now(),
        };

        let mut accounts = self.accounts.write().await;
        accounts.insert(account_id, client.clone());

        Ok(client)
    }

    /// Предоставить экземпляр [`Account`] для запрошенного клиента, если
    /// существует. В ином случае вернётся ошибка.
    pub async fn get_client(&self, owner: &str) -> Result<Account, BankError> {
        let accounts = self.accounts.read().await;

        let result = accounts
            .values()
            .find(|&x| x.owner_name.to_lowercase().eq(&owner.to_lowercase()))
            .cloned()
            .ok_or_else(|| BankError::db_err(format!("клиент {} не найден", owner)));

        if let Err(ref err) = result {
            warn!(error = %err, "get_client");
        }

        result
    }

    /// Предоставить вектор со всеми клиентами в базе.
    pub async fn clients_all(&self) -> Vec<Account> {
        let accounts = self.accounts.read().await;
        accounts.values().cloned().collect()
    }

    /// Совершить перевод между клиентами.
    ///
    /// Должна быть идентичная валюта счёта, а также баланс отправителя
    /// достаточный для совершения перевода.
    #[instrument(
        skip(self, transfer_data),
        fields(
            from_account = %transfer_data.from_account,
            to_account = %transfer_data.to_account,
            amount = %transfer_data.amount.value,
            currency = ?transfer_data.amount.currency,
        )
    )]
    pub async fn transfer(&self, transfer_data: Transfer) -> Result<(), BankError> {
        let mut from_client = self.get_client(&transfer_data.from_account).await?;
        let mut to_client = self.get_client(&transfer_data.to_account).await?;

        match Self::validate_transfer(&from_client, &to_client, &transfer_data) {
            Ok(_) => {}
            Err(err) => {
                warn!(error = %err, "Перевод отклонён");
                return Err(err);
            }
        }

        // Подготовим обновлённые данные.
        from_client.balance -= transfer_data.amount.value;
        to_client.balance += transfer_data.amount.value;

        let mut accounts = self.accounts.write().await;
        accounts.insert(from_client.id, from_client);
        accounts.insert(to_client.id, to_client);

        info!("Перевод выполнен успешно");

        Ok(())
    }

    /// Локальный валидатор данных для `transfer`.
    fn validate_transfer(
        from_client: &Account,
        to_client: &Account,
        transfer_data: &Transfer,
    ) -> Result<(), BankError> {
        if from_client.id == to_client.id {
            return Err(BankError::db_err("попытка перевода внутри одного аккаунта"));
        }

        if from_client.currency != transfer_data.amount.currency
            || to_client.currency != transfer_data.amount.currency
        {
            return Err(BankError::db_err("перевод должен быть в валюте аккаунта"));
        }

        if from_client.balance < transfer_data.amount.value {
            return Err(BankError::db_err("недостаточно средств для перевода"));
        }

        Ok(())
    }

    /// Проверка, что владелец счёта ещё не зарегистрирован.
    ///
    /// Регистр не учитывается: `ИВАН == Иван`.
    async fn is_owner_exists(&self, owner: &str) -> bool {
        let accounts = self.accounts.read().await;
        accounts
            .values()
            .any(|acc| acc.owner_name.to_lowercase().eq(&owner.to_lowercase()))
    }
}
