use crate::domain_layer::entity::entity::application_user::_component::id::Id as ApplicationUserId;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use super::_component::id::Id;

pub struct ApplicationUserSubscription {
    id: Option<Id>,
    publisher_application_user_id: ApplicationUserId,
    subscriber_application_user_id: ApplicationUserId
}

impl ApplicationUserSubscription {
    pub fn new(
        id: Option<Id>, publisher_application_user_id: ApplicationUserId, subscriber_application_user_id: ApplicationUserId
    ) -> Self {
        return Self {
            id, publisher_application_user_id, subscriber_application_user_id
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

    pub fn get_publisher_application_user_id<'this>(&'this self) -> &'this ApplicationUserId {
        return &self.publisher_application_user_id;
    }

    pub fn get_subscriber_application_user_id<'this>(&'this self) -> &'this ApplicationUserId {
        return &self.subscriber_application_user_id;
    }
}