use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};

pub mod menu;
mod systems;
pub mod session;
pub mod table;
pub mod order;

#[actix_rt::main]
pub async fn build() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(menu::menu_route_config)
            .configure(table::table_route_config)
            .configure(session::session_route_config)
            .configure(order::order_route_config)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

/*fn routes(app: &mut web::ServiceConfig) {
    app.service(web::resource("/healthz").route(web::get().to(systems::healthz)))
        .service(web::resoure)

}
*/