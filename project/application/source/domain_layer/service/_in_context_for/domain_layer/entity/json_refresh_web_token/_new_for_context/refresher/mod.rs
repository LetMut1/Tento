use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use super::obfuscation_value_generator::ObfuscationValueGenerator;

pub struct Refresher;

impl Refresher {
    pub fn refresh<'a>(
        json_refresh_web_token: &'a mut JsonRefreshWebToken<'_>
    ) -> () {
        json_refresh_web_token.set_obfuscation_value(ObfuscationValueGenerator::generate());

        return ();
    }
}