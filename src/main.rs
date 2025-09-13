use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, get, middleware, web};
use rop_prot::Permission;
use rop_prot::menu::handler::{add_item, delete_item, edit_item};
use rop_prot::table::handler::{close_table, create_table, delete_table, open_table};
use rop_prot::user::handler::{create_user, delete_user, edit_user};
use rop_prot::util::json::into_user_json;
use std::sync::Mutex;
use rop_prot::payment::handler::take_payment;

struct AppState {
    temp: Mutex<String>,
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


    let counter = web::Data::new(AppState { // <- temporary app state to hold
        temp: Mutex::new("temp".to_string()),                     //    necessary information
    });

    HttpServer::new(move || {

        App::new()
            .wrap(middleware::NormalizePath::default())
            .app_data(counter.clone()) // <- register the created data
            .service( // TODO: add auth middleware to each service
                web::scope("/user")  // <- all of these services redirect to their respected
                    .service(create_user)// paths, and in the handler files
                    .service(delete_user)// it redirects to the subpath
                    .service(edit_user),                      // that involves the service.
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
            .service(
                web::scope("/payment")
                    .service(take_payment)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
