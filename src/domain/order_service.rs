use diesel::result::Error;
use crate::models::{NewOrder, Order};
use diesel::prelude::*;
use crate::schema::*;
use crate::domain::establish_connection;


pub struct OrderService {}

impl OrderService {
    pub async fn create_order(new_order: NewOrder) -> Result<Order, Error> {
        use crate::schema::customer_order::dsl::*;

        let insert = diesel::insert_into(customer_order)
            .values(new_order)
            .get_result(&establish_connection())?;
        Ok(insert)
    }

    pub async fn complete_order(order_id: i32) -> Result<Order, Error> {
        use crate::schema::customer_order::dsl::*;
        let select = customer_order
            .find(order_id)
            .select(customer_order::all_columns())
            .get_result(&establish_connection())?;

        Ok(select)
    }

    pub async fn get_order(order_id: i32) -> Result<Order, Error> {
        use crate::schema::customer_order::dsl::*;
        let select = customer_order
            .find(order_id)
            .select(customer_order::all_columns())
            .get_result(&establish_connection())?;

        Ok(select)
    }

    pub async fn get_all_order() -> Result<Vec<Order>, Error> {
        use crate::schema::customer_order::dsl::*;
        let select = customer_order
            .select(customer_order::all_columns())
            .load::<Order>(&establish_connection())?;
        Ok(select)
    }
}