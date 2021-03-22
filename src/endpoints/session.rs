use actix_web::{HttpResponse, web};
use crate::domain::session_service::SessionService;
use crate::error::Error;
use crate::models::{NewSession};

pub fn session_route_config(cfg: &mut web::ServiceConfig) {
    cfg
        .service(web::resource("/session")
            .route(web::get().to(get_all_session))
            .route(web::post().to(post_session))
        );
    //.service(web::resource("/menu/{num}")
    //    .route(web::get().to(get_menu)));
}


async fn post_session(session: web::Json<NewSession>) -> Result<HttpResponse, Error> {
    log::info!("post_session");
    let created_session = SessionService::create_session(session.into_inner()).await;
    match created_session {
        Ok(_) => Ok(HttpResponse::Ok().json(created_session.unwrap())),
        Err(e) => {
            log::info!("error {}", e);
            HttpResponse::NoContent().json("");
            Err(crate::error::Error::InternalServerError)
        }
    }
}

async fn get_session(session_id: web::Path<i32>) -> Result<HttpResponse, Error> {
    log::info!("get_session {}", session_id);
    let request = SessionService::get_session(session_id.into_inner()).await;
    match request {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => {
            log::info!("error {}", e);
            HttpResponse::NoContent().json("");
            Err(crate::error::Error::NotFound)
        }
    }
}

async fn get_all_session() -> Result<HttpResponse, Error>{
    log::info!("get_all_session");
    let request = SessionService::get_all_session().await;
    match request {
        Ok(r) => Ok(HttpResponse::Ok().json(r)),
        Err(e) => {
            log::info!("error {}", e);
            HttpResponse::NoContent().json("");
            Err(crate::error::Error::NotFound)
        }
    }
}
