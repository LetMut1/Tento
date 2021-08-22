use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use super::_component::id::Id;
use super::_component::list_of_members::ListOfMembers;

pub struct ApplicationUserDirectMessage {
    id: Option<Id>,
    list_of_members: ListOfMembers
}

impl ApplicationUserDirectMessage {
    pub fn new(
        id: Option<Id>, list_of_members: ListOfMembers
    ) -> Self {
        return Self {
            id, list_of_members
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

    pub fn get_list_of_members<'this>(&'this self) -> &'this ListOfMembers {
        return &self.list_of_members;
    }
}