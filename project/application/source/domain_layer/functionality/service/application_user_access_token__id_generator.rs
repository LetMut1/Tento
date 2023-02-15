use extern_crate::uuid::Uuid;

pub struct ApplicationUserAccessToken_IdGenerator;

impl ApplicationUserAccessToken_IdGenerator {
    pub fn generate() -> String {
        return Uuid::new_v4().to_string();
    }
}