use crate::domain_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_new_for_context::password_hash_resolver_trait::PasswordHashResolverTrait;
use crate::infrastructure_layer::service::_in_context_for::domain_layer::entity::entity::application_user::_core::password::_new_for_context::encoder::Encoder;

pub struct PasswordHashResolver;

impl PasswordHashResolverTrait for PasswordHashResolver {
    type Encoder = Encoder;
}