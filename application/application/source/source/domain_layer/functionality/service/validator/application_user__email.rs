use super::Validator;
use crate::{
    domain_layer::data::entity::application_user::ApplicationUser_Email,
    infrastructure_layer::data::{
        auditor::{
            Backtrace,
        },
        error::{
            Error,
            OptionConverter,
            ResultConverter,
        },
    },
};
use regex::Regex;
use std::sync::OnceLock;
static REGULAR_EXPRESSION: OnceLock<Regex> = OnceLock::new();
impl Validator<ApplicationUser_Email> {
    pub fn is_valid<'a>(application_user__email: &'a str) -> Result<bool, Error> {
        let regular_expression = match REGULAR_EXPRESSION.get() {
            Some(regular_expression_) => regular_expression_,
            None => {
                if let Err(_) = REGULAR_EXPRESSION.set(
                    Regex::new(ApplicationUser_Email::REGULAR_EXPRESSION).convert_into_error(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                ) {
                    return Err(
                        Error::new_internal_logic_value_already_exist(
                            Backtrace::new(
                                line!(),
                                file!(),
                            )
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
        return Ok(regular_expression.is_match(application_user__email) && application_user__email.chars().count() <= ApplicationUser_Email::MAXIMUM_LENGTH);
    }
}
