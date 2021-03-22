use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::schema::*;
use uuid::Uuid;

pub mod menu;

#[derive(Deserialize, Serialize, Queryable)]
pub struct CustomerTable {
    pub customer_table_id: i32,
    pub code: String,
}

#[derive(Deserialize, Insertable)]
#[table_name="customer_table"]
pub struct NewCustomerTable {
    pub code: String,
}

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct Session {
    session_id: Uuid,
    customer_table_id: i32,
    start_timestamp: NaiveDateTime,
    finish_timestamp: Option<NaiveDateTime>,
}

#[derive(Deserialize, Insertable)]
#[table_name="session"]
pub struct NewSession {
    customer_table_id: i32,
   // start_timestamp: NaiveDateTime,
   // finish_timestamp: NaiveDateTime,
}


#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct Order {
    pub order_id: i32,
    pub menu_id: i32,
    pub session_id: Uuid,
    pub customer_table_id: i32,
    pub timestamp: NaiveDateTime,
    pub completed: bool,
}

#[derive(Deserialize, Insertable)]
#[table_name="customer_order"]
pub struct NewOrder {
    pub menu_id: i32,
    pub session_id: Uuid,
    pub customer_table_id: i32,
}

