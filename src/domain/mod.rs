pub mod menu;
pub mod order_service;
pub(crate) mod menu_service;
pub mod session_service;
pub mod table_service;

use diesel::pg::PgConnection;
use diesel::Connection;
use dotenv::dotenv;
//use crate::schema::*;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
