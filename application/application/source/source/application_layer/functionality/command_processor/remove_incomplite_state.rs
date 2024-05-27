use crate::infrastructure_layer::data::error::Error;
use crate::infrastructure_layer::data::auditor::Auditor;
use super::CommandProcessor;

pub use crate::infrastructure_layer::data::control_type::RemoveIncompliteState;

impl CommandProcessor<RemoveIncompliteState> {
    pub fn process() -> Result<(), Auditor<Error>> {
        // TODO УДалять из БД состояние с вышедшим сроком экспирации:
        // ApplicationUserRegistrationToken
        // ApplicationUserAuthorizationToken
        // ApplicationUserResetPasswordToken
        // application_user_access_refresh_token   - удалять очень редно, так как нет индекса на поле, по которому будет идти поиск кандидатов.

        todo!();
    }
}