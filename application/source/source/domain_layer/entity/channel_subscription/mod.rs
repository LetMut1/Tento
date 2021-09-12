use crate::infrastructure_layer::error::base_error::base_error::BaseError;

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

    pub fn get_id<'this>(
        &'this self
    ) -> Result<&'this i64, BaseError> {
        match self.id {
            Some(ref id) => {
                return Ok(id);
            }
            None => {
                return Err(BaseError::LogicError("Id does not exist yet."))
            }
        }
    }

    pub fn get_application_user_id<'this>(
        &'this self
    ) -> &'this i64 {
        return &self.application_user_id;
    }

    pub fn get_channel_id<'this>(
        &'this self
    ) -> &'this i64 {
        return &self.channel_id;
    }

    pub fn get_created_at<'this>(
        &'this self
    ) -> &'this str {
        return self.created_at.as_str();
    }
}