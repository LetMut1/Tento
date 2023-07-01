use super::incrementor::Incrementor;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;

impl Incrementor<ApplicationUserResetPasswordToken_WrongEnterTriesQuantity> {
    pub fn increment<'a>(application_user_reset_password_token_wrong_enter_tries_quantity: &'a mut ApplicationUserResetPasswordToken_WrongEnterTriesQuantity) -> Result<(), ErrorAuditor> {
        let application_user_reset_password_token_wrong_enter_tries_quantity_ = match application_user_reset_password_token_wrong_enter_tries_quantity.get().checked_add(1) {
            Some(application_user_reset_password_token_wrong_enter_tries_quantity_) => application_user_reset_password_token_wrong_enter_tries_quantity_,
            None => {
                return Err(
                    ErrorAuditor::new(
                        BaseError::create_out_of_range(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                            None,
                        ),
                    ),
                );
            }
        };

        *application_user_reset_password_token_wrong_enter_tries_quantity = ApplicationUserResetPasswordToken_WrongEnterTriesQuantity::new(application_user_reset_password_token_wrong_enter_tries_quantity_);

        return Ok(());
    }
}
