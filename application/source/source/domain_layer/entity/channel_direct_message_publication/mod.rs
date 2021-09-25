use crate::domain_layer::error::logic_error::LogicError;

pub struct ChannelDirectMessagePublication {
    id: Option<i64>,
    channel_id: i64,
    application_user_channel_administrator_id: i64,
    content_type: u8,
    content_type_component: String,
    viewing_quantity: i64,
    status: u8,
    visible_from: String,
    delete_on: String,
    created_at: String
}

impl ChannelDirectMessagePublication {
    pub fn new(
        id: Option<i64>,
        channel_id: i64,
        application_user_channel_administrator_id: i64,
        content_type: u8,
        content_type_component: String,
        viewing_quantity: i64,
        status: u8,
        visible_from: String,
        delete_on: String,
        created_at: String
    ) -> Self {
        return Self {
            id,
            channel_id,
            application_user_channel_administrator_id,
            content_type,
            content_type_component,
            viewing_quantity,
            status,
            visible_from,
            delete_on,
            created_at
        };
    }

    pub fn get_id<'a>(
        &'a self
    ) -> Result<&'a i64, LogicError> {
        match self.id {
            Some(ref id) => {
                return Ok(id);
            }
            None => {
                return Err(LogicError::new("Id does not exist yet."))
            }
        }
    }

    pub fn get_channel_id<'a>(
        &'a self
    ) -> &'a i64 {
        return &self.channel_id;
    }

    pub fn get_application_user_channel_administrator_id<'a>(
        &'a self
    ) -> &'a i64 {
        return &self.application_user_channel_administrator_id;
    }

    pub fn get_content_type<'a>(
        &'a self
    ) -> &'a u8 {
        return &self.content_type;
    }

    pub fn get_content_type_component<'a>(
        &'a self
    ) -> &'a str {
        return self.content_type_component.as_str();
    }

    pub fn get_viewing_quantity<'a>(
        &'a self
    ) -> &'a i64 {
        return &self.viewing_quantity;
    }

    pub fn get_status<'a>(
        &'a self
    ) -> &'a u8 {
        return &self.status;
    }

    pub fn get_visible_from<'a>(
        &'a self
    ) -> &'a str {
        return self.visible_from.as_str();
    }

    pub fn get_delete_on<'a>(
        &'a self
    ) -> &'a str {
        return self.delete_on.as_str();
    }

    pub fn get_created_at<'a>(
        &'a self
    ) -> &'a str {
        return self.created_at.as_str();
    }
}