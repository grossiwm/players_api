
use crate::{error_handler::CustomError, auth::generate_jwt, player::{Player, NoIdPlayer, AuthenticablePlayer}};
use actix_web::{get, post, delete, put, web, HttpResponse};


#[get("/players")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let players = web::block(|| Player::find_all()).await.unwrap();
    Ok(HttpResponse::Ok().json(players))
}

#[get("/players/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let player = Player::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(player))
}

#[post("/players")]
async fn create(player: web::Json<NoIdPlayer>) -> Result<HttpResponse, CustomError> {
    Player::create(player.into_inner());
    Ok(HttpResponse::Created().finish())
}

#[delete("/players/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    Player::delete(id.into_inner());
    Ok(HttpResponse::NoContent().finish())
}

#[put("/players/{id}")]
async fn update(id: web::Path<i32>, player: web::Json<NoIdPlayer>) -> Result<HttpResponse, CustomError> {
    let _ = Player::update(id.into_inner(), player.into_inner());
    Ok(HttpResponse::Ok().finish())
}

#[post("/players/authenticate")]
async fn authenticate(player: web::Json<AuthenticablePlayer>) -> Result<HttpResponse, CustomError> {
    let authenticated_player = Player::authenticate(player.into_inner());
    match authenticated_player {
        Ok(player) => {
            let token  = generate_jwt(player);
            Ok(HttpResponse::Ok().json(serde_json::json!({ "token": token })))
        },
        Err(_) => {
            Ok(HttpResponse::Unauthorized().finish())
        }
    }
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(find);
    config.service(create);
    config.service(delete);
    config.service(update);
    config.service(authenticate);
}
