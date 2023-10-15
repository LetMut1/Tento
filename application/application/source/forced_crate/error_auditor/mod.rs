use std::error::Error as StdError;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FormatError;
use std::boxed::Box;
use std::fmt::Formatter as StdFormatter;
use formatter::Format;
use formatter::Formatter;

#[derive(Debug)]
pub struct ErrorAuditor<T> {
    error: Error<T>,
    backtrace: Backtrace,
}

impl<T> ErrorAuditor<T> {
    pub fn new(
        error: Error<T>,
        backtrace_part: BacktracePart,
    ) -> Self {
        return Self {
            error,
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

    pub fn get_error<'a>(&'a self) -> &'a Error<T> {
        return &self.error;
    }

    pub fn get_backtrace<'a>(&'a self) -> &'a Backtrace {
        return &self.backtrace;
    }
}

impl<T> Display for ErrorAuditor<T> {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut StdFormatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}

impl<T> StdError for ErrorAuditor<T>
where
    T: Debug,
{}

impl<T> Format<ErrorAuditor<T>> for Formatter
where
    Formatter: Format<T>,
{
    fn prepare<'a>(subject: &'a ErrorAuditor<T>) -> String {
        let mut backtrace_message = String::new();

        '_a: for (index, backtrace_part) in subject.get_backtrace().get_backtrace_part_registry().iter().enumerate() {
            if index == 0 {
                backtrace_message = match backtrace_part.get_context() {
                    Some(context) => {
                        format!(
                            "({}) {}:{} ({}).\n",
                            index,
                            backtrace_part.get_file_path(),
                            backtrace_part.get_line_number(),
                            context
                        )
                    }
                    None => {
                        format!(
                            "({}) {}:{}.\n",
                            index,
                            backtrace_part.get_file_path(),
                            backtrace_part.get_line_number()
                        )
                    }
                };
            } else {
                backtrace_message = match backtrace_part.get_context() {
                    Some(context) => {
                        format!(
                            "{}({}) {}:{} ({})\n.",
                            backtrace_message.as_str(),
                            index,
                            backtrace_part.get_file_path(),
                            backtrace_part.get_line_number(),
                            context
                        )
                    }
                    None => {
                        format!(
                            "{}({}) {}:{}.\n",
                            backtrace_message.as_str(),
                            index,
                            backtrace_part.get_file_path(),
                            backtrace_part.get_line_number()
                        )
                    }
                }
            };
        }

        let error_message = match *subject.get_error() {
            Error::Logic {
                message,
            } => {
                format!(
                    "Error-->Logic: {}.",
                    message,
                )
            }
            Error::Runtime {
                ref runtime,
            } => {
                let error_message_ = match *runtime {
                    Runtime::Other {
                        ref other,
                    } => {
                        format!(
                            "Error-->Runtime-->Other: {}.",
                            other.get_error(),
                        )
                    }
                    Runtime::Resource {
                        ref resource,
                    } => {
                        let message = <Formatter as Format<T>>::prepare(resource);

                        let error_message__ = if !message.is_empty() {
                            format!(
                                "Error-->Runtime-->Resource: {}.",
                                message.as_str(),
                            )
                        } else {
                            "Error-->Runtime-->Resource: ???. It's probably better to tell exactly what happened...".to_string()
                        };

                        error_message__
                    }
                };

                error_message_
            }
        };

        return format!(
            "{} > {}",
            backtrace_message.as_str(),
            error_message.as_str()
        );
    }
}

#[derive(Debug)]
pub enum Error<T> {
    Logic {
        message: &'static str,
    },
    Runtime {
        runtime: Runtime<T>,
    },
}

impl<T> Error<T> {
    pub fn create_incoming_invalid_state() -> Self {
        return Self::Logic {
            message: "The action processor Incoming in invalid state.",
        };
    }

    pub fn create_unreachable_state() -> Self {
        return Self::Logic {
            message: "Unreachable state.",
        };
    }

    pub fn create_out_of_range() -> Self {
        return Self::Logic {
            message: "Out of range.",
        };
    }
}

impl<T> Display for Error<T> {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut StdFormatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub enum Runtime<T> {
    Other {
        other: Other,
    },
    Resource {
        resource: T,
    },
}

impl<T> Display for Runtime<T> {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut StdFormatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct Other {
    error: Box<dyn StdError + Send + Sync + 'static>,
}

impl Other {
    pub fn new<E>(error: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        return Self {
            error: error.into()
        };
    }

    pub fn new_(error: Box<dyn StdError + Send + Sync + 'static>) -> Self {
        return Self {
            error
        };
    }

    pub fn get_error<'a>(&'a self) -> &'a (dyn StdError + 'static) {
        return self.error.as_ref();
    }
}

impl Display for Other {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut StdFormatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
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

impl Display for Backtrace {
    fn fmt<'a, 'b>(
        &'a self,
        _: &'b mut StdFormatter<'_>,
    ) -> Result<(), FormatError> {
        return Ok(());
    }
}

#[derive(Debug)]
pub struct BacktracePart {
    line_number: u32,
    file_path: &'static str,
    context: Option<String>,
}

impl BacktracePart {
    pub fn new(
        line_number: u32,
        file_path: &'static str,
        context: Option<String>,
    ) -> Self {
        return Self {
            line_number,
            file_path,
            context,
        };
    }

    pub fn get_line_number<'a>(&'a self) -> u32 {
        return self.line_number;
    }

    pub fn get_file_path<'a>(&'a self) -> &'static str {
        return self.file_path;
    }

    pub fn get_context<'a>(&'a self) -> Option<&'a str> {
        return self.context.as_deref();
    }
}
