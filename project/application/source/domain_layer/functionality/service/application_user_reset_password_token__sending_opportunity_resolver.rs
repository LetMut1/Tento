use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken;
use crate::domain_layer::functionality::service::getter::Getter;
use crate::infrastructure_layer::functionality::service::resolver::DateTime;
use crate::infrastructure_layer::functionality::service::resolver::Resolver;
use super::sending_opportunity_resolver::SendingOpportunityResolver;

impl SendingOpportunityResolver<ApplicationUserResetPasswordToken<'_>> {
    pub fn can_send<'a, T>(subject: &'a T) -> bool
    where
        T: Getter<&'a T, ApplicationUserResetPasswordToken_CanBeResentFrom, i64>
    {
        return !Resolver::<DateTime>::unixtime_is_greater_or_equal_than_now(
            <T as Getter<&'_ T, ApplicationUserResetPasswordToken_CanBeResentFrom, i64>>::get(subject)
        );
    }
}