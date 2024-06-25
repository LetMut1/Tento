pub struct Auditor<T> {
    pub subject: T,
    pub backtrace: Backtrace,
}
impl<T> Auditor<T> {
    pub fn new(subject: T, backtrace: Backtrace) -> Self {
        return Self {
            subject,
            backtrace,
        };
    }
}
pub struct Backtrace {
    pub line_number: u32,
    pub file_path: &'static str,
}
impl Backtrace {
    pub fn new(line_number: u32, file_path: &'static str) -> Self {
        return Self {
            line_number,
            file_path,
        };
    }
}