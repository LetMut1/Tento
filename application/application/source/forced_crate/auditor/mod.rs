use error::Error;
use std::error::Error as StdError;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;

pub struct Auditor<T> {
    subject: T,
    backtrace: Backtrace,
}

impl<T> Auditor<T> {
    pub fn new(
        subject: T,
        backtrace_part: BacktracePart,
    ) -> Self {
        return Self {
            subject,
            backtrace: Backtrace::new(backtrace_part),
        };
    }

    pub fn add_backtrace_part<'a>(
        &'a mut self,
        backtrace_part: BacktracePart,
    ) -> () {
        self.backtrace.add(backtrace_part);

        return ();
    }

    pub fn get_subject<'a>(&'a self) -> &'a T {
        return &self.subject;
    }

    pub fn get_backtrace<'a>(&'a self) -> &'a Backtrace {
        return &self.backtrace;
    }
}

impl Debug for Auditor<Error> {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>,
    ) -> Result<(), FmtError> {
        return Ok(());
    }
}

impl Display for Auditor<Error> {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut Formatter<'_>,
    ) -> Result<(), FmtError> {
        return Ok(());
    }
}

impl StdError for Auditor<Error> {}

pub struct Backtrace {
    backtrace_part_registry: Vec<BacktracePart>,
}

impl Backtrace {
    pub fn new(backtrace_part: BacktracePart) -> Self {
        return Self {
            backtrace_part_registry: vec![backtrace_part],
        };
    }

    pub fn add<'a>(
        &'a mut self,
        backtrace_part: BacktracePart,
    ) -> () {
        self.backtrace_part_registry.push(backtrace_part);

        return ();
    }

    pub fn get_backtrace_part_registry<'a>(&'a self) -> &'a [BacktracePart] {
        return self.backtrace_part_registry.as_slice();
    }
}

pub struct BacktracePart {
    line_number: u32,
    file_path: &'static str,
}

impl BacktracePart {
    pub fn new(
        line_number: u32,
        file_path: &'static str,
    ) -> Self {
        return Self {
            line_number,
            file_path,
        };
    }

    pub fn get_line_number<'a>(&'a self) -> u32 {
        return self.line_number;
    }

    pub fn get_file_path<'a>(&'a self) -> &'static str {
        return self.file_path;
    }
}
