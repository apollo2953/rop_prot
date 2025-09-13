use actix_web::{App, HttpServer, middleware, web};
use rop_prot::menu::handler::{add_item, delete_item, edit_item};
use rop_prot::menu::menu::MenuItem;
use rop_prot::payment::handler::take_payment;
use rop_prot::storage::storage::ItemStorage;
use rop_prot::table::handler::{close_table, create_table, delete_table, open_table};
use rop_prot::table::table;
use rop_prot::table::table::Table;
use rop_prot::user::handler::{create_user, delete_user, edit_user};
use rop_prot::user::user::User;
use rop_prot::{AppState, ItemStoreType, Menu, Permission};
use std::collections::BTreeMap;
use std::sync::Mutex;

fn temp_init() -> web::Data<AppState> {
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
        ItemStoreType::Liters,
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

    web::Data::new(rop_prot::AppState {
        // <- temporary app state to hold
        //    necessary information
        vendor_name,
        vendor_id,
        vendor_owner,
        main_menu,
        user_list,
        table_list,
        item_storage: Mutex::new(ItemStorage {}),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // todo: add an initialization step, to create app state where you:
    //          1. initialize the app state
    //              1. get user list
    //              2. get menu
    //              3. get table list
    //          2. check for write/read perms for the database
    //          3. check for internet connection and firewall

    // let counter =

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::NormalizePath::default())
            // .app_data(counter.clone()) // <- register the created data
            .service(
                // TODO: add auth middleware to each service
                web::scope("/user") // <- all of these services redirect to their respected
                    .service(create_user) // paths, and in the handler files
                    .service(delete_user) // it redirects to the subpath
                    .service(edit_user), // that involves the service.
            )
            .service(
                web::scope("/table")
                    .service(create_table)
                    .service(delete_table)
                    .service(open_table)
                    .service(close_table),
            )
            .service(
                web::scope("/menu")
                    .service(add_item)
                    .service(delete_item)
                    .service(edit_item),
            )
            .service(web::scope("/payment").service(take_payment))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
