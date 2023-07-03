use super::generator::Generator;
use crate::domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use crate::infrastructure_layer::functionality::service::generator::Generator as Generator_;
use crate::infrastructure_layer::functionality::service::generator::NumberRow;

impl Generator<ApplicationUserResetPasswordToken_Value> {
    pub fn generate() -> ApplicationUserResetPasswordToken_Value {
        return ApplicationUserResetPasswordToken_Value::new(Generator_::<NumberRow>::generate_6());
    }
}
