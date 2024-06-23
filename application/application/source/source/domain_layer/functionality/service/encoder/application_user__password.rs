use super::Encoder;
use crate::{
    domain_layer::data::entity::application_user::ApplicationUser_Password,
    infrastructure_layer::{
        data::{
            auditor::Auditor,
            control_type::Argon2Id,
            error::Error,
        },
        functionality::service::encoder::Encoder as Encoder_,
    },
};
impl Encoder<ApplicationUser_Password> {
    pub fn encode<'a>(application_user_password: &'a str) -> Result<String, Auditor<Error>> {
        return Ok(Encoder_::<Argon2Id>::encode(application_user_password.as_bytes())?);
    }
    pub fn is_valid<'a>(application_user_password: &'a str, application_user__password_hash: &'a str) -> Result<bool, Auditor<Error>> {
        return Ok(
            Encoder_::<Argon2Id>::is_valid(
                application_user_password.as_bytes(),
                application_user__password_hash,
            )?,
        );
    }
}
