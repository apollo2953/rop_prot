use actix_web::{HttpRequest, HttpResponse};

#[actix_web::get("/user/create")]
pub async fn create_user(req: HttpRequest) -> HttpResponse {
    todo!()
}
#[actix_web::get("/user/delete")]
pub async fn delete_user(req: HttpRequest) -> HttpResponse {
    todo!()
}

#[actix_web::get("/user/edit")]
pub async fn edit_user(req: HttpRequest) -> HttpResponse {
    todo!()
}