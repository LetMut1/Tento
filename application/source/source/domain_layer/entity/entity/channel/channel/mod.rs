use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub struct Channel {
    id: Option<i64>,
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
        id: Option<i64>,
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

    pub fn get_id<'this>(&'this self) -> Result<i64, BaseError> {
        match self.id {
            Some(id) => {
                return Ok(id);
            }
            None => {
                return Err(BaseError::LogicError("Id does not exist yet."))
            }
        }
    }

    pub fn get_application_user_channel_administrator_id<'this>(&'this self) -> i64 {
        return self.application_user_channel_administrator_id;
    }

    pub fn get_name<'this>(&'this self) -> &'this str {
        return self.name.as_str();
    }

    pub fn get_personalization_image_path<'this>(&'this self) -> &'this str {
        return self.personalization_image_path.as_str();
    }

    pub fn get_description<'this>(&'this self) -> &'this Option<String> {
        return &self.description;
    }

    pub fn get_is_private<'this>(&'this self) -> bool {
        return self.is_private;
    }

    pub fn get_subscribers_quantity<'this>(&'this self) -> i64 {
        return self.subscribers_quantity;
    }
    
    pub fn get_public_marks_quantoty<'this>(&'this self) -> i64 {
        return self.public_marks_quantity;
    }

    pub fn get_hidden_marks_quantity<'this>(&'this self) -> i64 {
        return self.hidden_marks_quantity;
    }

    pub fn get_reactions_quantity<'this>(&'this self) -> i64 {
        return self.reactions_quantity;
    }

    pub fn get_viewing_quantity<'this>(&'this self) -> i64 {
        return self.viewing_quantity;
    }

    pub fn get_created_at<'this>(&'this self) -> Result<&'this str, BaseError> {
        match self.created_at {
            Some(ref created_at) => {
                return Ok(created_at.as_str());
            }
            None => {
                return Err(BaseError::LogicError("Created_at does not exist yet."))
            }
        }
    }
}