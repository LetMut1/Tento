use super::Generator;
use crate::{
    domain_layer::data::entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value,
    infrastructure_layer::{
        data::control_type::NumberRow,
        functionality::service::generator::Generator as Generator_,
    },
};
impl Generator<ApplicationUserResetPasswordToken_Value> {
    pub fn generate() -> String {
        return Generator_::<NumberRow>::generate_6();
    }
}
