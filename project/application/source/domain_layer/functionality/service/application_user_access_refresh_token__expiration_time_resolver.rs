use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
use crate::domain_layer::functionality::service::getter::Getter;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserAccessRefreshToken_ExpirationTimeResolver;

impl ApplicationUserAccessRefreshToken_ExpirationTimeResolver {
    pub fn is_expired<'a, T>(subject: &'a T) -> bool
    where
        T: Getter<&'a T, ApplicationUserAccessRefreshToken_ExpiresAt, i64>
    {
        return !DateTimeResolver::unixtime_is_greater_or_equal_than_now(
            <T as Getter<&'_ T, ApplicationUserAccessRefreshToken_ExpiresAt, i64>>::get(subject)
        );
    }
}