use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
use crate::domain_layer::functionality::service::getter::Getter;
use crate::infrastructure_layer::functionality::service::date_time_resolver::DateTimeResolver;

pub struct ApplicationUserRegistrationToken_SendingOpportunityResolver;

impl ApplicationUserRegistrationToken_SendingOpportunityResolver {
    pub fn can_send<'a, T>(subject: &'a T) -> bool
    where
        T: Getter<&'a T, ApplicationUserRegistrationToken_CanBeResentFrom, i64>
    {
        return !DateTimeResolver::unixtime_is_greater_or_equal_than_now(
            <T as Getter<&'_ T, ApplicationUserRegistrationToken_CanBeResentFrom, i64>>::get(subject)
        );
    }
}