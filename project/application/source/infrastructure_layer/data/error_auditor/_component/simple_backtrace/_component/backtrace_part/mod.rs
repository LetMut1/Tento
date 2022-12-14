#[derive(Debug)]
pub struct BacktracePart {
    line_number: u32,
    file_path: &'static str,
    context: Option<String>
}

impl BacktracePart {
    pub fn new(
        line_number: u32,
        file_path: &'static str,
        context: Option<String>
    ) -> Self {
        return Self {
            line_number,
            file_path,
            context
        }
    }
}