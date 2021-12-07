use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as ApplicationUserComponentValidatorTrait;

pub struct Base;

impl ApplicationUserComponentValidatorTrait for Base {
    type Error = BaseError;
}