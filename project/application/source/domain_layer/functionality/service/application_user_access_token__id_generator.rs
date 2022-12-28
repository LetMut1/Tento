use extern_crate::uuid::Uuid;

#[allow(non_camel_case_types)]
pub struct ApplicationUserAccessToken_IdGenerator;

impl ApplicationUserAccessToken_IdGenerator {
    pub fn generate() -> String {
        return Uuid::new_v4().to_string();
    }
}