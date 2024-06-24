use std::{
    error::Error as StdError,
    fmt::{
        Debug,
        Display,
        Error as FmtError,
        Formatter,
    },
};
pub enum Error {
    Internal {
        internal: Internal,
    },
    External {
        external: External,
    }
}
impl Error {
    pub fn new_internal_logic(message: &'static str) -> Self {
        return Self::Internal { internal: Internal::Logic { message } };
    }
    pub fn new_internal_logic_value_does_not_exist() -> Self {
        return Self::Internal { internal: Internal::Logic { message: "Value does not exist." } };
    }
    pub fn new_internal_logic_value_already_exist() -> Self {
        return Self::Internal { internal: Internal::Logic { message: "Value already exist."} };
    }
    pub fn new_internal_logic_unreachable_state() -> Self {
        return Self::Internal { internal: Internal::Logic { message: "Unreachable state." } };
    }
    pub fn new_internal_logic_out_of_range() -> Self {
        return Self::Internal { internal: Internal::Logic { message: "Out of range." } };
    }
    pub fn new_internal_runtime<E>(error: E) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        return Self::Internal { internal: Internal::Runtime { runtime: Runtime { inner: error.into() } } };
    }
    pub fn new_internal_runtime_(error: Box<dyn StdError + Send + Sync + 'static>) -> Self {
        return Self::Internal { internal: Internal::Runtime { runtime: Runtime { inner: error } } };
    }
    pub fn new_external_invalid_argument() -> Self {
        return Self::External { external: External::InvalidArgument };
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
pub enum Internal {
    Logic {
        message: &'static str,
    },
    Runtime {
        runtime: Runtime,
    },
}
pub struct Runtime {
    inner: Box<dyn StdError + Send + Sync + 'static>,
}
impl Runtime {
    pub fn get<'a>(&'a self) -> &'a (dyn StdError + 'static) {
        return self.inner.as_ref();
    }
}
pub enum External {
    InvalidArgument,
}
