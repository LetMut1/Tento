use crate::domain_layer::entity::json_refresh_web_token::JsonRefreshWebToken;
use super::obfuscation_value_generator_trait::ObfuscationValueGeneratorTrait;

pub trait RefresherTrait {
    type ObfuscationValueGenerator: ObfuscationValueGeneratorTrait;

    fn refresh<'outer_a>(
        json_refresh_web_token: &'outer_a mut JsonRefreshWebToken<'_>
    ) -> () {
        json_refresh_web_token.set_obfuscation_value(
            <Self::ObfuscationValueGenerator as ObfuscationValueGeneratorTrait>::generate()
        );

        return ();
    }
}