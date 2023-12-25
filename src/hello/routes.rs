use actix_web::{HttpResponse, web, get, Responder};

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello World!")
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(hello);
}