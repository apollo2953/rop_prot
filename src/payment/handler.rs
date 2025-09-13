use actix_web::{HttpRequest, HttpResponse};

#[actix_web::get("/payment/take_payment")]
pub async fn take_payment(req: HttpRequest) -> HttpResponse {
    todo!()
}
