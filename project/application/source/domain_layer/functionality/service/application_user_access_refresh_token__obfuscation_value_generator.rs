use extern_crate::uuid::Uuid;

pub struct ApplicationUserAccessRefreshToken_ObfuscationValueGenerator;

impl ApplicationUserAccessRefreshToken_ObfuscationValueGenerator {
    pub fn generate() -> String {
        return Uuid::new_v4().to_string();
    }
}