use serde::Serialize;

#[derive(Serialize)]
pub struct Base {
    #[serde(rename = "ci")]
    channel_id: i64,
    #[serde(rename = "cn")]
    channel_name: String,
    #[serde(rename = "cpip")]
    channel_personalization_image_path: String,
    #[serde(rename = "cd")]
    channel_description: String,
    #[serde(rename = "csq")]
    channel_subscribers_quantity: i64,
    #[serde(rename = "cpmq")]
    channel_public_marks_quantity: i64,
    #[serde(rename = "chmq")]
    channel_hidden_marks_quantity: i64,
    #[serde(rename = "crq")]
    channel_reactions_quantity: i64,
    #[serde(rename = "cvq")]
    channel_viewing_quantity: i64,
    #[serde(rename = "cesocq")]
    channel_entertaining_seeable_only_content_quantity: i64,
    #[serde(rename = "cesahcq")]
    channel_entertaining_seeable_and_hearable_content_quantity: i64,
    #[serde(rename = "cnesocq")]
    channel_non_entertaining_seeable_only_content_quantity: i64,
    #[serde(rename = "cnesahcq")]
    channel_non_entertaining_seeable_and_hearable_content_quantity: i64,
    #[serde(rename = "cca")]
    channel_created_at: String
}

impl Base {
    pub fn new(
        channel_id: i64,
        channel_name: String,
        channel_personalization_image_path: String,
        channel_description: String,
        channel_subscribers_quantity: i64,
        channel_public_marks_quantity: i64,
        channel_hidden_marks_quantity: i64,
        channel_reactions_quantity: i64,
        channel_viewing_quantity: i64,
        channel_entertaining_seeable_only_content_quantity: i64,
        channel_entertaining_seeable_and_hearable_content_quantity: i64,
        channel_non_entertaining_seeable_only_content_quantity: i64,
        channel_non_entertaining_seeable_and_hearable_content_quantity: i64,
        channel_created_at: String
    ) -> Self {
        return Self {
            channel_id,
            channel_name,
            channel_personalization_image_path,
            channel_description,
            channel_subscribers_quantity,
            channel_public_marks_quantity,
            channel_hidden_marks_quantity,
            channel_reactions_quantity,
            channel_viewing_quantity,
            channel_entertaining_seeable_only_content_quantity,
            channel_entertaining_seeable_and_hearable_content_quantity,
            channel_non_entertaining_seeable_only_content_quantity,
            channel_non_entertaining_seeable_and_hearable_content_quantity,
            channel_created_at
        };
    }
}