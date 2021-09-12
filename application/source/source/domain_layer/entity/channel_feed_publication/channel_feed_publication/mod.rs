use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub struct ChannelFeedPublication {
    id: Option<i64>,
    channel_id: i64,
    application_user_channel_administrator_id: i64,
    content_type: u8,
    content_type_component: String,
    content_type_component_preview: String,
    public_marks_quantity: i64,
    hidden_marks_quantity: i64,
    reactions_quantity: i64,
    viewing_quantity: i64,
    status: u8,
    visible_from: String,
    delete_on: Option<String>,
    created_at: String

}

impl ChannelFeedPublication {
    pub fn new(
        id: Option<i64>,
        channel_id: i64,
        application_user_channel_administrator_id: i64,
        content_type: u8,
        content_type_component: String,
        content_type_component_preview: String,
        public_marks_quantity: i64,
        hidden_marks_quantity: i64,
        reactions_quantity: i64,
        viewing_quantity: i64,
        status: u8,
        visible_from: String,
        delete_on: Option<String>,
        created_at: String
    ) -> Self {
        return Self {
            id,
            channel_id,
            application_user_channel_administrator_id,
            content_type,
            content_type_component,
            content_type_component_preview,
            public_marks_quantity,
            hidden_marks_quantity,
            reactions_quantity,
            viewing_quantity,
            status,
            visible_from,
            delete_on,
            created_at
        };
    }

    pub fn get_id<'this>(&'this self) -> Result<&'this i64, BaseError> {
        match self.id {
            Some(ref id) => {
                return Ok(id);
            }
            None => {
                return Err(BaseError::LogicError("Id does not exist yet."))
            }
        }
    }

    pub fn get_channel_id<'this>(&'this self) -> &'this i64 {
        return &self.channel_id;
    }

    pub fn get_application_user_channel_administrator_id<'this>(&'this self) -> &'this i64 {
        return &self.application_user_channel_administrator_id;
    }

    pub fn get_content_type<'this>(&'this self) -> &'this u8 {
        return &self.content_type;
    }

    pub fn get_content_type_component<'this>(&'this self) -> &'this str {
        return self.content_type_component.as_str();
    }

    pub fn get_content_type_component_preview<'this>(&'this self) -> &'this str {
        return self.content_type_component_preview.as_str();
    }

    pub fn get_public_marks_quantoty<'this>(&'this self) -> &'this i64 {
        return &self.public_marks_quantity;
    }

    pub fn get_hidden_marks_quantity<'this>(&'this self) -> &'this i64 {
        return &self.hidden_marks_quantity;
    }

    pub fn get_reactions_quantity<'this>(&'this self) -> &'this i64 {
        return &self.reactions_quantity;
    }

    pub fn get_viewing_quantity<'this>(&'this self) -> &'this i64 {
        return &self.viewing_quantity;
    }

    pub fn get_status<'this>(&'this self) -> &'this u8 {
        return &self.status;
    }

    pub fn get_visible_from<'this>(&'this self) -> &'this str {
        return self.visible_from.as_str();
    }

    pub fn get_delete_on<'this>(&'this self) -> &'this Option<String> {
        return &self.delete_on;
    }

    pub fn get_created_at<'this>(&'this self) -> &'this str {
        return self.created_at.as_str();
    }
}