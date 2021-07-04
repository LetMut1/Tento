use crate::domain_layer::utility::_in_context_for::entity::entity::application_user::_core::password::_new_for_context::validator::Validator;

pub struct Password {
    value: String
}

impl Password {
    pub fn new(value: String) -> Self {
        return Self {
            value
        };
    }

    pub fn is_valid<'this>(&'this self) -> bool {
        return Validator::is_valid(self);
    }

    pub fn get_value<'this>(&'this self) -> &'this str {
        return self.value.as_str();
    }
}