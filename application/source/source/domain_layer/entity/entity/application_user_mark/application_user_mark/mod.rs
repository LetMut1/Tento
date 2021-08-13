use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::channel_feed_publication::_component::id::Id as ChannelFeedPublicationId;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use super::_component::id::Id;
use super::_component::r#type::Type;

pub struct ApplicationUserMark {
    id: Option<Id>,
    application_user_id: ApplicationUserId,
    channel_feed_publication_id: ChannelFeedPublicationId,
    r#type: Type
}

impl ApplicationUserMark {
    pub fn new(
        id: Option<Id>,
        application_user_id: ApplicationUserId,
        channel_feed_publication_id: ChannelFeedPublicationId,
        r#type: Type
    ) -> Self {
        return Self {
            id, application_user_id, channel_feed_publication_id, r#type
        };
    }

    pub fn get_id<'this>(&'this self) -> Result<&'this Id, BaseError> {
        match self.id {
            Some(ref id) => {
                return Ok(id);
            }
            None => {
                return Err(BaseError::LogicError("Id does not exist yet."))
            }
        }
    }

    pub fn get_application_user_id<'this>(&'this self) -> &'this ApplicationUserId {
        return &self.application_user_id;
    }

    pub fn get_channel_feed_publication_id<'this>(&'this self) -> &'this ChannelFeedPublicationId {
        return &self.channel_feed_publication_id;
    }

    pub fn get_type<'this>(&'this self) -> &'this Type {
        return &self.r#type;
    }
}