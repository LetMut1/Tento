use super::CommandProcessor;
use crate::infrastructure_layer::data::control_type::RemoveIncompliteState;
use aggregate_error::AggregateError;
impl CommandProcessor<RemoveIncompliteState> {
    pub fn process() -> Result<(), AggregateError> {
        // TODO УДалять из БД состояние с вышедшим сроком экспирации:
        // ApplicationUserRegistrationToken
        // ApplicationUserAuthorizationToken
        // ApplicationUserResetPasswordToken
        // application_user_access_refresh_token   - удалять очень редно, так как нет индекса на поле, по которому будет идти поиск кандидатов.
        todo!();
    }
}
