use actix_web::{ HttpResponse, post, web, get};
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

// #[get("/wallets/1")]
// async fn get_wallet() -> Result<HttpResponse, CustomError> {
//     let balance = Wallet::get_balance().await;
//     Ok(HttpResponse::Ok().body(balance))
// }

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(create);
    // config.service(get_wallet);
}