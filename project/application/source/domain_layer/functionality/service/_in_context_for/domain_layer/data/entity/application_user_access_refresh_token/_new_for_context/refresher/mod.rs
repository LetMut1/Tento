use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken;
use super::obfuscation_value_generator::ObfuscationValueGenerator;

pub struct Refresher;

impl Refresher {
    pub fn refresh<'a>(
        application_user_access_refresh_token: &'a mut ApplicationUserAccessRefreshToken<'_>
    ) -> () {
        application_user_access_refresh_token.set_obfuscation_value(ObfuscationValueGenerator::generate());

        return ();
    }
}