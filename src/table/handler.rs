use actix_web::{HttpRequest, HttpResponse};

#[actix_web::get("/table/create")]
pub async fn create_table(req: HttpRequest) -> HttpResponse {
    todo!()
}

#[actix_web::get("/table/delete")]
pub async fn delete_table(req: HttpRequest) -> HttpResponse {
    todo!()
}

#[actix_web::get("/table/open")]
pub async fn open_table(req: HttpRequest) -> HttpResponse {
    todo!()
}

#[actix_web::get("/table/close")]
pub async fn close_table(req: HttpRequest) -> HttpResponse {
    todo!()
}
