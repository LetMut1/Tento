use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub struct ChannelFeedPublicationMark {
    id: Option<i64>,
    application_user_id: i64,
    channel_feed_publication_id: i64,
    r#type: u8
}

impl ChannelFeedPublicationMark {
    pub fn new(
        id: Option<i64>,
        application_user_id: i64,
        channel_feed_publication_id: i64,
        r#type: u8
    ) -> Self {
        return Self {
            id,
            application_user_id,
            channel_feed_publication_id,
            r#type
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

    pub fn get_channel_feed_publication_id<'this>(
        &'this self
    ) -> &'this i64 {
        return &self.channel_feed_publication_id;
    }

    pub fn get_type<'this>(
        &'this self
    ) -> &'this u8 {
        return &self.r#type;
    }
}