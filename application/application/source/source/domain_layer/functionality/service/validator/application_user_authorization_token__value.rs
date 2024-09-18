use super::Validator;
use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use aggregate_error::{
    AggregateError,
    Backtrace,
    Common,
    OptionConverter,
    ResultConverter,
};
use regex::Regex;
use std::sync::OnceLock;
static REGULAR_EXPRESSION: OnceLock<Regex> = OnceLock::new();
impl Validator<ApplicationUserAuthorizationToken_Value> {
    pub fn is_valid<'a>(application_user_authorization_token__value: &'a str) -> Result<bool, AggregateError> {
        let regular_expression = match REGULAR_EXPRESSION.get() {
            Some(regular_expression_) => regular_expression_,
            None => {
                if let Result::Err(_) = REGULAR_EXPRESSION.set(
                    Regex::new(ApplicationUserAuthorizationToken_Value::REGULAR_EXPRESSION).into_logic(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                ) {
                    return Result::Err(
                        AggregateError::new_logic_(
                            Common::ValueAlreadyExist,
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
                REGULAR_EXPRESSION.get().into_logic_value_does_not_exist(
                    Backtrace::new(
                        line!(),
                        file!(),
                    ),
                )?
            }
        };
        return Result::Ok(regular_expression.is_match(application_user_authorization_token__value));
    }
}
