use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::domain_layer::entity::entity::channel::_component::id::Id as ChannelId;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use super::_component::created_at::CreatedAt;
use super::_component::id::Id;
use super::_component::public_marks_quantity::PublicMarksQuantity;
use super::_component::value::Value;

pub struct ChannelFeedReaction {
    id: Option<Id>,
    channel_id: ChannelId,
    application_user_id: ApplicationUserId,
    value: Value,
    public_marks_quantity: PublicMarksQuantity,
    created_at: CreatedAt
}

impl ChannelFeedReaction {
    pub fn new(
        id: Option<Id>,
        channel_id: ChannelId,
        application_user_id: ApplicationUserId,
        value: Value,
        public_marks_quantity: PublicMarksQuantity,
        created_at: CreatedAt
    ) -> Self {
        return Self {
            id, channel_id, application_user_id, value, public_marks_quantity, created_at
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

    pub fn get_channel_id<'this>(&'this self) -> &'this ChannelId {
        return &self.channel_id;
    }

    pub fn get_application_user_id<'this>(&'this self) -> &'this ApplicationUserId {
        return &self.application_user_id;
    }

    pub fn get_value<'this>(&'this self) -> &'this Value {
        return &self.value;
    }

    pub fn get_public_marks_quantoty<'this>(&'this self) -> &'this PublicMarksQuantity {
        return &self.public_marks_quantity;
    }

    pub fn get_created_at<'this>(&'this self) -> &'this CreatedAt {
        return &self.created_at;
    }
}