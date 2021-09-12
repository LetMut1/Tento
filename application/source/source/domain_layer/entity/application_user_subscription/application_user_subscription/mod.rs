use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub struct ApplicationUserSubscription {
    id: Option<i64>,
    publisher_application_user_id: i64,
    subscriber_application_user_id: i64
}

impl ApplicationUserSubscription {
    pub fn new(
        id: Option<i64>,
        publisher_application_user_id: i64,
        subscriber_application_user_id: i64
    ) -> Self {
        return Self {
            id,
            publisher_application_user_id,
            subscriber_application_user_id
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

    pub fn get_publisher_application_user_id<'this>(&'this self) -> &'this i64 {
        return &self.publisher_application_user_id;
    }

    pub fn get_subscriber_application_user_id<'this>(&'this self) -> &'this i64 {
        return &self.subscriber_application_user_id;
    }
}