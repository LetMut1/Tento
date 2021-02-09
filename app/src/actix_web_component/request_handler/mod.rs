use actix_web::get;
use actix_web::HttpRequest;
use actix_web::HttpResponse;
use actix_web::Responder;

#[get("/test")]
pub async fn test(req: HttpRequest) -> impl Responder {                     // TODO попробовать создать структуру и к ней статический методы как рхендлеры
    let qwe: String = req.match_info().get("qwe").unwrap().to_string();
    // println!("{:?}", value);
    return HttpResponse::Ok().body(qwe);
}                                                                                  