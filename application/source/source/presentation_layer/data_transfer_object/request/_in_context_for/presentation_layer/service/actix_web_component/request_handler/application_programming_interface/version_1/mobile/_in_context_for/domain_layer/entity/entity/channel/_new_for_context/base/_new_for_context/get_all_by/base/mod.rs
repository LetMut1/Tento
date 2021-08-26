use serde::Deserialize;

#[derive(Deserialize)]
pub struct Base {
    #[serde(rename = "l")]
    limit: u8,
    #[serde(rename = "ci")]
    channel_created_at: String,     // TODO  (ПОка что сделал так на пробу) Сделать Через тип (все, что ниже лимита): по дате: по ...
    #[serde(rename = "o")]
    order: u8
}

impl Base {
    pub fn into_inner(self) -> (u8, String, u8) {
        return (
            self.limit,
            self.channel_created_at,
            self.order
        );
    }
}