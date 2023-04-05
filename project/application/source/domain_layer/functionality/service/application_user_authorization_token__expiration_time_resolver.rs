use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_ExpiresAt;
use crate::domain_layer::functionality::service::getter::Getter;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserAuthorizationToken_ExpirationTimeResolver;

impl ApplicationUserAuthorizationToken_ExpirationTimeResolver {
    pub fn is_expired<'a, T>(subject: &'a T) -> bool
    where
        T: Getter<&'a T, ApplicationUserAuthorizationToken_ExpiresAt, i64>
    {
        return !DateTimeResolver::unixtime_is_greater_or_equal_than_now(
            <T as Getter<&'_ T, ApplicationUserAuthorizationToken_ExpiresAt, i64>>::get(subject)
        );
    }
}