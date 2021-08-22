use crate::domain_layer::entity::entity::application_user_direct_message::_component::id::Id as ApplicationUserDirectMessageId;
use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::channel_feed_publication_reaction::_component::id::Id as ChannelFeedPublicationReactionId;
use crate::domain_layer::entity::entity::channel_feed_publication::_component::id::Id as ChannelFeedPublicationId;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use super::_component::created_at::CreatedAt;
use super::_component::id::Id;


pub struct ApplicationUserDirectMessagePublication {
    id: Option<Id>,
    application_user_direct_message_id: ApplicationUserDirectMessageId,
    application_user_id: ApplicationUserId,
    channel_feed_publication_id: ChannelFeedPublicationId,
    channel_feed_publication_reaction_id: ChannelFeedPublicationReactionId,
    created_at: CreatedAt
}

impl ApplicationUserDirectMessagePublication {
    pub fn new(
        id: Option<Id>,
        application_user_direct_message_id: ApplicationUserDirectMessageId,
        application_user_id: ApplicationUserId,
        channel_feed_publication_id: ChannelFeedPublicationId,
        channel_feed_publication_reaction_id: ChannelFeedPublicationReactionId,
        created_at: CreatedAt
    ) -> Self {
        return Self {
            id,
            application_user_direct_message_id,
            application_user_id,
            channel_feed_publication_id,
            channel_feed_publication_reaction_id,
            created_at
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

    pub fn get_application_user_direct_message_id<'this>(&'this self) -> &'this ApplicationUserDirectMessageId {
        return &self.application_user_direct_message_id;
    }

    pub fn get_application_user_id<'this>(&'this self) -> &'this ApplicationUserId {
        return &self.application_user_id;
    }

    pub fn get_channel_feed_publication_id<'this>(&'this self) -> &'this ChannelFeedPublicationId {
        return &self.channel_feed_publication_id;
    }

    pub fn get_channel_feed_publication_reaction_id<'this>(&'this self) -> &'this ChannelFeedPublicationReactionId {
        return &self.channel_feed_publication_reaction_id;
    }

    pub fn get_created_at<'this>(&'this self) -> &'this CreatedAt {
        return &self.created_at;
    }
}