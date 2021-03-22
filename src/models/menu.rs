use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct Menu {
    pub menu_id: i32,
    pub item_name: String,
    pub cooktimeseconds: i32,
    pub price: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name="menu"]
pub struct NewMenu {
    pub item_name: String,
    pub cooktimeseconds: i32,
    pub price: i32,
}