use extern_crate::uuid::Uuid;

pub struct ApplicationUserAccessRefreshTokenObfuscationValueGenerator;

impl ApplicationUserAccessRefreshTokenObfuscationValueGenerator {
    pub fn generate(
    ) -> String {
        return Uuid::new_v4().to_string();
    }
}