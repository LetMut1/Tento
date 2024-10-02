use super::CommandProcessor;
use aggregate_error::AggregateError;
pub struct RemoveIncompliteState;
impl CommandProcessor<RemoveIncompliteState> {
    pub fn process() -> Result<(), AggregateError> {
        // TODO УДалять из БД состояние с вышедшим сроком экспирации:
        // UserRegistrationToken
        // UserAuthorizationToken
        // UserResetPasswordToken
        // user_access_refresh_token   - удалять очень редно, так как нет индекса на поле, по которому будет идти поиск кандидатов.
        todo!();
    }
}
