use super::Incrementor;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;
use crate::infrastructure_layer::data::auditor::Backtrace;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::OptionConverter;
use crate::infrastructure_layer::data::auditor::Auditor;

impl Incrementor<ApplicationUserAuthorizationToken_WrongEnterTriesQuantity> {
    pub fn increment<'a>(application_user_authorization_token_wrong_enter_tries_quantity: &'a mut ApplicationUserAuthorizationToken_WrongEnterTriesQuantity) -> Result<(), Auditor<Error>> {
        let application_user_authorization_token_wrong_enter_tries_quantity_ = application_user_authorization_token_wrong_enter_tries_quantity.0.checked_add(1).convert_out_of_range(Backtrace::new(line!(), file!()))?;

        *application_user_authorization_token_wrong_enter_tries_quantity = ApplicationUserAuthorizationToken_WrongEnterTriesQuantity(application_user_authorization_token_wrong_enter_tries_quantity_);

        return Ok(());
    }
}
