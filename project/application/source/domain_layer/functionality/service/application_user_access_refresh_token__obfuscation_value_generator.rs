use extern_crate::uuid::Uuid;

#[allow(non_camel_case_types)]
pub struct ApplicationUserAccessRefreshToken_ObfuscationValueGenerator;

impl ApplicationUserAccessRefreshToken_ObfuscationValueGenerator {
    pub fn generate(
    ) -> String {
        return Uuid::new_v4().to_string();
    }
}