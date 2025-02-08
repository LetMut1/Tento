use {
    super::Generator,
    crate::{
        domain_layer::data::entity::user_reset_password_token::UserResetPasswordToken_Value,
        infrastructure_layer::functionality::service::generator::{
            Generator as Generator_,
            NumberRow,
        },
    },
};
impl Generator<UserResetPasswordToken_Value> {
    pub fn generate() -> String {
        return Generator_::<NumberRow>::generate_6();
    }
}
