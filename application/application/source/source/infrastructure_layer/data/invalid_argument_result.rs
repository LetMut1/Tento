use crate::infrastructure_layer::functionality::service::formatter::Format;
use crate::infrastructure_layer::functionality::service::formatter::Formatter_;

pub enum InvalidArgumentResult<T> {
    Ok {
        subject: T,
    },
    InvalidArgument {
        invalid_argument: InvalidArgument,
    },
}

pub enum InvalidArgument {
    ApplicationUser_AccessModifier,
    ApplicationUser_Email,
    ApplicationUser_Id,
    ApplicationUser_Nickname,
    ApplicationUser_Password,
    ApplicationUser_VisabilityModifier,
    ApplicationUserAccessRefreshTokenEncrypted,
    ApplicationUserAccessTokenEncrypted,
    ApplicationUserAuthorizationToken_Value,
    ApplicationUserDevice_Id,
    ApplicationUserRegistrationToken_Value,
    ApplicationUserResetPasswordToken_Value,
    Channel_Id,
    Channel_Name,
    HttpHeader,
    HttpRoute,
    Limit,
    SearchParameter,
    SortOrderRepresentation,
    Timestamp,
}

impl Format<InvalidArgument> for Formatter_ {
    fn prepare<'a>(subject: &'a InvalidArgument) -> String {
        let message_part = match *subject {
            InvalidArgument::ApplicationUser_AccessModifier => "AccessModifier",
            InvalidArgument::ApplicationUser_Email => "ApplicationUser_Email",
            InvalidArgument::ApplicationUser_Id => "ApplicationUser_Id",
            InvalidArgument::ApplicationUser_Nickname => "ApplicationUser_Nickname",
            InvalidArgument::ApplicationUser_Password => "ApplicationUser_Password",
            InvalidArgument::ApplicationUser_VisabilityModifier => "VisabilityModifier",
            InvalidArgument::ApplicationUserAccessRefreshTokenEncrypted => "ApplicationUserAccessRefreshToken_DeserializedForm",
            InvalidArgument::ApplicationUserAccessTokenEncrypted => "ApplicationUserAccessTokenEncrypted",
            InvalidArgument::ApplicationUserAuthorizationToken_Value => "ApplicationUserAuthorizationToken_Value",
            InvalidArgument::ApplicationUserDevice_Id => "ApplicationUserDevice_Id",
            InvalidArgument::ApplicationUserRegistrationToken_Value => "ApplicationUserRegistrationToken_Value",
            InvalidArgument::ApplicationUserResetPasswordToken_Value => "ApplicationUserResetPasswordToken_Value",
            InvalidArgument::Channel_Id => "Channel_Id",
            InvalidArgument::Channel_Name => "Channel_Name",
            InvalidArgument::HttpHeader => "HttpHeader",
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