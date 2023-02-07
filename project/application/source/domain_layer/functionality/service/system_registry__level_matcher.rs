use crate::domain_layer::data::entity::system_registry::Level;

#[allow(non_camel_case_types)]
pub struct SystemRegistry_LevelMatcher;

impl SystemRegistry_LevelMatcher {
    pub fn r#match(level: Level) -> i16 {
        return match level {
            Level::Trace => 0,
            Level::Debug => 1,
            Level::Info => 2,
            Level::Warn => 3,
            Level::Error => 4,
            Level::FatalError => 5
        };
    }
}