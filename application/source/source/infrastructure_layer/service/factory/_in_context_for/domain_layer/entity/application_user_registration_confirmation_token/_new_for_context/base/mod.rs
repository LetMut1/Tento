use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::domain_layer::service::factory::_in_context_for::domain_layer::entity::application_user_registration_confirmation_token::_new_for_context::base_trait::BaseTrait as ApplicationUserRegistrationConfirmationTokenFactoryTrait;

pub struct Base;

impl ApplicationUserRegistrationConfirmationTokenFactoryTrait for Base {
    type Error = BaseError;
}