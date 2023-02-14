#[allow(dead_code)]
pub struct SystemRegistry {
    message: String,
    level: Level,
    created_at: String
}

pub enum Level {
    /// 0 as integer.
    Trace,
    /// 1 as integer.
    Debug,
    /// 2 as integer.
    Info,
    /// 3 as integer.
    Warn,
    /// 4 as integer.
    Error,
    /// 5 as integer.
    FatalError
}