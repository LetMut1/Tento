use crate::domain_layer::entity::entity::application_user_registration_confirmation_token::application_user_registration_confirmation_token::ApplicationUserRegistrationConfirmationToken;
use crate::domain_layer::entity::entity::pre_confirmed_application_user::_component::id::Id as PreConfirmedApplicationUserId;
use crate::infrastructure_layer::error::base_error::base_error::BaseError;
use redis::Connection;

pub trait BaseRepositoryTrait {
    fn create<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_registration_confirmation_token: &'outer_a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), BaseError>;

    fn delete<'outer_a>(
        connection: &'outer_a mut Connection, 
        application_user_registration_confirmation_token: &'outer_a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), BaseError>;

    fn update_expiration_time<'outer_a>(
        connection: &'outer_a mut Connection,
        application_user_registration_confirmation_token: &'outer_a ApplicationUserRegistrationConfirmationToken<'_>
    ) -> Result<(), BaseError>;

    fn get_by_pre_confirmed_application_user_id<'outer_a, 'outer_b>(
        connection: &'outer_a mut Connection, pre_confirmed_application_user_id: &'outer_b PreConfirmedApplicationUserId
    ) -> Result<Option<ApplicationUserRegistrationConfirmationToken<'outer_b>>, BaseError>;
}