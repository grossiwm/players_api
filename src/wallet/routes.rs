use actix_web::{ HttpResponse, post, web};
use serde::Deserialize;
use crate::{error_handler::CustomError, wallet::Wallet};

#[derive(Deserialize)]
pub struct CreateWalletQuery {
    player: i32
}


#[post("/wallets")]
async fn create(query: web::Query<CreateWalletQuery>) -> Result<HttpResponse, CustomError> {
    Wallet::create(query.player);
    Ok(HttpResponse::Created().finish())
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create);
}