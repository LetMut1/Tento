use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(Debug)]
pub struct BacktracePart {
    line_number: u32,
    file_path: &'static str,
    information: Option<String>
}

impl BacktracePart {
    pub fn new(
        line_number: u32,
        file_path: &'static str,
        information: Option<String>
    ) -> Self {
        return Self {
            line_number,
            file_path,
            information
        }
    }
}

impl Display for BacktracePart {
    fn fmt<'a, 'b>(
        &'a self,
        formatter: &'b mut Formatter<'_>
    ) -> Result {
        return Ok(());
    }
}
