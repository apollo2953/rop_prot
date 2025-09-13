pub use crate::menu::menu::Menu;
use crate::user::user::User;
use std::sync::Mutex;

pub mod menu;
pub mod payment;
pub mod storage;
pub mod table;
pub mod user;
pub mod util;

pub struct Permission {
    // <- the struct to check for permissions on each user.
    table_create: bool, //    TODO: needs more work.
    table_delete: bool,
    table_edit: bool,
    table_open: bool,
    table_close: bool,
    menu_item_create: bool,
    menu_item_edit: bool,
    menu_item_delete: bool,
    add_discount: bool,
    user_create: bool,
    user_edit: bool,
    user_delete: bool,
    take_pay: bool,
}

impl Permission {
    pub fn new(
        table_create: bool,
        table_delete: bool,
        table_edit: bool,
        table_open: bool,
        table_close: bool,
    ) -> Permission {
        Self {
            table_create: false,
            table_delete: false,
            table_edit: false,
            table_open: false,
            table_close: false,
            menu_item_create: false,
            menu_item_edit: false,
            menu_item_delete: false,
            add_discount: false,
            user_create: false,
            user_edit: false,
            user_delete: false,
            take_pay: false,
        }
    }
    pub fn new_waiter_perm() -> Permission {
        // <- These two functions are the templates
        Self {
            // to create common roles, and their permissions
            table_open: true,
            table_close: true,

            table_create: false,
            table_delete: false,
            table_edit: false,
            menu_item_create: false,
            menu_item_edit: false,
            menu_item_delete: false,
            add_discount: false,
            user_create: false,
            user_edit: false,
            user_delete: false,
            take_pay: false,
        }
    }
    pub fn new_cashier_perm() -> Permission {
        // <- the other one is here
        //TODO: CHANGE NAME OF THE FUNCTION
        Self {
            table_open: true,
            table_close: true,
            take_pay: true,

            table_create: false,
            table_delete: false,
            table_edit: false,
            menu_item_create: false,
            menu_item_edit: false,
            menu_item_delete: false,
            add_discount: true,
            user_create: false,
            user_edit: false,
            user_delete: false,
        }
    }
}

pub struct AppState {
    pub vendor_name: Mutex<String>,
    pub vendor_id: Mutex<u32>,
    pub vendor_owner: Mutex<User>,
    pub main_menu: Mutex<Menu>,
    pub user_list: Mutex<Vec<User>>,
    pub table_list: Mutex<Vec<table::table::Table>>,
    pub item_storage: Mutex<storage::storage::ItemStorage>,
}

pub enum ItemStoreType {
    Count,
    Liters,
    Grams,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::menu::menu::MenuItem;
    use crate::storage::storage::ItemStorage;
    use crate::table::table::Table;
    use actix_web::web;
    use std::collections::BTreeMap;
    
    #[test]
    fn a_lot() {
        let vendor_name = Mutex::new("Burgers".to_string());
        let vendor_id = Mutex::new(2);
        let vendor_owner = Mutex::new(User::new(
            0,
            "BOB".to_string(),
            Permission::new(true, true, true, true, true),
        ));

        let bob = User::new(2, "NOT_BOB".to_string(), Permission::new_waiter_perm());
        let john = User::new(2, "JOHN".to_string(), Permission::new_cashier_perm());
        let user_list = Mutex::new(vec![bob, john]);
        let beer = MenuItem::new(
            "Beer".to_string(),
            "Beer to drink".to_string(),
            ItemStoreType::Count,
            20,
            21,
        );

        let mut menu_list: BTreeMap<String, (MenuItem, u32)> = BTreeMap::new();
        menu_list.insert("Beer".to_string(), (beer, 2655));
        let main_menu = Mutex::new(Menu::new(vec![], menu_list));

        let table_0 = Table::create_table(false, 20, 0);
        let table_1 = Table::create_table(true, 25, 1);
        let table_2 = Table::create_table(true, 5, 2);
        let table_3 = Table::create_table(false, 34, 3);
        let table_list = Mutex::new(vec![table_0, table_1, table_2]);

        let state = web::Data::new(crate::AppState {
            // <- temporary app state to hold
            //    necessary information
            vendor_name,
            vendor_id,
            vendor_owner,
            main_menu,
            user_list,
            table_list,
            item_storage: Mutex::new(ItemStorage {}),
        });

        state.main_menu.lock().unwrap().add_item("Beer".to_string(), 2);
        assert_eq!(
            state.main_menu.lock().unwrap().storage.get("Beer").unwrap().1,
            2657
        );
        state.main_menu.lock().unwrap().remove_item("Beer".to_string(), 20);
        assert_eq!(
            state.main_menu.lock().unwrap().storage.get("Beer").unwrap().1,
            2637
        );state.main_menu.lock().unwrap().remove_item("Beer".to_string(), 252);
        assert_eq!(
            state.main_menu.lock().unwrap().storage.get("Beer").unwrap().1,
            2637-252
        );

        assert_eq!(
            state.vendor_name.lock().unwrap().clone(),
            String::from("Burgers")
        );
        assert_eq!(state.vendor_id.lock().unwrap().clone(), 2);
        assert_eq!(
            state.user_list.lock().unwrap().get(1).unwrap().name,
            "JOHN".to_string()
        );
        assert_eq!(
            state.user_list.lock().unwrap().get(0).unwrap().name,
            "NOT_BOB".to_string()
        );
    }
}
