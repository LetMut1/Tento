use auditor::{
    Auditor,
    Backtrace,
};
use std::{
    error::Error as StdError,
    fmt::{
        Debug,
        Display,
        Error as FmtError,
        Formatter,
    },
};
pub enum AlternativeWorkflow {
    InternalError {
        internal_error_auditor: Auditor<InternalError>,
    },
    InvalidArgument {
        invalid_argument_auditor: Auditor<InvalidArgument>,
    },
}
impl AlternativeWorkflow {
    pub fn new_internal_error_logic(message: &'static str, backtrace: Backtrace) -> Self {
        return Self::InternalError {
            internal_error_auditor: Auditor::new(
                InternalError::Logic {
                    message,
                },
                backtrace,
            ),
        };
    }
    pub fn new_internal_error_logic_value_does_not_exist(backtrace: Backtrace) -> Self {
        return Self::InternalError {
            internal_error_auditor: Auditor::new(
                InternalError::Logic {
                    message: "Value does not exist.",
                },
                backtrace,
            ),
        };
    }
    pub fn new_internal_error_logic_value_already_exist(backtrace: Backtrace) -> Self {
        return Self::InternalError {
            internal_error_auditor: Auditor::new(
                InternalError::Logic {
                    message: "Value already exist.",
                },
                backtrace,
            ),
        };
    }
    pub fn new_internal_error_logic_unreachable_state(backtrace: Backtrace) -> Self {
        return Self::InternalError {
            internal_error_auditor: Auditor::new(
                InternalError::Logic {
                    message: "Unreachable state.",
                },
                backtrace,
            ),
        };
    }
    pub fn new_internal_error_logic_out_of_range(backtrace: Backtrace) -> Self {
        return Self::InternalError {
            internal_error_auditor: Auditor::new(
                InternalError::Logic {
                    message: "Out of range.",
                },
                backtrace,
            ),
        };
    }
    pub fn new_internal_error_runtime<E>(error: E, backtrace: Backtrace) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        return Self::InternalError {
            internal_error_auditor: Auditor::new(
                InternalError::Runtime {
                    runtime: Runtime {
                        error: error.into(),
                    },
                },
                backtrace,
            ),
        };
    }
    pub fn new_internal_error_runtime_(error: Box<dyn StdError + Send + Sync + 'static>, backtrace: Backtrace) -> Self {
        return Self::InternalError {
            internal_error_auditor: Auditor::new(
                InternalError::Runtime {
                    runtime: Runtime {
                        error,
                    },
                },
                backtrace,
            ),
        };
    }
    pub fn new_invalid_argument_from_outside(backtrace: Backtrace) -> Self {
        return Self::InvalidArgument {
            invalid_argument_auditor: Auditor::new(
                InvalidArgument::FromOutside,
                backtrace,
            ),
        };
    }
    pub fn new_invalid_argument_from_client_code(backtrace: Backtrace) -> Self {
        return Self::InvalidArgument {
            invalid_argument_auditor: Auditor::new(
                InvalidArgument::FromClientCode,
                backtrace,
            ),
        };
    }
}
impl Debug for AlternativeWorkflow {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FmtError> {
        return Ok(());
    }
}
impl Display for AlternativeWorkflow {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FmtError> {
        return Ok(());
    }
}
impl StdError for AlternativeWorkflow {}
pub enum InternalError {
    Logic {
        message: &'static str,
    },
    Runtime {
        runtime: Runtime,
    },
}
pub struct Runtime {
    error: Box<dyn StdError + Send + Sync + 'static>,
}
impl Runtime {
    pub fn get<'a>(&'a self) -> &'a (dyn StdError + 'static) {
        return self.error.as_ref();
    }
}
pub enum InvalidArgument {
    FromOutside,
    FromClientCode,
}
pub trait ResultConverter<T> {
    fn into_internal_runtime(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow>;
}
impl<E, T> ResultConverter<T> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn into_internal_runtime(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow> {
        return self.map_err(
            move |error: _| -> _ {
                return AlternativeWorkflow::new_internal_error_runtime(
                    error,
                    backtrace,
                );
            },
        );
    }
}
pub trait ResultConverter_<T> {
    fn into_internal_runtime(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow>;
}
impl<T> ResultConverter_<T> for Result<T, Box<dyn StdError + Sync + Send + 'static>> {
    fn into_internal_runtime(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow> {
        return self.map_err(
            move |error: _| -> _ {
                return AlternativeWorkflow::new_internal_error_runtime_(
                    error,
                    backtrace,
                );
            },
        );
    }
}
pub trait OptionConverter<T> {
    fn into_internal_logic_unreachable_state(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow>;
    fn into_internal_logic_out_of_range(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow>;
    fn into_internal_logic_value_does_not_exist(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow>;
}
impl<T> OptionConverter<T> for Option<T> {
    fn into_internal_logic_unreachable_state(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow> {
        return self.ok_or_else(
            move || -> _ {
                return AlternativeWorkflow::new_internal_error_logic_unreachable_state(backtrace);
            },
        );
    }
    fn into_internal_logic_out_of_range(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow> {
        return self.ok_or_else(
            move || -> _ {
                return AlternativeWorkflow::new_internal_error_logic_out_of_range(backtrace);
            },
        );
    }
    fn into_internal_logic_value_does_not_exist(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow> {
        return self.ok_or_else(
            move || -> _ {
                return AlternativeWorkflow::new_internal_error_logic_value_does_not_exist(backtrace);
            },
        );
    }
}