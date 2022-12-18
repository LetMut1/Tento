use extern_crate::uuid::Uuid;

pub struct IdGenerator;

impl IdGenerator {
    pub fn generate(
    ) -> String {
        return Uuid::new_v4().to_string();
    }
}