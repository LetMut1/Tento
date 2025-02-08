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
        return Result::Ok(());
    }
}
impl Display for AggregateError {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FmtError> {
        return Result::Ok(());
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
macro_rules! new_indefinite_argument {
    ($error:expr) => {
        crate::infrastructure_layer::data::aggregate_error::AggregateError::new_indefinite_argument(
            $error.into(),
            crate::infrastructure_layer::data::aggregate_error::Backtrace::new(
                std::line!(),
                std::file!(),
            ),
        )
    };
}
macro_rules! new_invalid_argument {
    () => {
        crate::infrastructure_layer::data::aggregate_error::AggregateError::new_invalid_argument(
            crate::infrastructure_layer::data::aggregate_error::Backtrace::new(
                std::line!(),
                std::file!(),
            ),
        )
    };
}
macro_rules! new_logic {
    ($error:expr) => {
        crate::infrastructure_layer::data::aggregate_error::AggregateError::new_logic(
            $error.into(),
            crate::infrastructure_layer::data::aggregate_error::Backtrace::new(
                std::line!(),
                std::file!(),
            ),
        )
    };
}
macro_rules! new_logic_invalid_socket_address {
    () => {
        crate::infrastructure_layer::data::aggregate_error::new_logic!(
            crate::infrastructure_layer::data::aggregate_error::Common::InvalidSocketAddress
        )
    };
}
macro_rules! new_logic_out_of_range {
    () => {
        crate::infrastructure_layer::data::aggregate_error::new_logic!(
            crate::infrastructure_layer::data::aggregate_error::Common::OutOfRange
        )
    };
}
macro_rules! new_logic_unreachable_state {
    () => {
        crate::infrastructure_layer::data::aggregate_error::new_logic!(
            crate::infrastructure_layer::data::aggregate_error::Common::UnreachableState
        )
    };
}
macro_rules! new_logic_value_already_exist {
    () => {
        crate::infrastructure_layer::data::aggregate_error::new_logic!(
            crate::infrastructure_layer::data::aggregate_error::Common::ValueAlreadyExist
        )
    };
}
macro_rules! new_logic_value_does_not_exist {
    () => {
        crate::infrastructure_layer::data::aggregate_error::new_logic!(
            crate::infrastructure_layer::data::aggregate_error::Common::ValueDoesNotExist
        )
    };
}
macro_rules! new_runtime {
    ($error:expr) => {
        crate::infrastructure_layer::data::aggregate_error::AggregateError::new_runtime(
            $error.into(),
            crate::infrastructure_layer::data::aggregate_error::Backtrace::new(
                std::line!(),
                std::file!(),
            ),
        )
    };
}
macro_rules! result_return_indefinite_argument {
    ($std_result:expr) => {
        match $std_result {
            std::result::Result::Ok(value) => value,
            std::result::Result::Err(error) => {
                return std::result::Result::Err(
                    crate::infrastructure_layer::data::aggregate_error::new_indefinite_argument!(error),
                );
            }
        }
    };
}
macro_rules! result_into_indefinite_argument {
    ($std_result:expr) => {
        $std_result.map_err(
            move |error: _| -> _ {
                return crate::infrastructure_layer::data::aggregate_error::new_indefinite_argument!(error);
            },
        )
    };
}
macro_rules! result_return_logic {
    ($std_result:expr) => {
        match $std_result {
            std::result::Result::Ok(value) => value,
            std::result::Result::Err(error) => {
                return std::result::Result::Err(
                    crate::infrastructure_layer::data::aggregate_error::new_logic!(error),
                );
            }
        }
    };
}
macro_rules! result_into_logic {
    ($std_result:expr) => {
        $std_result.map_err(
            move |error: _| -> _ {
                return crate::infrastructure_layer::data::aggregate_error::new_logic!(error);
            },
        )
    };
}
macro_rules! result_return_runtime {
    ($std_result:expr) => {
        match $std_result {
            std::result::Result::Ok(value) => value,
            std::result::Result::Err(error) => {
                return std::result::Result::Err(
                    crate::infrastructure_layer::data::aggregate_error::new_runtime!(error),
                );
            }
        }
    };
}
macro_rules! result_into_runtime {
    ($std_result:expr) => {
        $std_result.map_err(
            move |error: _| -> _ {
                return crate::infrastructure_layer::data::aggregate_error::new_runtime!(error);
            },
        )
    };
}
macro_rules! option_return_logic_unreachable_state {
    ($std_option:expr) => {
        match $std_option {
            std::option::Option::Some(value) => value,
            std::option::Option::None => {
                return std::result::Result::Err(
                    crate::infrastructure_layer::data::aggregate_error::new_logic_unreachable_state!(),
                );
            }
        }
    };
}
macro_rules! option_return_logic_out_of_range {
    ($std_option:expr) => {
        match $std_option {
            std::option::Option::Some(value) => value,
            std::option::Option::None => {
                return std::result::Result::Err(
                    crate::infrastructure_layer::data::aggregate_error::new_logic_out_of_range!(),
                );
            }
        }
    };
}
macro_rules! option_into_logic_out_of_range {
    ($std_option:expr) => {
        $std_option.ok_or(
            crate::infrastructure_layer::data::aggregate_error::new_logic_out_of_range!(),
        )
    };
}
macro_rules! option_return_logic_value_does_not_exist {
    ($std_option:expr) => {
        match $std_option {
            std::option::Option::Some(value) => value,
            std::option::Option::None => {
                return std::result::Result::Err(
                    crate::infrastructure_layer::data::aggregate_error::new_logic_value_does_not_exist!(),
                );
            }
        }
    };
}
macro_rules! option_into_logic_value_does_not_exist {
    ($std_option:expr) => {
        $std_option.ok_or(
            crate::infrastructure_layer::data::aggregate_error::new_logic_value_does_not_exist!(),
        )
    };
}
macro_rules! option_return_logic_invalid_socket_address {
    ($std_option:expr) => {
        match $std_option {
            std::option::Option::Some(value) => value,
            std::option::Option::None => {
                return std::result::Result::Err(
                    crate::infrastructure_layer::data::aggregate_error::new_logic_invalid_socket_address!(),
                );
            }
        }
    };
}
pub(crate) use {
    new_indefinite_argument,
    new_invalid_argument,
    new_logic,
    new_logic_invalid_socket_address,
    new_logic_out_of_range,
    new_logic_unreachable_state,
    new_logic_value_already_exist,
    new_logic_value_does_not_exist,
    new_runtime,
    result_return_indefinite_argument,
    result_into_indefinite_argument,
    result_return_logic,
    result_into_logic,
    result_return_runtime,
    result_into_runtime,
    option_return_logic_unreachable_state,
    option_return_logic_out_of_range,
    option_into_logic_out_of_range,
    option_return_logic_value_does_not_exist,
    option_into_logic_value_does_not_exist,
    option_return_logic_invalid_socket_address,
};