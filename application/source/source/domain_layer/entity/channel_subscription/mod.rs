use crate::domain_layer::error::logic_error::LogicError;

pub struct ChannelSubscription {
    id: Option<i64>,
    channel_id: i64,
    application_user_id: i64,
    created_at: String
}

impl ChannelSubscription {
    pub fn new(
        id: Option<i64>,
        channel_id: i64,
        application_user_id: i64,
        created_at: String
    ) -> Self {
        return Self {
            id,
            channel_id,
            application_user_id,
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

    pub fn get_application_user_id<'a>(
        &'a self
    ) -> &'a i64 {
        return &self.application_user_id;
    }

    pub fn get_channel_id<'a>(
        &'a self
    ) -> &'a i64 {
        return &self.channel_id;
    }

    pub fn get_created_at<'a>(
        &'a self
    ) -> &'a str {
        return self.created_at.as_str();
    }
}