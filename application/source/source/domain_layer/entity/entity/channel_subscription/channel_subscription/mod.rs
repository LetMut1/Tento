use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::channel::_component::id::Id as ChannelId;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use super::_component::id::Id;

pub struct ChannelSubscription {
    id: Option<Id>,
    channel_id: ChannelId,
    application_user_id: ApplicationUserId
}

impl ChannelSubscription {
    pub fn new(id: Option<Id>, channel_id: ChannelId, application_user_id: ApplicationUserId) -> Self {
        return Self {
            id, channel_id, application_user_id
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

    pub fn get_channel_id<'this>(&'this self) -> &'this ChannelId {
        return &self.channel_id;
    }
}