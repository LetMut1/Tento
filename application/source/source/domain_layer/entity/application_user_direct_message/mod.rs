use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub struct ApplicationUserDirectMessage {
    id: Option<i64>,
    list_of_members: String
}

impl ApplicationUserDirectMessage {
    pub fn new(
        id: Option<i64>,
        list_of_members: String
    ) -> Self {
        return Self {
            id,
            list_of_members
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

    pub fn get_list_of_members<'this>(&'this self) -> &'this str {
        return self.list_of_members.as_str();
    }
}