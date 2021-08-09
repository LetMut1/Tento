use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use super::_component::created_at::CreatedAt;
use super::_component::description::Description;
use super::_component::hidden_marks_quantity::HiddenMarksQuantity;
use super::_component::id::Id;
use super::_component::is_private::IsPrivate;
use super::_component::name::Name;
use super::_component::owner_application_user_administrator_id::OwnerApplicationUserAdministratorId;
use super::_component::public_marks_quantity::PublicMarksQuantity;
use super::_component::reactions_quantity::ReactionsQuantity;
use super::_component::subscribers_quantity::SubscribersQuantity;

pub struct Channel {
    id: Option<Id>,
    owner_application_user_administrator_id: OwnerApplicationUserAdministratorId,
    name: Name,
    description: Option<Description>,
    is_private: IsPrivate,
    subscribers_quantity: SubscribersQuantity,
    public_marks_quantity: PublicMarksQuantity,
    hidden_marks_quantity: HiddenMarksQuantity,
    reactions_quantity: ReactionsQuantity,
    created_at: CreatedAt
}

impl Channel {
    pub fn new(
        id: Option<Id>,
        owner_application_user_administrator_id: OwnerApplicationUserAdministratorId,
        name: Name,
        description: Option<Description>,
        is_private: IsPrivate,
        subscribers_quantity: SubscribersQuantity,
        public_marks_quantity: PublicMarksQuantity,
        hidden_marks_quantity: HiddenMarksQuantity,
        reactions_quantity: ReactionsQuantity,
        created_at: CreatedAt
    ) -> Self {
        return Self {
            id,
            owner_application_user_administrator_id,
            name,
            description,
            is_private,
            subscribers_quantity,
            public_marks_quantity,
            hidden_marks_quantity,
            reactions_quantity,
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

    pub fn get_owner_application_user_administrator_id<'this>(&'this self) -> &'this OwnerApplicationUserAdministratorId {
        return &self.owner_application_user_administrator_id;
    }

    pub fn get_name<'this>(&'this self) -> &'this Name {
        return &self.name;
    }

    pub fn get_description<'this>(&'this self) -> &'this Option<Description> {
        return &self.description;
    }

    pub fn get_is_private<'this>(&'this self) -> &'this IsPrivate {
        return &self.is_private;
    }

    pub fn get_subscribers_quantity<'this>(&'this self) -> &'this SubscribersQuantity {
        return &self.subscribers_quantity;
    }
    
    pub fn get_public_marks_quantoty<'this>(&'this self) -> &'this PublicMarksQuantity {
        return &self.public_marks_quantity;
    }

    pub fn get_hidden_marks_quantity<'this>(&'this self) -> &'this HiddenMarksQuantity {
        return &self.hidden_marks_quantity;
    }

    pub fn get_reactions_quantity<'this>(&'this self) -> &'this ReactionsQuantity {
        return &self.reactions_quantity;
    }

    pub fn get_created_at<'this>(&'this self) -> &'this CreatedAt {
        return &self.created_at;
    }
}