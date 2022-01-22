use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize)]
#[cfg_attr(feature="facilitate_non_automatic_functional_testing", derive(Deserialize))]
pub struct Channel {
    channel_id: i64,
    channel_name: String,
    channel_personalization_image_path: String,
    channel_subscribers_quantity: i64,
    channel_public_marks_quantity: i64,
    channel_hidden_marks_quantity: i64,
    channel_reactions_quantity: i64,
    channel_viewing_quantity: i64,
    channel_created_at: String
}

impl Channel {
    pub fn new(
        channel_id: i64,
        channel_name: String,
        channel_personalization_image_path: String,
        channel_subscribers_quantity: i64,
        channel_public_marks_quantity: i64,
        channel_hidden_marks_quantity: i64,
        channel_reactions_quantity: i64,
        channel_viewing_quantity: i64,
        channel_created_at: String
    ) -> Self {
        return Self {
            channel_id,
            channel_name,
            channel_personalization_image_path,
            channel_subscribers_quantity,
            channel_public_marks_quantity,
            channel_hidden_marks_quantity,
            channel_reactions_quantity,
            channel_viewing_quantity,
            channel_created_at
        };
    }
}