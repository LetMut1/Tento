use actix_web::web::Bytes;
use actix_http::Payload;
use actix_http::h1::Payload as H1Payload;

pub struct HttpPayloadCreator;

impl HttpPayloadCreator {
    pub fn create_from_data(
        data: Bytes
    ) -> Payload {
        let mut h1_payload: H1Payload = H1Payload::create(true).1;
        h1_payload.unread_data(data);

        return Payload::from(h1_payload);
    }
}