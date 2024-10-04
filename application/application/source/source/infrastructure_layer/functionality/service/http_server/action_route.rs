#[cfg(feature = "manual_testing")]
use const_format::concatcp;
pub enum ActionRoute {
    UserAuthorization {
        user_authorization: UserAuthorization,
    },
    Channel {
        channel: Channel,
    },
    ChannelSubscription {
        channel_subscription: ChannelSubscription,
    },
}
#[cfg(feature = "manual_testing")]
impl ActionRoute {
    const CONCATENATING_PART: &'static str = "_";
}
pub enum UserAuthorization {
    CheckNicknameForExisting,
    CheckEmailForExisting,
    RegisterByFirstStep,
    RegisterBySecondStep,
    RegisterByLastStep,
    SendEmailForRegister,
    AuthorizeByFirstStep,
    AuthorizeByLastStep,
    SendEmailForAuthorize,
    ResetPasswordByFirstStep,
    ResetPasswordBySecondStep,
    ResetPasswordByLastStep,
    SendEmailForResetPassword,
    RefreshAccessToken,
    DeauthorizeFromOneDevice,
    DeauthorizeFromAllDevices,
    #[cfg(feature = "manual_testing")]
    CheckNicknameForExisting_,
    #[cfg(feature = "manual_testing")]
    CheckEmailForExisting_,
    #[cfg(feature = "manual_testing")]
    RegisterByFirstStep_,
    #[cfg(feature = "manual_testing")]
    RegisterBySecondStep_,
    #[cfg(feature = "manual_testing")]
    RegisterByLastStep_,
    #[cfg(feature = "manual_testing")]
    SendEmailForRegister_,
    #[cfg(feature = "manual_testing")]
    AuthorizeByFirstStep_,
    #[cfg(feature = "manual_testing")]
    AuthorizeByLastStep_,
    #[cfg(feature = "manual_testing")]
    SendEmailForAuthorize_,
    #[cfg(feature = "manual_testing")]
    ResetPasswordByFirstStep_,
    #[cfg(feature = "manual_testing")]
    ResetPasswordBySecondStep_,
    #[cfg(feature = "manual_testing")]
    ResetPasswordByLastStep_,
    #[cfg(feature = "manual_testing")]
    SendEmailForResetPassword_,
    #[cfg(feature = "manual_testing")]
    RefreshAccessToken_,
    #[cfg(feature = "manual_testing")]
    DeauthorizeFromOneDevice_,
    #[cfg(feature = "manual_testing")]
    DeauthorizeFromAllDevices_,
}
impl UserAuthorization {
    pub const AUTHORIZE_BY_FIRST_STEP: &'static str = "/1/7";
    pub const AUTHORIZE_BY_LAST_STEP: &'static str = "/1/8";
    pub const CHECK_EMAIL_FOR_EXISTING: &'static str = "/1/2";
    pub const CHECK_NICKNAME_FOR_EXISTING: &'static str = "/1/1";
    pub const DEAUTHORIZE_FROM_ALL_DEVICES: &'static str = "/1/16";
    pub const DEAUTHORIZE_FROM_ONE_DEVICE: &'static str = "/1/15";
    pub const REFRESH_ACCESS_TOKEN: &'static str = "/1/14";
    pub const REGISTER_BY_FIRST_STEP: &'static str = "/1/3";
    pub const REGISTER_BY_LAST_STEP: &'static str = "/1/5";
    pub const REGISTER_BY_SECOND_STEP: &'static str = "/1/4";
    pub const RESET_PASSWORD_BY_FIRST_STEP: &'static str = "/1/10";
    pub const RESET_PASSWORD_BY_LAST_STEP: &'static str = "/1/12";
    pub const RESET_PASSWORD_BY_SECOND_STEP: &'static str = "/1/11";
    pub const SEND_EMAIL_FOR_AUTHORIZE: &'static str = "/1/9";
    pub const SEND_EMAIL_FOR_REGISTER: &'static str = "/1/6";
    pub const SEND_EMAIL_FOR_RESET_PASSWORD: &'static str = "/1/13";
}
#[cfg(feature = "manual_testing")]
impl UserAuthorization {
    pub const AUTHORIZE_BY_FIRST_STEP_: &'static str = concatcp!(
        UserAuthorization::AUTHORIZE_BY_FIRST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    pub const AUTHORIZE_BY_LAST_STEP_: &'static str = concatcp!(
        UserAuthorization::AUTHORIZE_BY_LAST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    pub const CHECK_EMAIL_FOR_EXISTING_: &'static str = concatcp!(
        UserAuthorization::CHECK_EMAIL_FOR_EXISTING,
        ActionRoute::CONCATENATING_PART
    );
    pub const CHECK_NICKNAME_FOR_EXISTING_: &'static str = concatcp!(
        UserAuthorization::CHECK_NICKNAME_FOR_EXISTING,
        ActionRoute::CONCATENATING_PART
    );
    pub const DEAUTHORIZE_FROM_ALL_DEVICES_: &'static str = concatcp!(
        UserAuthorization::DEAUTHORIZE_FROM_ALL_DEVICES,
        ActionRoute::CONCATENATING_PART
    );
    pub const DEAUTHORIZE_FROM_ONE_DEVICE_: &'static str = concatcp!(
        UserAuthorization::DEAUTHORIZE_FROM_ONE_DEVICE,
        ActionRoute::CONCATENATING_PART
    );
    pub const REFRESH_ACCESS_TOKEN_: &'static str = concatcp!(
        UserAuthorization::REFRESH_ACCESS_TOKEN,
        ActionRoute::CONCATENATING_PART
    );
    pub const REGISTER_BY_FIRST_STEP_: &'static str = concatcp!(
        UserAuthorization::REGISTER_BY_FIRST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    pub const REGISTER_BY_LAST_STEP_: &'static str = concatcp!(
        UserAuthorization::REGISTER_BY_LAST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    pub const REGISTER_BY_SECOND_STEP_: &'static str = concatcp!(
        UserAuthorization::REGISTER_BY_SECOND_STEP,
        ActionRoute::CONCATENATING_PART
    );
    pub const RESET_PASSWORD_BY_FIRST_STEP_: &'static str = concatcp!(
        UserAuthorization::RESET_PASSWORD_BY_FIRST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    pub const RESET_PASSWORD_BY_LAST_STEP_: &'static str = concatcp!(
        UserAuthorization::RESET_PASSWORD_BY_LAST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    pub const RESET_PASSWORD_BY_SECOND_STEP_: &'static str = concatcp!(
        UserAuthorization::RESET_PASSWORD_BY_SECOND_STEP,
        ActionRoute::CONCATENATING_PART
    );
    pub const SEND_EMAIL_FOR_AUTHORIZE_: &'static str = concatcp!(
        UserAuthorization::SEND_EMAIL_FOR_AUTHORIZE,
        ActionRoute::CONCATENATING_PART
    );
    pub const SEND_EMAIL_FOR_REGISTER_: &'static str = concatcp!(
        UserAuthorization::SEND_EMAIL_FOR_REGISTER,
        ActionRoute::CONCATENATING_PART
    );
    pub const SEND_EMAIL_FOR_RESET_PASSWORD_: &'static str = concatcp!(
        UserAuthorization::SEND_EMAIL_FOR_RESET_PASSWORD,
        ActionRoute::CONCATENATING_PART
    );
}
pub enum Channel {
    GetOneById,
    GetManyByNameInSubscriptions,
    GetManyBySubscription,
    GetManyPublicByName,
    Create,
    CheckNameForExisting,
    CheckLinkedNameForExisting,
    #[cfg(feature = "manual_testing")]
    GetOneById_,
    #[cfg(feature = "manual_testing")]
    GetManyByNameInSubscriptions_,
    #[cfg(feature = "manual_testing")]
    GetManyBySubscription_,
    #[cfg(feature = "manual_testing")]
    GetManyPublicByName_,
    #[cfg(feature = "manual_testing")]
    Create_,
    #[cfg(feature = "manual_testing")]
    CheckNameForExisting_,
    #[cfg(feature = "manual_testing")]
    CheckLinkedNameForExisting_,
}
impl Channel {
    pub const GET_MANY_BY_NAME_IN_SUBSCRIPTIONS: &'static str = "/1/18";
    pub const GET_MANY_BY_SUBSCRIPTION: &'static str = "/1/19";
    pub const GET_MANY_PUBLIC_BY_NAME: &'static str = "/1/20";
    pub const GET_ONE_BY_ID: &'static str = "/1/17";
    pub const CREATE: &'static str = "/1/22";
    pub const CHECK_NAME_FOR_EXISTING: &'static str = "/1/23";
    pub const CHECK_LINKED_NAME_FOR_EXISTING: &'static str = "/1/24";
}
#[cfg(feature = "manual_testing")]
impl Channel {
    pub const GET_MANY_BY_NAME_IN_SUBSCRIPTIONS_: &'static str = concatcp!(
        Channel::GET_MANY_BY_NAME_IN_SUBSCRIPTIONS,
        ActionRoute::CONCATENATING_PART
    );
    pub const GET_MANY_BY_SUBSCRIPTION_: &'static str = concatcp!(
        Channel::GET_MANY_BY_SUBSCRIPTION,
        ActionRoute::CONCATENATING_PART
    );
    pub const GET_MANY_PUBLIC_BY_NAME_: &'static str = concatcp!(
        Channel::GET_MANY_PUBLIC_BY_NAME,
        ActionRoute::CONCATENATING_PART
    );
    pub const GET_ONE_BY_ID_: &'static str = concatcp!(
        Channel::GET_ONE_BY_ID,
        ActionRoute::CONCATENATING_PART
    );
    pub const CREATE_: &'static str = concatcp!(
        Channel::CREATE,
        ActionRoute::CONCATENATING_PART
    );
    pub const CHECK_NAME_FOR_EXISTING_: &'static str = concatcp!(
        Channel::CHECK_NAME_FOR_EXISTING,
        ActionRoute::CONCATENATING_PART
    );
    pub const CHECK_LINKED_NAME_FOR_EXISTING_: &'static str = concatcp!(
        Channel::CHECK_LINKED_NAME_FOR_EXISTING,
        ActionRoute::CONCATENATING_PART
    );
}
pub enum ChannelSubscription {
    Create,
    #[cfg(feature = "manual_testing")]
    Create_,
}
impl ChannelSubscription {
    pub const CREATE: &'static str = "/1/21";
}
#[cfg(feature = "manual_testing")]
impl ChannelSubscription {
    pub const CREATE_: &'static str = concatcp!(
        ChannelSubscription::CREATE,
        ActionRoute::CONCATENATING_PART
    );
}