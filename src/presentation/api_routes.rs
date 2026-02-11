//! Роутеры для API-ссылок.

use crate::presentation::api_errors::BankApiError;
use crate::{
    domain::{CreateAccount, Transfer},
    AppState,
};
use actix_web::{get, post, web, HttpResponse, Responder};

/// Роутер для получения данных об аккаунте по ID пользователя.
///
/// `{url_to_api}/account`
#[get("/account/{owner}")]
async fn account(
    state: web::Data<AppState>,
    path: web::Path<String>,
) -> Result<impl Responder, BankApiError> {
    let owner_name = path.into_inner();

    let account = state.bank.get_client(&owner_name).await?;
    Ok(HttpResponse::Ok().json(account))
}

/// Роутер для выгрузки данных об имеющихся
#[get("/accounts")]
async fn accounts(state: web::Data<AppState>) -> impl Responder {
    let clients = state.bank.clients_all().await;
    HttpResponse::Ok().json(clients)
}

/// Роутер для добавления нового клиента.
#[post("/account/register")]
async fn new_client(
    state: web::Data<AppState>,
    payload: web::Json<CreateAccount>,
) -> Result<impl Responder, BankApiError> {
    let CreateAccount {
        owner_name,
        initial_balance,
        currency,
    } = payload.into_inner();

    let client = state
        .bank
        .new_client(owner_name, initial_balance, currency)
        .await?;

    Ok(HttpResponse::Ok().json(client))
}

/// Роутер для совершения перевода средств.
#[post("/accounts/transfer")]
async fn transfer(
    state: web::Data<AppState>,
    payload: web::Json<Transfer>,
) -> Result<impl Responder, BankApiError> {
    let transfer: Transfer = payload.into_inner();
    state.bank.transfer(transfer).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Зарегистрировать HTTP-сервисы.
pub fn configurate(cfg: &mut web::ServiceConfig) {
    cfg.service(account)
        .service(new_client)
        .service(transfer)
        .service(accounts);
}
