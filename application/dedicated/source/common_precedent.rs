pub enum CommonPrecedent {
    Channel__IsClose,
    Channel__LinkedNameAlreadyExist,
    Channel__NameAlreadyExist,
    Channel__NotFound,
    Channel__UserIsOwner,
    ChannelPublication1__IsAlreadyDeleted,
    ChannelPublication1__NotFound,
    ChannelPublication1Commentary__NotFound,
    ChannelPublication1Mark__AlreadyExist,
    ChannelPublication1Mark__NotFound,
    ChannelPublication1Token__AlreadyExpired,
    ChannelSubscription__AlreadyExist,
    ChannelSubscription__NotFound,
    ChannelSubscriptionToken__AlreadyExpired,
    ChannelToken__AlreadyExpired,
    ChannelToken__InvalidChannelOwnerDefinition,
    ParallelExecution,
    QuantityLimiter__ExceededOwnedChannelsQuantity,
    User__EmailAlreadyExist,
    User__IsNotChannelOwner,
    User__NicknameAlreadyExist,
    User__NotFound,
    User__WrongEmailOrNicknameOrPassword,
    UserAccessRefreshToken__AlreadyExpired,
    UserAccessRefreshToken__NotFound,
    UserAccessToken__AlreadyExpired,
    UserAuthorizationToken__AlreadyExpired,
    UserAuthorizationToken__NotFound,
    UserAuthorizationToken__TimeToResendHasNotCome,
    UserAuthorizationToken__WrongValue,
    UserRegistrationToken__AlreadyApproved,
    UserRegistrationToken__AlreadyExpired,
    UserRegistrationToken__IsNotApproved,
    UserRegistrationToken__NotFound,
    UserRegistrationToken__TimeToResendHasNotCome,
    UserRegistrationToken__WrongValue,
    UserResetPasswordToken__AlreadyApproved,
    UserResetPasswordToken__AlreadyExpired,
    UserResetPasswordToken__IsNotApproved,
    UserResetPasswordToken__NotFound,
    UserResetPasswordToken__TimeToResendHasNotCome,
    UserResetPasswordToken__WrongValue,
}
macro_rules! enum_from {
    ($visability:vis enum $enum_name:ident { $($enum:ident :: $enum_variant:ident $({ $($enum_variant_field:ident : $enum_variant_field_type:ty),* $(,)? })?),* $(,)? }) => {
        const _: () = {
            $(
                let _ = move |r#enum: $enum| -> () {
                    match r#enum {
                        $enum::$enum_variant => (),
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
