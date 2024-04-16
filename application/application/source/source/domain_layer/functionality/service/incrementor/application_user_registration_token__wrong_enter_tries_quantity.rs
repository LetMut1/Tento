use super::Incrementor;
use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
use crate::infrastructure_layer::data::error::BacktracePart;
use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::error::Auditor;

impl Incrementor<ApplicationUserRegistrationToken_WrongEnterTriesQuantity> {
    pub fn increment<'a>(application_user_registration_token_wrong_enter_tries_quantity: &'a mut ApplicationUserRegistrationToken_WrongEnterTriesQuantity) -> Result<(), Auditor<Error>> {
        let application_user_registration_token_wrong_enter_tries_quantity_ = match application_user_registration_token_wrong_enter_tries_quantity.0.checked_add(1) {
            Some(application_user_registration_token_wrong_enter_tries_quantity__) => application_user_registration_token_wrong_enter_tries_quantity__,
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::create_out_of_range(),
                        BacktracePart::new(
                            line!(),
                            file!(),
                        ),
                    ),
                );
            }
        };

        *application_user_registration_token_wrong_enter_tries_quantity = ApplicationUserRegistrationToken_WrongEnterTriesQuantity(application_user_registration_token_wrong_enter_tries_quantity_);

        return Ok(());
    }
}
