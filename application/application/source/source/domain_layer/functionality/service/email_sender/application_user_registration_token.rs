use super::EmailSender;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use crate::infrastructure_layer::data::auditor::Auditor;
use crate::infrastructure_layer::data::environment_configuration::EnvironmentConfiguration;
use crate::infrastructure_layer::data::error::Error;

use crate::infrastructure_layer::functionality::service::sender::email::Email;
use crate::infrastructure_layer::functionality::service::sender::Sender;

impl EmailSender<ApplicationUserRegistrationToken<'_>> {
    pub fn send<'a>(
        environment_configuration: &'a EnvironmentConfiguration,
        application_user_registration_token_value: &'a ApplicationUserRegistrationToken_Value,
        application_user_email: &'a str,
        application_user_device_id: &'a ApplicationUserDevice_Id,
    ) -> Result<(), Auditor<Error>> {
        let message_body = format!(
            "Your code {} for device {}.",
            application_user_registration_token_value.0.as_str(),
            application_user_device_id.0.as_str()
        );

        Sender::<Email>::send(
            environment_configuration,
            "Registration confirmation",
            message_body,
            application_user_email,
        )?;

        return Ok(());
    }
}
