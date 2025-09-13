pub mod handler;

use crate::Permission;

struct User {
    is_admin: bool,
    id: u32,
    name: String,
    permission: Permission,
}
impl User {
    fn new_waiter() -> User {
        Self{
            is_admin: false,
            id: 0,
            name: "new waiter".to_string(),
            permission: Permission::new_waiter_perm(),
        }
    }
    fn new_cashier() -> User { //TODO: CHANGE NAME OF THE FUNCTION
        Self{
            is_admin: false,
            id: 2,
            name: "new cashier".to_string(),
            permission: Permission::new_cashier_perm(),
        }
    }
}
