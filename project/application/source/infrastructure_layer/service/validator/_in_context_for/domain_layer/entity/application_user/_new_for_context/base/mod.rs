use crate::domain_layer::service::validator::_in_context_for::domain_layer::entity::application_user::_new_for_context::base_trait::BaseTrait as ApplicationUserValidatorTrait;
use crate::infrastructure_layer::error::error_aggregator::error_aggregator::ErrorAggregator;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_hash_resolver::PasswordHashResolver;

pub struct Base;

impl ApplicationUserValidatorTrait for Base {
    type Error = ErrorAggregator;
    type PasswordHashResolver = PasswordHashResolver;
}