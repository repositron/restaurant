use crate::models::menu as menu;
struct MenuServiceImpl {}

trait MenuService {
    fn create(menu: menu::Menu) -> menu::Menu;// : models::menu;
}

impl MenuServiceImpl {

}

impl MenuService for MenuServiceImpl {
    fn create(menu: menu::Menu) -> menu::Menu {
        unimplemented!()
    }
}