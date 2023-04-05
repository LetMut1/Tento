use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
use crate::domain_layer::functionality::service::getter::Getter;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserAuthorizationToken_SendingOpportunityResolver;

impl ApplicationUserAuthorizationToken_SendingOpportunityResolver {
    pub fn can_send<'a, T>(subject: &'a T) -> bool
    where
        T: Getter<&'a T, ApplicationUserAuthorizationToken_CanBeResentFrom, i64>
    {
        return !DateTimeResolver::unixtime_is_greater_or_equal_than_now(
            <T as Getter<&'a T, ApplicationUserAuthorizationToken_CanBeResentFrom, i64>>::get(subject)
        );
    }
}