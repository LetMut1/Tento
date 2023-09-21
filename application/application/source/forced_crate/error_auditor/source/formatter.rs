use super::error_auditor::Error;
use super::error_auditor::EmailServer;
use super::error_auditor::ErrorAuditor;
use super::error_auditor::Resource;
use super::error_auditor::Runtime;

pub struct Formatter;

impl Formatter {
    pub fn prepare<'a>(error_auditor: &'a ErrorAuditor) -> String {
        let mut backtrace_message = String::new();

        '_a: for (index, backtrace_part) in error_auditor.get_backtrace().get_backtrace_part_registry().iter().enumerate() {
            if index == 0 {
                backtrace_message = match backtrace_part.get_context() {
                    Some(context) => {
                        format!(
                            "({}) {}:{} ({}).\n",
                            index,
                            backtrace_part.get_file_path(),
                            backtrace_part.get_line_number(),
                            context
                        )
                    }
                    None => {
                        format!(
                            "({}) {}:{}.\n",
                            index,
                            backtrace_part.get_file_path(),
                            backtrace_part.get_line_number()
                        )
                    }
                };
            } else {
                backtrace_message = match backtrace_part.get_context() {
                    Some(context) => {
                        format!(
                            "{}({}) {}:{} ({})\n.",
                            backtrace_message.as_str(),
                            index,
                            backtrace_part.get_file_path(),
                            backtrace_part.get_line_number(),
                            context
                        )
                    }
                    None => {
                        format!(
                            "{}({}) {}:{}.\n",
                            backtrace_message.as_str(),
                            index,
                            backtrace_part.get_file_path(),
                            backtrace_part.get_line_number()
                        )
                    }
                }
            };
        }

        let error_message = match *error_auditor.get_error() {
            Error::Logic {
                message,
            } => {
                format!(
                    "Error, logic: {}.",
                    message
                )
            }
            Error::Runtime {
                runtime: ref run_time_error,
            } => match *run_time_error {
                Runtime::Other {
                    other: ref other_error,
                } => {
                    format!(
                        "Error, runtime, other: {}.",
                        other_error.get_message()
                    )
                }
                Runtime::Resource {
                    resource: ref resource_error,
                } => match *resource_error {
                    Resource::ConnectionPoolRedis {
                        ref bb8_redis_error,
                    } => {
                        format!(
                            "Error, runtime, resource, Redis connection pool : {}.",
                            bb8_redis_error
                        )
                    }
                    Resource::ConnectionPoolPostgresql {
                        ref bb8_postgresql_error,
                    } => {
                        format!(
                            "Error, runtime, resource, Postgresql connection pool : {}.",
                            bb8_postgresql_error
                        )
                    }
                    Resource::EmailServer {
                        email_server: ref email_server_error,
                    } => match *email_server_error {
                        EmailServer::Email {
                            ref email_error,
                        } => {
                            format!(
                                "Error, runtime, resource, email : {}.",
                                email_error
                            )
                        }
                        EmailServer::Smtp {
                            ref smtp_error,
                        } => {
                            format!(
                                "Error, runtime, resource, email : {}.",
                                smtp_error
                            )
                        }
                    },
                    Resource::Postgresql {
                        ref postgresql_error,
                    } => {
                        format!(
                            "Error, runtime, resource, Postgresql : {}.",
                            postgresql_error
                        )
                    }
                    Resource::Redis {
                        ref redis_error,
                    } => {
                        format!(
                            "Error, runtime, resource, Redis : {}.",
                            redis_error
                        )
                    }
                },
            },
        };

        return format!(
            "{} > {}",
            backtrace_message.as_str(),
            error_message.as_str()
        );
    }
}