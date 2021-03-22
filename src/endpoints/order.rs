use actix_web::{HttpResponse, web};

use crate::domain::order_service::*;
use crate::error::Error;
use crate::models::{NewOrder};

pub fn order_route_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/order")
            .route(web::get().to(get_all_order))
            .route(web::post().to(post_order))
            .route(web::head().to(|| HttpResponse::MethodNotAllowed()))
        )
        .service(web::resource("/order/{num}")
            .route(web::get().to(get_order)));
}

async fn post_order(order: web::Json<NewOrder>) -> Result<HttpResponse, Error> {
    log::info!("post_order");
    let created_order = OrderService::create_order(order.into_inner()).await;
    match created_order {
        Ok(_) => Ok(HttpResponse::Ok().json(created_order.unwrap())),
        Err(e) => {
            log::info!("error {}", e);
            HttpResponse::NoContent().json("");
            Err(crate::error::Error::InternalServerError)
        }
    }
}

async fn get_order(order_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    log::info!("get_order {}", order_id);
    let request = OrderService::get_order(order_id.into_inner()).await;
    match request {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => {
            log::info!("error {}", e);
            HttpResponse::NoContent().json("");
            Err(crate::error::Error::NotFound)
        }
    }
}

async fn get_all_order() -> Result<HttpResponse, Error> {
    log::info!("get_all_order");
    let request = OrderService::get_all_order().await;
    match request {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => {
            log::info!("error {}", e);
            HttpResponse::NoContent().json("");
            Err(crate::error::Error::NotFound)
        }
    }
}