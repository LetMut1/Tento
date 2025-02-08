use {
    crate::{
        domain_layer::data::entity::user_authorization_token::UserAuthorizationToken_Value,
        infrastructure_layer::data::aggregate_error::AggregateError,
    },
    super::Validator,
    regex::Regex,
    std::sync::OnceLock,
};
static REGULAR_EXPRESSION: OnceLock<Regex> = OnceLock::new();
impl Validator<UserAuthorizationToken_Value> {
    pub fn is_valid<'a>(user_authorization_token__value: &'a str) -> Result<bool, AggregateError> {
        let regular_expression = match REGULAR_EXPRESSION.get() {
            Option::Some(regular_expression_) => regular_expression_,
            Option::None => {
                if REGULAR_EXPRESSION
                    .set(
                        crate::result_return_logic!(
                            Regex::new(UserAuthorizationToken_Value::REGULAR_EXPRESSION)
                        ),
                    )
                    .is_err()
                {
                    return Result::Err(crate::new_logic_value_already_exist!());
                }
                crate::option_return_logic_value_does_not_exist!(REGULAR_EXPRESSION.get())
            }
        };
        return Result::Ok(regular_expression.is_match(user_authorization_token__value));
    }
}
