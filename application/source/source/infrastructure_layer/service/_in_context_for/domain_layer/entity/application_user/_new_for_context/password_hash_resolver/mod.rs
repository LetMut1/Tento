use crate::domain_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_hash_resolver_trait::PasswordHashResolverTrait;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::application_user::_new_for_context::password_encoder::PasswordEnvoder;

pub struct PasswordHashResolver;

impl PasswordHashResolverTrait for PasswordHashResolver {
    type Error = BaseError;
    type Encoder = PasswordEnvoder;
}