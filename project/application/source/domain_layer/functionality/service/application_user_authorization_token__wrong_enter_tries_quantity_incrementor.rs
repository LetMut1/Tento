use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken;
use crate::infrastructure_layer::data::error_auditor::BacktracePart;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::functionality::service::counter::Counter;

pub struct ApplicationUserAuthorizationToken_WrongEnterTriesQuantityIncrementor;

impl ApplicationUserAuthorizationToken_WrongEnterTriesQuantityIncrementor {
    const STEP_SIZE: i16 = 1;

    pub fn increment<'a>(application_user_authorization_token: &'a mut ApplicationUserAuthorizationToken<'_>) -> Result<(), ErrorAuditor> {
        let mut counter = Counter::new(application_user_authorization_token.get_wrong_enter_tries_quantity(), Self::STEP_SIZE);

        let value = match counter.get_next_value() {
            Ok(value_) => value_,
            Err(mut error) => {
                error.add_backtrace_part(BacktracePart::new(line!(), file!(), None));

                return Err(error);
            }
        };

        application_user_authorization_token.set_wrong_enter_tries_quantity(value);

        return Ok(());
    }
}