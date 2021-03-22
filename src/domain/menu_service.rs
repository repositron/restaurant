use diesel::result::Error;
use crate::models::menu::{NewMenu, Menu};
use diesel::prelude::*;
use crate::domain::establish_connection;
use crate::schema::*;

pub struct MenuService {

}

impl MenuService {
    pub async fn get_menu(menu_id: i32) -> Result<Menu, Error> {
        use crate::schema::menu::dsl::*;
        let select = menu
            .select(menu::all_columns())
            .find(menu_id)
            .get_result(&establish_connection())?;
        Ok(select)
    }

    pub async fn get_all_menu() -> Result<Vec<Menu>, Error> {
        use crate::schema::menu::dsl::*;
        let select = menu
            .select(menu::all_columns())
            .load::<Menu>(&establish_connection())?;
        Ok(select)
    }

    pub async fn create_menu(new_menu: NewMenu) -> Result<Menu, Error> {
        use crate::schema::menu::dsl::*;
        let insert = diesel::insert_into(menu)
            .values(new_menu)
            .get_result(&establish_connection())?;
        Ok(insert)
    }
}
