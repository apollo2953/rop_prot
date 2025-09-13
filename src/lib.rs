pub mod menu;
pub mod user;
pub mod util;
pub mod table;
pub mod payment;

pub struct Permission { // <- the struct to check for permissions on each user.
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
    pub fn new_waiter_perm() -> Permission { // <- These two functions are the templates 
        Self {                               // to create common roles, and their permissions
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
    pub fn new_cashier_perm() -> Permission { // <- the other one is here
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
