pub struct Channel {    // TODO НАПРАВЛЕННОСТЬБ
    id: i64,
    application_user_channel_administrator_id: i64,
    name: String,
    personalization_image_path: String,
    description: Option<String>,
    is_private: bool,
    subscribers_quantity: i64,
    public_marks_quantity: i64,
    hidden_marks_quantity: i64,
    reactions_quantity: i64,
    viewing_quantity: i64,
    created_at: Option<String>
}

impl Channel {
    pub fn new(
        id: i64,
        application_user_channel_administrator_id: i64,
        name: String,
        personalization_image_path: String,
        description: Option<String>,
        is_private: bool,
        subscribers_quantity: i64,
        public_marks_quantity: i64,
        hidden_marks_quantity: i64,
        reactions_quantity: i64,
        viewing_quantity: i64,
        created_at: Option<String>
    ) -> Self {
        return Self {
            id,
            application_user_channel_administrator_id,
            name,
            personalization_image_path,
            description,
            is_private,
            subscribers_quantity,
            public_marks_quantity,
            hidden_marks_quantity,
            reactions_quantity,
            viewing_quantity,
            created_at
        };
    }

    pub fn get_id<'a>(&'a self) -> i64 {
        return self.id;
    }

    pub fn get_application_user_channel_administrator_id<'a>(&'a self) -> i64 {
        return self.application_user_channel_administrator_id;
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        return self.name.as_str();
    }

    pub fn get_personalization_image_path<'a>(&'a self) -> &'a str {
        return self.personalization_image_path.as_str();
    }

    pub fn get_description<'a>(&'a self) -> Option<&'a str> {
        match self.description {
            Some(ref description) => {
                return Some(description.as_str());
            }
            None => {
                return None;
            }
        }
    }

    pub fn is_private<'a>(&'a self) -> &'a bool {
        return &self.is_private;
    }

    pub fn get_subscribers_quantity<'a>(&'a self) -> i64 {
        return self.subscribers_quantity;
    }

    pub fn get_public_marks_quantoty<'a>(&'a self) -> i64 {
        return self.public_marks_quantity;
    }

    pub fn get_hidden_marks_quantity<'a>(&'a self) -> i64 {
        return self.hidden_marks_quantity;
    }

    pub fn get_reactions_quantity<'a>(&'a self) -> i64 {
        return self.reactions_quantity;
    }

    pub fn get_viewing_quantity<'a>(&'a self) -> i64 {
        return self.viewing_quantity;
    }

    pub fn get_created_at<'a>(&'a self) -> &'a Option<String> {
        return &self.created_at;
    }
}