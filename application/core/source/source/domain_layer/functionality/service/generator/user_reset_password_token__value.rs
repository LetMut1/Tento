use {
    super::Generator,
    crate::domain_layer::data::entity::user_reset_password_token::UserResetPasswordToken_Value,
};
#[cfg(not(feature = "token_666666"))]
use crate::infrastructure_layer::functionality::service::generator::{
    Generator as Generator_,
    NumberRow,
};
impl Generator<UserResetPasswordToken_Value> {
    pub fn generate() -> String {
        #[cfg(feature = "token_666666")]
        {
            return "666666".to_string();
        }
        #[cfg(not(feature = "token_666666"))]
        {
            return Generator_::<NumberRow>::generate_6();
        }
    }
}
