pub enum CommonPrecedent {
    Channel_IsClose,
    Channel_LinkedNameAlreadyExist,
    Channel_NameAlreadyExist,
    Channel_NotFound,
    ChannelPublication1_NotFound,
    ChannelPublication1Mark_AlreadyExist,
    ChannelSubscription_NotFound,
    ChannelSubscriptionToken_AlreadyExpired,
    ChannelToken_AlreadyExpired,
    ChannelToken_NotFound,
    User_EmailAlreadyExist,
    User_IsChannelOwner,
    User_IsNotChannelOwner,
    User_NicknameAlreadyExist,
    User_NotFound,
    User_WrongEmailOrNicknameOrPassword,
    User_WrongPassword,
    UserAccessRefreshToken_AlreadyExpired,
    UserAccessRefreshToken_NotFound,
    UserAccessToken_AlreadyExpired,
    UserAccessToken_InUserAccessTokenBlackList,
    UserAuthorizationToken_AlreadyExpired,
    UserAuthorizationToken_NotFound,
    UserAuthorizationToken_TimeToResendHasNotCome,
    UserAuthorizationToken_WrongValue,
    UserRegistrationToken_AlreadyApproved,
    UserRegistrationToken_AlreadyExpired,
    UserRegistrationToken_IsNotApproved,
    UserRegistrationToken_NotFound,
    UserRegistrationToken_TimeToResendHasNotCome,
    UserRegistrationToken_WrongValue,
    UserResetPasswordToken_AlreadyApproved,
    UserResetPasswordToken_AlreadyExpired,
    UserResetPasswordToken_IsNotApproved,
    UserResetPasswordToken_NotFound,
    UserResetPasswordToken_TimeToResendHasNotCome,
    UserResetPasswordToken_WrongValue,
}
macro_rules! enum_from {
    ($visability:vis enum $enum_name:ident { $($enum:ident :: $enum_variant:ident $({ $($enum_variant_field:ident : $enum_variant_field_type:ty),* $(,)? })?),* $(,)? }) => {
        const _: () = {
            $(
                let _ = move |r#enum: $enum| -> () {
                    match r#enum {
                        $enum :: $enum_variant => (),
                        _ => (),
                    }
                };
            )*
            ()
        };
        #[cfg_attr(feature = "serde_for_manual_test", derive(serde::Serialize, serde::Deserialize))]
        #[derive(bitcode::Encode, bitcode::Decode)]
        $visability enum $enum_name {
            $($enum_variant $({ $($enum_variant_field: $enum_variant_field_type,)* })?,)*
        }
    };
}
pub(crate) use enum_from;
