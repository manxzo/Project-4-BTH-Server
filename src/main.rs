mod auth;
mod db;
mod middleware;
mod handlers;
mod models;
mod routes;

use actix_web::{App, HttpServer, web};

use middleware::auth_middleware::AuthMiddleware;
use handlers::ws::init_ws_routes;
use routes::{user_auth::config_user_auth_routes,user_info::config_user_info_routes,sponsor::config_sponsor_routes,matching::config_matching_routes};
use std::io::Result as IoResult;
use crate::db::connect_db;

#[actix_web::main]
async fn main() -> IoResult<()> {
    dotenvy::dotenv().ok();
    let pool = connect_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/public")
                            .configure(config_user_auth_routes)
                            
                    )
                    .service(
                        web::scope("/protected")
                            .wrap(AuthMiddleware)
                            .configure(config_user_info_routes)
                            .configure(config_sponsor_routes)
                            .configure(config_matching_routes)
                    )
            )
            .configure(init_ws_routes)
    })
    .bind("localhost:3000")?
    .run()
    .await
}
