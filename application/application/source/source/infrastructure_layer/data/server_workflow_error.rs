use super::aggregate_error::{
    AggregateError,
    AggregateError_,
    Auditor,
    InvalidArgument,
    FromClientCode,
    FromOutsideAndClientCode,
    Logic,
    Runtime,
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
                logic,
            } => Self::Unexpected {
                unexpected_auditor: Auditor::<Unexpected>::new(
                    Unexpected::Logic {
                        logic,
                    },
                    aggregate_error.0.backtrace,
                ),
            },
            AggregateError_::Runtime {
                runtime,
            } => Self::Unexpected {
                unexpected_auditor: Auditor::<Unexpected>::new(
                    Unexpected::Runtime {
                        runtime,
                    },
                    aggregate_error.0.backtrace,
                ),
            },
            AggregateError_::InvalidArgument {
                invalid_argument,
            } => {
                let server_workflow_error = match invalid_argument {
                    InvalidArgument::FromOutside => Self::Expected {
                        expected_auditor: Auditor::<Expected>::new(
                            Expected {
                                expected_invalid_argument: ExpectedInvalidArgument::FromOutside,
                            },
                            aggregate_error.0.backtrace,
                        ),
                    },
                    InvalidArgument::FromClientCode {
                        from_client_code
                    } => Self::Unexpected {
                        unexpected_auditor: Auditor::<Unexpected>::new(
                            Unexpected::InvalidArgument {
                                unexpected_invalid_argument: UnexpectedInvalidArgument {
                                    from_client_code,
                                }
                            },
                            aggregate_error.0.backtrace,
                        ),
                    },
                    InvalidArgument::FromOutsideAndClientCode {
                        from_outside_and_client_code
                    } => Self::Expected {
                        expected_auditor: Auditor::<Expected>::new(
                            Expected {
                                expected_invalid_argument: ExpectedInvalidArgument::FromOutsideAndClientCode {
                                    from_outside_and_client_code,
                                },
                            },
                            aggregate_error.0.backtrace,
                        ),
                    },
                };
                server_workflow_error
            }
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
    InvalidArgument {
        unexpected_invalid_argument: UnexpectedInvalidArgument,
    }
}
pub struct UnexpectedInvalidArgument {
    pub from_client_code: FromClientCode,
}
pub struct Expected {
    pub expected_invalid_argument: ExpectedInvalidArgument,
}
pub enum ExpectedInvalidArgument {
    FromOutside,
    FromOutsideAndClientCode {
        from_outside_and_client_code: FromOutsideAndClientCode,
    },
}