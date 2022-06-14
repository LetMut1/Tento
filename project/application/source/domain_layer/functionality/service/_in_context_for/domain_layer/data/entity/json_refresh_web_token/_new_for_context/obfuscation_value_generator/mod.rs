use uuid::Uuid;

pub struct ObfuscationValueGenerator;

impl ObfuscationValueGenerator {
    pub fn generate(
    ) -> String {
        return Uuid::new_v4().to_string();
    }
}