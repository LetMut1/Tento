use super::Validator;
use crate::domain_layer::data::entity::user_reset_password_token::UserResetPasswordToken_Value;
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
impl Validator<UserResetPasswordToken_Value> {
    pub fn is_valid<'a>(user_authorization_token__value: &'a str) -> Result<bool, AggregateError> {
        let regular_expression = match REGULAR_EXPRESSION.get() {
            Option::Some(regular_expression_) => regular_expression_,
            Option::None => {
                if let Result::Err(_) = REGULAR_EXPRESSION.set(
                    Regex::new(UserResetPasswordToken_Value::REGULAR_EXPRESSION).into_logic(
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
        return Result::Ok(regular_expression.is_match(user_authorization_token__value));
    }
}
