use actix_web::{HttpResponse, web};

use crate::domain::table_service::*;
use crate::error::Error;
use crate::models::{NewCustomerTable};

pub fn table_route_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/table")
            .route(web::get().to(get_all_table))
            .route(web::post().to(post_table))
        )
        .service(web::resource("/table/{num}")
            .route(web::get().to(get_table)));
}

async fn post_table(table: web::Json<NewCustomerTable>) -> Result<HttpResponse, Error> {
    log::info!("post_table");
    let created_table = TableService::create_table(table.into_inner()).await;
    match created_table {
        Ok(_) => Ok(HttpResponse::Ok().json(created_table.unwrap())),
        Err(e) => {
            log::info!("error {}", e);
            HttpResponse::NoContent().json("");
            Err(crate::error::Error::InternalServerError)
        }
    }
}

async fn get_table(table_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    log::info!("get_table {}", table_id);
    let request = TableService::get_table(table_id.into_inner()).await;
    match request {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => {
            log::info!("error {}", e);
            HttpResponse::NoContent().json("");
            Err(crate::error::Error::NotFound)
        }
    }
}

async fn get_all_table() -> Result<HttpResponse, Error> {
    log::info!("get_all_table");
    let request = TableService::get_all_table().await;
    match request {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => {
            log::info!("error {}", e);
            HttpResponse::NoContent().json("");
            Err(crate::error::Error::NotFound)
        }
    }
}