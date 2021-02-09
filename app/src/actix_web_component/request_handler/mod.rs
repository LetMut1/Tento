// TODO попробовать создать структуру и к ней статический методы как рхендлеры

use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;
use actix_web::HttpRequest;

#[get("/test")]
pub async fn test(req: HttpRequest) -> impl Responder {
    let qwe: String = req.match_info().get("qwe").unwrap().to_string();
    // println!("{:?}", value);
    return HttpResponse::Ok().body(qwe);
}