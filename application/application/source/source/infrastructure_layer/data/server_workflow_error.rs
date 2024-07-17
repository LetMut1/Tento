use std::{
    error::Error as StdError,
    fmt::{
        Debug,
        Display,
        Error as FmtError,
        Formatter,
    },
};
use super::aggregate_error::{
    AggregateError,
    AggregateError_,
    InvalidArgument,
    Runtime,
    Logic,
    Auditor,
};
pub enum ServerWorkflowError {
    Unexpected {
        unexpected_auditor: Auditor<Unexpected>,
    },
    Expected {
        expected_auditor: Auditor<Expected>,
    },
}
impl ServerWorkflowError {
    pub fn new(aggregate_error: AggregateError) -> Self {
        return match aggregate_error.0.subject {
            AggregateError_::Logic {
                logic
            } => Self::Unexpected {
                unexpected_auditor: Auditor::<Unexpected>::new(
                    Unexpected::Logic {
                        logic
                    },
                    aggregate_error.0.backtrace,
                ),
            },
            AggregateError_::Runtime {
                runtime
            } => Self::Unexpected {
                unexpected_auditor: Auditor::<Unexpected>::new(
                    Unexpected::Runtime {
                        runtime
                    },
                    aggregate_error.0.backtrace,
                ),
            },
            AggregateError_::InvalidArgument {
                invalid_argument
            } => Self::Expected {
                expected_auditor: Auditor::<Expected>::new(
                    Expected {
                        invalid_argument,
                    },
                    aggregate_error.0.backtrace,
                ),
            },
        };
    }
}
impl Debug for ServerWorkflowError {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FmtError> {
        return Ok(());
    }
}
impl Display for ServerWorkflowError {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FmtError> {
        return Ok(());
    }
}
impl StdError for ServerWorkflowError {}
pub enum Unexpected {
    Logic {
        logic: Logic,
    },
    Runtime {
        runtime: Runtime,
    },
}
pub struct Expected {
    pub invalid_argument: InvalidArgument,
}