use std::{
    error::Error as StdError,
    fmt::{
        Debug,
        Display,
        Error as FmtError,
        Formatter,
    },
    marker::PhantomData,
};
pub struct AggregateError(pub Auditor<AggregateError_>);
impl AggregateError {
    pub fn new_indefinite_argument(error: Box<dyn StdError + Send + Sync + 'static>, backtrace: Backtrace) -> Self {
        return Self(
            Auditor {
                subject: AggregateError_::IndefiniteArgument {
                    indefinite_argument_context: Context {
                        subject: PhantomData,
                        error,
                    },
                },
                backtrace,
            },
        );
    }
    pub fn new_invalid_argument(backtrace: Backtrace) -> Self {
        return Self(
            Auditor {
                subject: AggregateError_::InvalidArgument {
                    invalid_argument: PhantomData,
                },
                backtrace,
            },
        );
    }
    pub fn new_logic(error: Box<dyn StdError + Send + Sync + 'static>, backtrace: Backtrace) -> Self {
        return Self(
            Auditor {
                subject: AggregateError_::Logic {
                    logic_context: Context {
                        subject: PhantomData,
                        error,
                    },
                },
                backtrace,
            },
        );
    }
    pub fn new_logic_(common: Common, backtrace: Backtrace) -> Self {
        return Self(
            Auditor {
                subject: AggregateError_::Logic {
                    logic_context: Context {
                        subject: PhantomData,
                        error: common.into(),
                    },
                },
                backtrace,
            },
        );
    }
    pub fn new_runtime(error: Box<dyn StdError + Send + Sync + 'static>, backtrace: Backtrace) -> Self {
        return Self(
            Auditor {
                subject: AggregateError_::Runtime {
                    runtime_context: Context {
                        subject: PhantomData,
                        error,
                    },
                },
                backtrace,
            },
        );
    }
}
impl Debug for AggregateError {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FmtError> {
        return Ok(());
    }
}
impl Display for AggregateError {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FmtError> {
        return Ok(());
    }
}
impl StdError for AggregateError {}
pub struct Auditor<T> {
    pub subject: T,
    pub backtrace: Backtrace,
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
pub enum AggregateError_ {
    // Used for methods that should always return only one of 'Ok' or 'Err' at different
    // times with the same set of parameters and in method scope it is impossible to
    // understand if this is a programmer error or invalid input parameters.
    IndefiniteArgument {
        indefinite_argument_context: Context<PhantomData<IndefiniteArgument>>,
    },
    // Used for invalid user input.
    InvalidArgument {
        invalid_argument: PhantomData<InvalidArgument>,
    },
    // Used for errors that should never be thrown.
    Logic {
        logic_context: Context<PhantomData<Logic>>,
    },
    // Used for methods that can return 'Ok' or 'Err' at different times with the same
    // set of parameters.
    Runtime {
        runtime_context: Context<PhantomData<Runtime>>,
    },
}
pub struct InvalidArgument;
pub struct IndefiniteArgument;
pub struct Logic;
pub struct Runtime;
pub struct Context<T> {
    pub subject: T,
    pub error: Box<dyn StdError + Send + Sync + 'static>,
}
pub trait ResultConverter<T> {
    fn into_indefinite_argument(self, backtrace: Backtrace) -> Result<T, AggregateError>;
    fn into_logic(self, backtrace: Backtrace) -> Result<T, AggregateError>;
    fn into_runtime(self, backtrace: Backtrace) -> Result<T, AggregateError>;
}
impl<E, T> ResultConverter<T> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn into_indefinite_argument(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.map_err(
            move |error: _| -> _ {
                return AggregateError::new_indefinite_argument(
                    error.into(),
                    backtrace,
                );
            },
        );
    }
    fn into_logic(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.map_err(
            move |error: _| -> _ {
                return AggregateError::new_logic(
                    error.into(),
                    backtrace,
                );
            },
        );
    }
    fn into_runtime(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.map_err(
            move |error: _| -> _ {
                return AggregateError::new_runtime(
                    error.into(),
                    backtrace,
                );
            },
        );
    }
}
pub trait ResultConverter_<T> {
    fn into_indefinite_argument(self, backtrace: Backtrace) -> Result<T, AggregateError>;
    fn into_logic(self, backtrace: Backtrace) -> Result<T, AggregateError>;
    fn into_runtime(self, backtrace: Backtrace) -> Result<T, AggregateError>;
}
impl<T> ResultConverter_<T> for Result<T, Box<dyn StdError + Sync + Send + 'static>> {
    fn into_indefinite_argument(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.map_err(
            move |error: _| -> _ {
                return AggregateError::new_indefinite_argument(
                    error,
                    backtrace,
                );
            },
        );
    }
    fn into_logic(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.map_err(
            move |error: _| -> _ {
                return AggregateError::new_logic(
                    error,
                    backtrace,
                );
            },
        );
    }
    fn into_runtime(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.map_err(
            move |error: _| -> _ {
                return AggregateError::new_runtime(
                    error,
                    backtrace,
                );
            },
        );
    }
}
pub trait OptionConverter<T> {
    fn into_logic_unreachable_state(self, backtrace: Backtrace) -> Result<T, AggregateError>;
    fn into_logic_out_of_range(self, backtrace: Backtrace) -> Result<T, AggregateError>;
    fn into_logic_value_does_not_exist(self, backtrace: Backtrace) -> Result<T, AggregateError>;
}
impl<T> OptionConverter<T> for Option<T> {
    fn into_logic_unreachable_state(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.ok_or_else(
            move || -> _ {
                return AggregateError::new_logic(
                    Common::UnreachableState.into(),
                    backtrace,
                );
            },
        );
    }
    fn into_logic_out_of_range(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.ok_or_else(
            move || -> _ {
                return AggregateError::new_logic(
                    Common::OutOfRange.into(),
                    backtrace,
                );
            },
        );
    }
    fn into_logic_value_does_not_exist(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.ok_or_else(
            move || -> _ {
                return AggregateError::new_logic(
                    Common::ValueDoesNotExist.into(),
                    backtrace,
                );
            },
        );
    }
}
#[derive(Debug)]
pub enum Common {
    InvalidSocketAddress,
    OutOfRange,
    UnreachableState,
    ValueAlreadyExist,
    ValueDoesNotExist,
}
impl Display for Common {
    fn fmt<'a>(&'a self, formatter: &'a mut Formatter<'_>) -> Result<(), FmtError> {
        let message = match *self {
            Self::InvalidSocketAddress => "Invalid socket address.",
            Self::OutOfRange => "Out of range.",
            Self::UnreachableState => "Unreachable state.",
            Self::ValueAlreadyExist => "Value already exist.",
            Self::ValueDoesNotExist => "Value does not exist.",
        };
        return write!(formatter, "{}", message);
    }
}
impl StdError for Common {}
