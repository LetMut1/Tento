use std::error::Error as StdError;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Error as FmtError;
use std::fmt::Formatter;
pub enum Error {
    Logic {
        message: &'static str,
    },
    Runtime {
        runtime: Runtime,
    },
}
impl Error {
    pub fn new_logic(message: &'static str) -> Self {
        return Self::Logic {
            message,
        };
    }

    pub fn new_logic_value_does_not_exist() -> Self {
        return Self::Logic {
            message: "Value does not exist.",
        };
    }

    pub fn new_logic_value_already_exist() -> Self {
        return Self::Logic {
            message: "Value already exist.",
        };
    }

    pub fn new_logic_unreachable_state() -> Self {
        return Self::Logic {
            message: "Unreachable state.",
        };
    }

    pub fn new_logic_out_of_range() -> Self {
        return Self::Logic {
            message: "Out of range.",
        };
    }

    pub fn new_runtime<E>(error: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        return Self::Runtime {
            runtime: Runtime {
                error: error.into(),
            },
        };
    }

    pub fn new_runtime_(error: Box<dyn StdError + Send + Sync + 'static>) -> Self {
        return Self::Runtime {
            runtime: Runtime {
                error,
            },
        };
    }
}
impl Debug for Error {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FmtError> {
        return Ok(());
    }
}
impl Display for Error {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FmtError> {
        return Ok(());
    }
}
impl StdError for Error {}
pub struct Runtime {
    error: Box<dyn StdError + Send + Sync + 'static>,
}
impl Runtime {
    pub fn get<'a>(&'a self) -> &'a (dyn StdError + 'static) {
        return self.error.as_ref();
    }
}
