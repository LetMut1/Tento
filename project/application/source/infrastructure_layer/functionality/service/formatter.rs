use crate::infrastructure_layer::data::error_auditor::BaseError;
use crate::infrastructure_layer::data::error_auditor::EmailServerError;
use crate::infrastructure_layer::data::error_auditor::ErrorAuditor;
use crate::infrastructure_layer::data::error_auditor::ResourceError;
use crate::infrastructure_layer::data::error_auditor::RuntimeError;
use crate::infrastructure_layer::data::invalid_argument_result::InvalidArgument;

pub struct Formatter;

pub trait Format<T> {
    fn prepare<'a>(subject: &'a T) -> String;
}

impl Format<ErrorAuditor> for Formatter {
    fn prepare<'a>(subject: &'a ErrorAuditor) -> String {
        let mut backtrace_message = String::new();

        '_a: for (index, backtrace_part) in subject
            .get_backtrace()
            .get_backtrace_part_registry()
            .iter()
            .enumerate()
        {
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

        let error_message = match *subject.get_base_error() {
            BaseError::LogicError {
                message,
            } => {
                format!(
                    "Error, logic: {}.",
                    message
                )
            }
            BaseError::RuntimeError {
                runtime_error: ref run_time_error,
            } => match *run_time_error {
                RuntimeError::OtherError {
                    ref other_error,
                } => {
                    format!(
                        "Error, runtime, other: {}.",
                        other_error.get_message()
                    )
                }
                RuntimeError::ResourceError {
                    ref resource_error,
                } => match *resource_error {
                    ResourceError::ConnectionPoolRedisError {
                        ref bb8_redis_error,
                    } => {
                        format!(
                            "Error, runtime, resource, Redis connection pool : {}.",
                            bb8_redis_error
                        )
                    }
                    ResourceError::ConnectionPoolPostgresqlError {
                        ref bb8_postgresql_error,
                    } => {
                        format!(
                            "Error, runtime, resource, Postgresql connection pool : {}.",
                            bb8_postgresql_error
                        )
                    }
                    ResourceError::EmailServerError {
                        ref email_server_error,
                    } => match *email_server_error {
                        EmailServerError::EmailError {
                            ref email_error,
                        } => {
                            format!(
                                "Error, runtime, resource, email : {}.",
                                email_error
                            )
                        }
                        EmailServerError::SmtpError {
                            ref smtp_error,
                        } => {
                            format!(
                                "Error, runtime, resource, email : {}.",
                                smtp_error
                            )
                        }
                    },
                    ResourceError::PostgresqlError {
                        ref postgresql_error,
                    } => {
                        format!(
                            "Error, runtime, resource, Postgresql : {}.",
                            postgresql_error
                        )
                    }
                    ResourceError::RedisError {
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

impl Format<InvalidArgument> for Formatter {
    fn prepare<'a>(subject: &'a InvalidArgument) -> String {
        let message_part = match *subject {
            InvalidArgument::ApplicationUser_AccessModifier => "AccessModifier",
            InvalidArgument::ApplicationUser_Email => "ApplicationUser_Email",
            InvalidArgument::ApplicationUser_Id => "ApplicationUser_Id",
            InvalidArgument::ApplicationUser_Nickname => "ApplicationUser_Nickname",
            InvalidArgument::ApplicationUser_Password => "ApplicationUser_Password",
            InvalidArgument::ApplicationUser_VisabilityModifier => "VisabilityModifier",
            InvalidArgument::ApplicationUserAccessRefreshToken_DeserializedForm => "ApplicationUserAccessRefreshToken_DeserializedForm",
            InvalidArgument::ApplicationUserAccessTokenEncrypted => "ApplicationUserAccessTokenEncrypted",
            InvalidArgument::ApplicationUserAuthorizationToken_Value => "ApplicationUserAuthorizationToken_Value",
            InvalidArgument::ApplicationUserDevice_Id => "ApplicationUserDevice_Id",
            InvalidArgument::ApplicationUserRegistrationToken_Value => "ApplicationUserRegistrationToken_Value",
            InvalidArgument::ApplicationUserResetPasswordToken_Value => "ApplicationUserResetPasswordToken_Value",
            InvalidArgument::Channel_Id => "Channel_Id",
            InvalidArgument::Channel_Name => "Channel_Name",
            InvalidArgument::HttpHeaders => "HttpHeader",
            InvalidArgument::HttpRoute => "HttpRoute",
            InvalidArgument::Limit => "Limit",
            InvalidArgument::SearchParameter => "SearchParameter",
            InvalidArgument::SortOrderRepresentation => "SortOrderRepresentation",
            InvalidArgument::Timestamp => "Timestamp",
        };

        return format!(
            "Invalid argument: {}.",
            message_part
        );
    }
}
