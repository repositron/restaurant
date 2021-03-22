use diesel::result::Error;
use crate::models::{NewCustomerTable, CustomerTable};
use diesel::prelude::*;
use crate::domain::establish_connection;
use crate::schema::*;

pub struct TableService {

}

impl TableService {
    pub async fn get_table(table_id: i32) -> Result<CustomerTable, Error> {
        use crate::schema::customer_table::dsl::*;
        let select = customer_table
            .select(customer_table::all_columns())
            .find(table_id)
            .get_result(&establish_connection())?;
        Ok(select)
    }

    pub async fn get_all_table() -> Result<Vec<CustomerTable>, Error> {
        use crate::schema::customer_table::dsl::*;
        let select = customer_table
            .select(customer_table::all_columns())
            .load::<CustomerTable>(&establish_connection())?;
        Ok(select)
    }

    pub async fn create_table(new_table: NewCustomerTable) -> Result<CustomerTable, Error> {
        use crate::schema::customer_table::dsl::*;
        let insert = diesel::insert_into(customer_table)
            .values(new_table)
            .get_result(&establish_connection())?;
        Ok(insert)
    }
}