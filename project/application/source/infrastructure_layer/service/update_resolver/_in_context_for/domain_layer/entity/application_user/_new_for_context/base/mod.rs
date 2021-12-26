use crate::domain_layer::service::update_resolver::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as UpdateResolverApplicationUserTrait;
use crate::infrastructure_layer::error::base_error::_component::logic_error::LogicError;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;

pub struct Base {
    update_email: bool,
    update_nickname: bool,
    update_password_hash: bool
}

impl Base {
    pub fn new(
        update_email: bool,
        update_nickname: bool,
        update_password_hash: bool
    ) -> Result<Self, BaseError> {
        if update_email || update_nickname || update_password_hash {
            return  Ok(
                Self {
                    update_email,
                    update_nickname,
                    update_password_hash
                }
            );
        }

        return Err(BaseError::LogicError {logic_error: LogicError::new(false, "The columns allowing update should exist for ApplicationUser.")})
    }
}

impl UpdateResolverApplicationUserTrait for Base {
    fn is_update_email<'a>(
        &'a self
    ) -> bool {
        return self.update_email;
    }

    fn is_update_nickname<'a>(
        &'a self
    ) -> bool {
        return self.update_nickname;
    }

    fn is_update_password_hash<'a>(
        &'a self
    ) -> bool {
        return self.update_password_hash;
    }
}