use crate::Permission;
use serde_json::{Result, Value, json};
//
// struct user_db  {
//     name: String,
//     age: u8,
//     permissions: Permission,
// }

pub fn into_user_json(permissions: Permission) -> Result<Value> {
    let data = json!({
        "name" : "john",
        "id" : 2,
        "table_create": permissions.table_create,
        "table_delete": permissions.table_delete,
        "table_edit": permissions.table_edit,
        "table_open": permissions.table_open,
        "table_close": permissions.table_close,
        "menu_item_create": permissions.menu_item_create,
        "menu_item_delete": permissions.menu_item_delete, 
        "add_discount": permissions.add_discount,
        "user_create": permissions.user_create,
        "user_edit": permissions.user_edit,
        "user_delete": permissions.user_delete,
        "take_pay": permissions.take_pay,
    });
    return Ok(data);
}
