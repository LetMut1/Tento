use std::{
    error::Error as StdError,
    fmt::{
        Debug,
        Display,
        Error as FmtError,
        Formatter,
    },
};
pub struct AggregateError(pub Auditor<AggregateError_>);
impl AggregateError {
    pub fn new_logic(message: &'static str, backtrace: Backtrace) -> Self {
        return Self(
            Auditor::<AggregateError_>::new(
                AggregateError_::Logic {
                    logic: Logic {
                        message,
                    },
                },
                backtrace,
            ),
        );
    }
    pub fn new_logic_value_does_not_exist(backtrace: Backtrace) -> Self {
        return Self(
            Auditor::<AggregateError_>::new(
                AggregateError_::Logic {
                    logic: Logic {
                        message: "Value does not exist.",
                    },
                },
                backtrace,
            ),
        );
    }
    pub fn new_logic_value_already_exist(backtrace: Backtrace) -> Self {
        return Self(
            Auditor::<AggregateError_>::new(
                AggregateError_::Logic {
                    logic: Logic {
                        message: "Value already exist.",
                    },
                },
                backtrace,
            ),
        );
    }
    pub fn new_logic_unreachable_state(backtrace: Backtrace) -> Self {
        return Self(
            Auditor::<AggregateError_>::new(
                AggregateError_::Logic {
                    logic: Logic {
                        message: "Unreachable state.",
                    },
                },
                backtrace,
            ),
        );
    }
    pub fn new_logic_out_of_range(backtrace: Backtrace) -> Self {
        return Self(
            Auditor::<AggregateError_>::new(
                AggregateError_::Logic {
                    logic: Logic {
                        message: "Out of range.",
                    },
                },
                backtrace,
            ),
        );
    }
    pub fn new_runtime<E>(error: E, backtrace: Backtrace) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        return Self(
            Auditor::<AggregateError_>::new(
                AggregateError_::Runtime {
                    runtime: Runtime {
                        context: Context(error.into()),
                    },
                },
                backtrace,
            ),
        );
    }
    pub fn new_runtime_(error: Box<dyn StdError + Send + Sync + 'static>, backtrace: Backtrace) -> Self {
        return Self(
            Auditor::<AggregateError_>::new(
                AggregateError_::Runtime {
                    runtime: Runtime {
                        context: Context(error),
                    },
                },
                backtrace,
            ),
        );
    }
    pub fn new_invalid_argument_from_outside(backtrace: Backtrace) -> Self {
        return Self(
            Auditor::<AggregateError_>::new(
                AggregateError_::InvalidArgument {
                    invalid_argument: InvalidArgument::FromOutside,
                },
                backtrace,
            ),
        );
    }
    pub fn new_invalid_argument_from_client_code<E>(error: E, backtrace: Backtrace) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        return Self(
            Auditor::<AggregateError_>::new(
                AggregateError_::InvalidArgument {
                    invalid_argument: InvalidArgument::FromClientCode {
                        from_client_code: FromClientCode {
                            context: Context(error.into()),
                        },
                    },
                },
                backtrace,
            ),
        );
    }
    pub fn new_invalid_argument_from_client_code_(error: Box<dyn StdError + Send + Sync + 'static>, backtrace: Backtrace) -> Self {
        return Self(
            Auditor::<AggregateError_>::new(
                AggregateError_::InvalidArgument {
                    invalid_argument: InvalidArgument::FromClientCode {
                        from_client_code: FromClientCode {
                            context: Context(error),
                        },
                    },
                },
                backtrace,
            ),
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
pub enum AggregateError_ {
    Logic {
        logic: Logic,
    },
    // Used for methods that can return Ok or Err at different times with the same set of parameters.
    Runtime {
        runtime: Runtime,
    },
    // Used for methods that should always return only one of Ok or Err at different times with the same set of parameters.
    InvalidArgument {
        invalid_argument: InvalidArgument,
    },
}
pub struct Logic {
    pub message: &'static str,
}
pub struct Runtime {
    pub context: Context,
}
pub struct Context(Box<dyn StdError + Send + Sync + 'static>);
impl Context {
    pub fn get<'a>(&'a self) -> &'a (dyn StdError + 'static) {
        return self.0.as_ref();
    }
}
pub enum InvalidArgument {
    FromOutside,
    FromClientCode {
        from_client_code: FromClientCode,
    },
}
pub struct FromClientCode {
    pub context: Context,
}
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
pub trait ResultConverter<T> {
    fn into_runtime(self, backtrace: Backtrace) -> Result<T, AggregateError>;
    fn into_invalid_argument_from_client_code(self, backtrace: Backtrace) -> Result<T, AggregateError>;
}
impl<E, T> ResultConverter<T> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
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
    fn into_invalid_argument_from_client_code(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.map_err(
            move |error: _| -> _ {
                return AggregateError::new_invalid_argument_from_client_code(
                    error,
                    backtrace,
                );
            },
        );
    }
}
pub trait ResultConverter_<T> {
    fn into_runtime(self, backtrace: Backtrace) -> Result<T, AggregateError>;
    fn into_invalid_argument_from_client_code(self, backtrace: Backtrace) -> Result<T, AggregateError>;
}
impl<T> ResultConverter_<T> for Result<T, Box<dyn StdError + Sync + Send + 'static>> {
    fn into_runtime(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.map_err(
            move |error: _| -> _ {
                return AggregateError::new_runtime_(
                    error,
                    backtrace,
                );
            },
        );
    }
    fn into_invalid_argument_from_client_code(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.map_err(
            move |error: _| -> _ {
                return AggregateError::new_invalid_argument_from_client_code_(
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
                return AggregateError::new_logic_unreachable_state(backtrace);
            },
        );
    }
    fn into_logic_out_of_range(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.ok_or_else(
            move || -> _ {
                return AggregateError::new_logic_out_of_range(backtrace);
            },
        );
    }
    fn into_logic_value_does_not_exist(self, backtrace: Backtrace) -> Result<T, AggregateError> {
        return self.ok_or_else(
            move || -> _ {
                return AggregateError::new_logic_value_does_not_exist(backtrace);
            },
        );
    }
}
