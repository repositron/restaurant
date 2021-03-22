use actix_web::{get, HttpResponse, Responder, web};

use crate::domain::menu_service::*;
use crate::error::Error;
use crate::models::menu::*;

#[get("/menu/item")]
async fn again() -> impl Responder {
    println!("GET: /again");
    HttpResponse::Ok().body("Hello world again!")
}

pub fn menu_route_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/menu")
            .route(web::get().to(get_all_menu))
            .route(web::post().to(post_menu))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
        )
        .service(web::resource("/menu/{num}")
            .route(web::get().to(get_menu)));

}

async fn post_menu(menu: web::Json<NewMenu>) -> Result<HttpResponse, Error> {
    log::info!("post_menu {}", menu.item_name);
    let created_menu = MenuService::create_menu(menu.into_inner()).await;
    match created_menu {
        Ok(_) => Ok(HttpResponse::Ok().json(created_menu.unwrap())),
        Err(e) => {
            log::info!("error {}", e);
            HttpResponse::NoContent().json("");
            Err(crate::error::Error::InternalServerError)
        }
    }
}

async fn get_menu(menu_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    log::info!("get_menu {}", menu_id);
    let request = MenuService::get_menu(menu_id.into_inner()).await;
    match request {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => {
            log::info!("error {}", e);
            HttpResponse::NoContent().json("");
            Err(crate::error::Error::NotFound)
        }
    }
}

async fn get_all_menu() -> Result<HttpResponse, Error> {
    log::info!("get_all_menu");
    let request = MenuService::get_all_menu().await;
    match request {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => {
            log::info!("error {}", e);
            HttpResponse::NoContent().json("");
            Err(crate::error::Error::NotFound)
        }
    }
}