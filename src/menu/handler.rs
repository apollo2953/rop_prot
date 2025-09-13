use actix_web::{HttpRequest, HttpResponse};

#[actix_web::get("/menu/add_item")]
pub async fn add_item(req: HttpRequest) -> HttpResponse {
    todo!()
}

#[actix_web::get("/menu/delete_item")]
pub async fn delete_item(req: HttpRequest) -> HttpResponse {
    todo!()
}

#[actix_web::get("/menu/edit_item")]
pub async fn edit_item(req: HttpRequest) -> HttpResponse {
    todo!()
}
