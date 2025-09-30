use std::io;

use actix_web::{App, HttpResponse, HttpServer, Responder, web};

// 配置路由
pub fn general_routes(config: &mut web::ServiceConfig) {
    config.route("/health", web::get().to(health_check_handler));
}

// 配置handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix Web Service is running!")
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    //构建app ,配置route
    let app = || App::new().configure(general_routes);
    // 运行Http server
    HttpServer::new(app).bind("localhost:3000")?.run().await
}
