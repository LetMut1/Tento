#[allow(dead_code)]
pub struct Channel {
    id: i64,
    application_user_id: i64,
    name: String,
    description: Option<String>,
    is_private: bool,
    orientation: Vec<i16>,
    personalization_image_path: String,
    subscribers_quantity: i64,
    marks_quantity: i64,
    viewing_quantity: i64,
    created_at: String
}

impl Channel {
    pub fn new(
        id: i64,
        application_user_id: i64,
        name: String,
        description: Option<String>,
        is_private: bool,
        orientation: Vec<i16>,
        personalization_image_path: String,
        subscribers_quantity: i64,
        marks_quantity: i64,
        viewing_quantity: i64,
        created_at: String
    ) -> Self {
        return Self {
            id,
            application_user_id,
            name,
            description,
            is_private,
            orientation,
            personalization_image_path,
            subscribers_quantity,
            marks_quantity,
            viewing_quantity,
            created_at
        };
    }
}