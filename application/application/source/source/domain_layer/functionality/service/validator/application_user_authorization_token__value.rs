use super::Validator;
use crate::{
    domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value,
    infrastructure_layer::data::{
        alternative_workflow::{
            AlternativeWorkflow,
            OptionConverter,
            ResultConverter,
        },
        auditor::Backtrace,
    },
};
use regex::Regex;
use std::sync::OnceLock;
static REGULAR_EXPRESSION: OnceLock<Regex> = OnceLock::new();
impl Validator<ApplicationUserAuthorizationToken_Value> {
    pub fn is_valid<'a>(application_user_authorization_token__value: &'a str) -> Result<bool, AlternativeWorkflow> {
        let regular_expression = match REGULAR_EXPRESSION.get() {
            Some(regular_expression_) => regular_expression_,
            None => {
                if let Err(_) = REGULAR_EXPRESSION.set(
                    Regex::new(ApplicationUserAuthorizationToken_Value::REGULAR_EXPRESSION).into_internal_runtime(
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    )?,
                ) {
                    return Err(
                        AlternativeWorkflow::new_internal_error_logic_value_already_exist(
                            Backtrace::new(
                                line!(),
                                file!(),
                            ),
                        ),
                    );
                }
                REGULAR_EXPRESSION.get().into_internal_logic_value_does_not_exist(
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
