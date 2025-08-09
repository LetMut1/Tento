use {
    super::aggregate_error::{
        AggregateError,
        AggregateError_,
        Auditor,
        Context,
        IndefiniteArgument,
        InvalidArgument,
        Logic,
        Runtime,
    },
    std::{
        error::Error as StdError,
        fmt::{
            Debug,
            Display,
            Error as FmtError,
            Formatter,
        },
        marker::PhantomData,
    },
};
pub enum ServerWorkflowError {
    Responsive {
        responsive_auditor: Auditor<Responsive>,
    },
    Unresponsive {
        unresponsive_auditor: Auditor<Unresponsive>,
    },
}
impl ServerWorkflowError {
    pub fn new(aggregate_error: AggregateError) -> Self {
        return match aggregate_error.0.subject {
            AggregateError_::IndefiniteArgument {
                indefinite_argument_context,
            } =>
                Self::Responsive {
                    responsive_auditor: Auditor {
                        subject: Responsive::IndefiniteArgument {
                            indefinite_argument_context,
                        },
                        backtrace: aggregate_error.0.backtrace,
                    },
                },
            AggregateError_::InvalidArgument {
                invalid_argument,
            } =>
                Self::Responsive {
                    responsive_auditor: Auditor {
                        subject: Responsive::InvalidArgument {
                            invalid_argument,
                        },
                        backtrace: aggregate_error.0.backtrace,
                    },
                },
            AggregateError_::Logic {
                logic_context,
            } =>
                Self::Unresponsive {
                    unresponsive_auditor: Auditor {
                        subject: Unresponsive::Logic {
                            logic_context,
                        },
                        backtrace: aggregate_error.0.backtrace,
                    },
                },
            AggregateError_::Runtime {
                runtime_context,
            } =>
                Self::Unresponsive {
                    unresponsive_auditor: Auditor {
                        subject: Unresponsive::Runtime {
                            runtime_context,
                        },
                        backtrace: aggregate_error.0.backtrace,
                    },
                },
        };
    }
}
impl Debug for ServerWorkflowError {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FmtError> {
        return Result::Ok(());
    }
}
impl Display for ServerWorkflowError {
    fn fmt<'a, 'b>(&'a self, _: &'b mut Formatter<'_>) -> Result<(), FmtError> {
        return Result::Ok(());
    }
}
impl StdError for ServerWorkflowError {}
pub enum Responsive {
    IndefiniteArgument {
        indefinite_argument_context: Context<PhantomData<IndefiniteArgument>>,
    },
    InvalidArgument {
        invalid_argument: PhantomData<InvalidArgument>,
    },
}
pub enum Unresponsive {
    Logic {
        logic_context: Context<PhantomData<Logic>>,
    },
    Runtime {
        runtime_context: Context<PhantomData<Runtime>>,
    },
}
