use super::Validator;
use crate::{
    domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value,
    infrastructure_layer::data::{
        auditor::{
            Auditor,
            Backtrace,
            OptionConverter,
            ResultConverter,
        },
        error::Error,
    },
};
use regex::Regex;
use std::sync::OnceLock;
static REGULAR_EXPRESSION: OnceLock<Regex> = OnceLock::new();
impl Validator<ApplicationUserResetPasswordToken_Value> {
    pub fn is_valid<'a>(application_user_authorization_token__value: &'a str) -> Result<bool, Auditor<Error>> {
        let regular_expression = match REGULAR_EXPRESSION.get() {
            Some(regular_expression_) => regular_expression_,
            None => {
                if let Err(_) = REGULAR_EXPRESSION.set(
                    Regex::new(ApplicationUserResetPasswordToken_Value::REGULAR_EXPRESSION).convert_into_error(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                ) {
                    return Err(
                        Auditor::<Error>::new(
                            Error::new_logic_value_already_exist(),
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
                REGULAR_EXPRESSION.get().convert_value_does_not_exist(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?
            }
        };
        return Ok(regular_expression.is_match(application_user_authorization_token__value));
    }
}
