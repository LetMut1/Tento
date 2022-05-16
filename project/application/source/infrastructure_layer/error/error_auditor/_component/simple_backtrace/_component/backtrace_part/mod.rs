use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

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

impl Display for BacktracePart {
    fn fmt<'a, 'b>(
        &'a self,
        _formatter: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}
