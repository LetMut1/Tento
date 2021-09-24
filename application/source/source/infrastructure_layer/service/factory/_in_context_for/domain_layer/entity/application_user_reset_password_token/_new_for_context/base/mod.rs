use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::base_trait::BaseTrait as ApplicationUserResetPasswordTokenFactoryTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user_reset_password_token::_new_for_context::value_generator::ValueGenerator;

pub struct Base;

impl ApplicationUserResetPasswordTokenFactoryTrait for Base {
    type Error = BaseError;
    type ValueGenerator = ValueGenerator;
}