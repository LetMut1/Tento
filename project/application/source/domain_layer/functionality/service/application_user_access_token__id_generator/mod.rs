use extern_crate::uuid::Uuid;

pub struct ApplicationUserAccessTokenIdGenerator;

impl ApplicationUserAccessTokenIdGenerator {
    pub fn generate(
    ) -> String {
        return Uuid::new_v4().to_string();
    }
}