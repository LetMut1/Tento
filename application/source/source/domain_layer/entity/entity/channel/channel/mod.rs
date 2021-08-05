use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use std::borrow::Cow;
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

pub struct ApplicationUser<'outer_a> {
    id: Option<Id>,
    owner_application_user_administrator_id: Cow<'outer_a, OwnerApplicationUserAdministratorId>,
    name: Name,
    description: Description,
    is_private: IsPrivate,
    subscribers_quantity: SubscribersQuantity,
    public_marks_quantity: PublicMarksQuantity,
    hidden_marks_quantity: HiddenMarksQuantity,
    reactions_quantity: ReactionsQuantity,
    created_at: CreatedAt
}

impl<'outer_a> ApplicationUser<'outer_a> {
    pub fn new(
        id: Option<Id>,
        owner_application_user_administrator_id: Cow<'outer_a, OwnerApplicationUserAdministratorId>,
        name: Name,
        description: Description,
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

    // pub fn set_email<'this>(&'this mut self, email: Email) -> &'this mut Self {
    //     self.email = Cow::Owned(email);

    //     return self;
    // }

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

    // pub fn get_email<'this>(&'this self) -> &'this Email {
    //     return self.email.as_ref();
    // }
}