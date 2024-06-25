use std::{
    error::Error as StdError,
    fmt::{
        Debug,
        Display,
        Error as FmtError,
        Formatter,
    },
};
use auditor::{
    Auditor,
    Backtrace,
};
pub enum AlternativeWorkflow {
    Internal {
        internal_auditor: Auditor<Internal>,
    },
    External {
        external_auditor: Auditor<External>,
    }
}
impl AlternativeWorkflow {
    pub fn new_internal_logic(message: &'static str, backtrace: Backtrace) -> Self {
        return Self::Internal { internal_auditor: Auditor::new(Internal::Logic { message }, backtrace) };
    }
    pub fn new_internal_logic_value_does_not_exist(backtrace: Backtrace) -> Self {
        return Self::Internal { internal_auditor: Auditor::new(Internal::Logic { message: "Value does not exist." }, backtrace) };
    }
    pub fn new_internal_logic_value_already_exist(backtrace: Backtrace) -> Self {
        return Self::Internal { internal_auditor: Auditor::new(Internal::Logic { message: "Value already exist."}, backtrace) };
    }
    pub fn new_internal_logic_unreachable_state(backtrace: Backtrace) -> Self {
        return Self::Internal { internal_auditor: Auditor::new(Internal::Logic { message: "Unreachable state." }, backtrace) };
    }
    pub fn new_internal_logic_out_of_range(backtrace: Backtrace) -> Self {
        return Self::Internal { internal_auditor: Auditor::new(Internal::Logic { message: "Out of range." }, backtrace) };
    }
    pub fn new_internal_runtime<E>(error: E, backtrace: Backtrace) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        return Self::Internal { internal_auditor: Auditor::new(Internal::Runtime { runtime: Runtime { inner: error.into() } }, backtrace) };
    }
    pub fn new_internal_runtime_(error: Box<dyn StdError + Send + Sync + 'static>, backtrace: Backtrace) -> Self {
        return Self::Internal { internal_auditor: Auditor::new(Internal::Runtime { runtime: Runtime { inner: error } }, backtrace) };
    }
    pub fn new_external_invalid_argument(backtrace: Backtrace) -> Self {
        return Self::External { external_auditor: Auditor::new(External::InvalidArgument, backtrace) };
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
pub trait ResultConverter<T> {
    fn convert_into_error(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow>;
}
impl<E, T> ResultConverter<T> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn convert_into_error(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow> {
        return self.map_err(
            move |error: _| -> _ {
                return AlternativeWorkflow::new_internal_runtime(error, backtrace);
            }
        );
    }
}
pub trait ResultConverter_<T> {
    fn convert_into_error(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow>;
}
impl<T> ResultConverter_<T> for Result<T, Box<dyn StdError + Sync + Send + 'static>> {
    fn convert_into_error(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow> {
        return self.map_err(
            move |error: _| -> _ {
                return AlternativeWorkflow::new_internal_runtime_(error, backtrace);
            }
        );
    }
}
pub trait OptionConverter<T> {
    fn convert_unreachable_state(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow>;
    fn convert_out_of_range(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow>;
    fn convert_value_does_not_exist(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow>;
}
impl<T> OptionConverter<T> for Option<T> {
    fn convert_unreachable_state(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow> {
        return self.ok_or_else(
            move || -> _ {
                return AlternativeWorkflow::new_internal_logic_unreachable_state(backtrace);
            }
        );
    }
    fn convert_out_of_range(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow> {
        return self.ok_or_else(
            move || -> _ {
                return AlternativeWorkflow::new_internal_logic_out_of_range(backtrace);
            }
        );
    }
    fn convert_value_does_not_exist(self, backtrace: Backtrace) -> Result<T, AlternativeWorkflow> {
        return self.ok_or_else(
            move || -> _ {
                return AlternativeWorkflow::new_internal_logic_value_does_not_exist(backtrace);
            }
        );
    }
}
