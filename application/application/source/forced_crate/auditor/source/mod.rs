use error::Error;
use invalid_argument::InvalidArgument;
use std::{
    boxed::Box,
    error::Error as StdError,
};
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
    fn convert_into_error(self, backtrace_part: Backtrace) -> Result<T, Auditor<Error>>;
}
impl<E, T> ResultConverter<T> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn convert_into_error(self, backtrace_part: Backtrace) -> Result<T, Auditor<Error>> {
        return match self {
            Ok(value) => Ok(value),
            Err(error) => {
                Err(
                    Auditor::<Error>::new(
                        Error::new_internal_runtime(error),
                        backtrace_part,
                    ),
                )
            }
        };
    }
}
pub trait ResultConverter_<T> {
    fn convert_into_error(self, backtrace_part: Backtrace) -> Result<T, Auditor<Error>>;
}
impl<T> ResultConverter_<T> for Result<T, Box<dyn StdError + Sync + Send + 'static>> {
    fn convert_into_error(self, backtrace_part: Backtrace) -> Result<T, Auditor<Error>> {
        return match self {
            Ok(value) => Ok(value),
            Err(error) => {
                Err(
                    Auditor::<Error>::new(
                        Error::new_internal_runtime_(error),
                        backtrace_part,
                    ),
                )
            }
        };
    }
}
pub trait OptionConverter<T> {
    fn convert_unreachable_state(self, backtrace_part: Backtrace) -> Result<T, Auditor<Error>>;
    fn convert_out_of_range(self, backtrace_part: Backtrace) -> Result<T, Auditor<Error>>;
    fn convert_value_does_not_exist(self, backtrace_part: Backtrace) -> Result<T, Auditor<Error>>;
}
impl<T> OptionConverter<T> for Option<T> {
    fn convert_unreachable_state(self, backtrace_part: Backtrace) -> Result<T, Auditor<Error>> {
        return match self {
            Some(value) => Ok(value),
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::new_internal_logic_unreachable_state(),
                        backtrace_part,
                    ),
                );
            }
        };
    }
    fn convert_out_of_range(self, backtrace_part: Backtrace) -> Result<T, Auditor<Error>> {
        return match self {
            Some(value) => Ok(value),
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::new_internal_logic_out_of_range(),
                        backtrace_part,
                    ),
                );
            }
        };
    }
    fn convert_value_does_not_exist(self, backtrace_part: Backtrace) -> Result<T, Auditor<Error>> {
        return match self {
            Some(value) => Ok(value),
            None => {
                return Err(
                    Auditor::<Error>::new(
                        Error::new_internal_logic_value_does_not_exist(),
                        backtrace_part,
                    ),
                );
            }
        };
    }
}
