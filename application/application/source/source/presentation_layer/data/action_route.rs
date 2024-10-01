#[cfg(feature = "manual_testing")]
use const_format::concatcp;
pub const ACTION_ROUTE: ActionRoute = ActionRoute {
    application_user___authorization: ApplicationUser__Authorization {
        check_nickname_for_existing: ApplicationUser__Authorization::CHECK_NICKNAME_FOR_EXISTING,
        check_email_for_existing: ApplicationUser__Authorization::CHECK_EMAIL_FOR_EXISTING,
        regisgter_by_first_step: ApplicationUser__Authorization::REGISTER_BY_FIRST_STEP,
        regisgter_by_second_step: ApplicationUser__Authorization::REGISTER_BY_SECOND_STEP,
        regisgter_by_last_step: ApplicationUser__Authorization::REGISTER_BY_LAST_STEP,
        send_email_for_register: ApplicationUser__Authorization::SEND_EMAIL_FOR_REGISTER,
        authorize_by_first_step: ApplicationUser__Authorization::AUTHORIZE_BY_FIRST_STEP,
        authorize_by_last_step: ApplicationUser__Authorization::AUTHORIZE_BY_LAST_STEP,
        send_email_for_authorize: ApplicationUser__Authorization::SEND_EMAIL_FOR_AUTHORIZE,
        reset_password_by_first_step: ApplicationUser__Authorization::RESET_PASSWORD_BY_FIRST_STEP,
        reset_password_by_second_step: ApplicationUser__Authorization::RESET_PASSWORD_BY_SECOND_STEP,
        reset_password_by_last_step: ApplicationUser__Authorization::RESET_PASSWORD_BY_LAST_STEP,
        send_email_for_reset_password: ApplicationUser__Authorization::SEND_EMAIL_FOR_RESET_PASSWORD,
        refresh_access_token: ApplicationUser__Authorization::REFRESH_ACCESS_TOKEN,
        deauthorize_from_one_device: ApplicationUser__Authorization::DEAUTHORIZE_FROM_ONE_DEVICE,
        deauthorize_from_all_devices: ApplicationUser__Authorization::DEAUTHORIZE_FROM_ALL_DEVICES,
        #[cfg(feature = "manual_testing")]
        check_nickname_for_existing_: ApplicationUser__Authorization::CHECK_NICKNAME_FOR_EXISTING_,
        #[cfg(feature = "manual_testing")]
        check_email_for_existing_: ApplicationUser__Authorization::CHECK_EMAIL_FOR_EXISTING_,
        #[cfg(feature = "manual_testing")]
        regisgter_by_first_step_: ApplicationUser__Authorization::REGISTER_BY_FIRST_STEP_,
        #[cfg(feature = "manual_testing")]
        regisgter_by_second_step_: ApplicationUser__Authorization::REGISTER_BY_SECOND_STEP_,
        #[cfg(feature = "manual_testing")]
        regisgter_by_last_step_: ApplicationUser__Authorization::REGISTER_BY_LAST_STEP_,
        #[cfg(feature = "manual_testing")]
        send_email_for_register_: ApplicationUser__Authorization::SEND_EMAIL_FOR_REGISTER_,
        #[cfg(feature = "manual_testing")]
        authorize_by_first_step_: ApplicationUser__Authorization::AUTHORIZE_BY_FIRST_STEP_,
        #[cfg(feature = "manual_testing")]
        authorize_by_last_step_: ApplicationUser__Authorization::AUTHORIZE_BY_LAST_STEP_,
        #[cfg(feature = "manual_testing")]
        send_email_for_authorize_: ApplicationUser__Authorization::SEND_EMAIL_FOR_AUTHORIZE_,
        #[cfg(feature = "manual_testing")]
        reset_password_by_first_step_: ApplicationUser__Authorization::RESET_PASSWORD_BY_FIRST_STEP_,
        #[cfg(feature = "manual_testing")]
        reset_password_by_second_step_: ApplicationUser__Authorization::RESET_PASSWORD_BY_SECOND_STEP_,
        #[cfg(feature = "manual_testing")]
        reset_password_by_last_step_: ApplicationUser__Authorization::RESET_PASSWORD_BY_LAST_STEP_,
        #[cfg(feature = "manual_testing")]
        send_email_for_reset_password_: ApplicationUser__Authorization::SEND_EMAIL_FOR_RESET_PASSWORD_,
        #[cfg(feature = "manual_testing")]
        refresh_access_token_: ApplicationUser__Authorization::REFRESH_ACCESS_TOKEN_,
        #[cfg(feature = "manual_testing")]
        deauthorize_from_one_device_: ApplicationUser__Authorization::DEAUTHORIZE_FROM_ONE_DEVICE_,
        #[cfg(feature = "manual_testing")]
        deauthorize_from_all_devices_: ApplicationUser__Authorization::DEAUTHORIZE_FROM_ALL_DEVICES_,
    },
    channel___base: Channel__Base {
        get_one_by_id: Channel__Base::GET_ONE_BY_ID,
        get_many_by_name_in_subscription: Channel__Base::GET_MANY_BY_NAME_IN_SUBSCRIPTIONS,
        get_many_by_subscription: Channel__Base::GET_MANY_BY_SUBSCRIPTION,
        get_many_public_by_name: Channel__Base::GET_MANY_PUBLIC_BY_NAME,
        create: Channel__Base::CREATE,
        check_name_for_existing: Channel__Base::CHECK_NAME_FOR_EXISTING,
        check_linked_name_for_existing: Channel__Base::CHECK_LINKED_NAME_FOR_EXISTING,
        #[cfg(feature = "manual_testing")]
        get_one_by_id_: Channel__Base::GET_ONE_BY_ID_,
        #[cfg(feature = "manual_testing")]
        get_many_by_name_in_subscription_: Channel__Base::GET_MANY_BY_NAME_IN_SUBSCRIPTIONS_,
        #[cfg(feature = "manual_testing")]
        get_many_by_subscription_: Channel__Base::GET_MANY_BY_SUBSCRIPTION_,
        #[cfg(feature = "manual_testing")]
        get_many_public_by_name_: Channel__Base::GET_MANY_PUBLIC_BY_NAME_,
        #[cfg(feature = "manual_testing")]
        create_: Channel__Base::CREATE_,
        #[cfg(feature = "manual_testing")]
        check_name_for_existing_: Channel__Base::CHECK_NAME_FOR_EXISTING_,
        #[cfg(feature = "manual_testing")]
        check_linked_name_for_existing_: Channel__Base::CHECK_LINKED_NAME_FOR_EXISTING_,
    },
    channel_subscription___base: ChannelSubscription__Base {
        create: ChannelSubscription__Base::CREATE,
        #[cfg(feature = "manual_testing")]
        create_: ChannelSubscription__Base::CREATE_,
    },
};
pub struct ActionRoute {
    pub application_user___authorization: ApplicationUser__Authorization,
    pub channel___base: Channel__Base,
    pub channel_subscription___base: ChannelSubscription__Base,
}
#[cfg(feature = "manual_testing")]
impl ActionRoute {
    const CONCATENATING_PART: &'static str = "_";
}
pub struct ApplicationUser__Authorization {
    pub check_nickname_for_existing: &'static str,
    pub check_email_for_existing: &'static str,
    pub regisgter_by_first_step: &'static str,
    pub regisgter_by_second_step: &'static str,
    pub regisgter_by_last_step: &'static str,
    pub send_email_for_register: &'static str,
    pub authorize_by_first_step: &'static str,
    pub authorize_by_last_step: &'static str,
    pub send_email_for_authorize: &'static str,
    pub reset_password_by_first_step: &'static str,
    pub reset_password_by_second_step: &'static str,
    pub reset_password_by_last_step: &'static str,
    pub send_email_for_reset_password: &'static str,
    pub refresh_access_token: &'static str,
    pub deauthorize_from_one_device: &'static str,
    pub deauthorize_from_all_devices: &'static str,
    #[cfg(feature = "manual_testing")]
    pub check_nickname_for_existing_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub check_email_for_existing_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub regisgter_by_first_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub regisgter_by_second_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub regisgter_by_last_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub send_email_for_register_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub authorize_by_first_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub authorize_by_last_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub send_email_for_authorize_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub reset_password_by_first_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub reset_password_by_second_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub reset_password_by_last_step_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub send_email_for_reset_password_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub refresh_access_token_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub deauthorize_from_one_device_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub deauthorize_from_all_devices_: &'static str,
}
impl ApplicationUser__Authorization {
    const AUTHORIZE_BY_FIRST_STEP: &'static str = "/1/7";
    const AUTHORIZE_BY_LAST_STEP: &'static str = "/1/8";
    const CHECK_EMAIL_FOR_EXISTING: &'static str = "/1/2";
    const CHECK_NICKNAME_FOR_EXISTING: &'static str = "/1/1";
    const DEAUTHORIZE_FROM_ALL_DEVICES: &'static str = "/1/16";
    const DEAUTHORIZE_FROM_ONE_DEVICE: &'static str = "/1/15";
    const REFRESH_ACCESS_TOKEN: &'static str = "/1/14";
    const REGISTER_BY_FIRST_STEP: &'static str = "/1/3";
    const REGISTER_BY_LAST_STEP: &'static str = "/1/5";
    const REGISTER_BY_SECOND_STEP: &'static str = "/1/4";
    const RESET_PASSWORD_BY_FIRST_STEP: &'static str = "/1/10";
    const RESET_PASSWORD_BY_LAST_STEP: &'static str = "/1/12";
    const RESET_PASSWORD_BY_SECOND_STEP: &'static str = "/1/11";
    const SEND_EMAIL_FOR_AUTHORIZE: &'static str = "/1/9";
    const SEND_EMAIL_FOR_REGISTER: &'static str = "/1/6";
    const SEND_EMAIL_FOR_RESET_PASSWORD: &'static str = "/1/13";
}
#[cfg(feature = "manual_testing")]
impl ApplicationUser__Authorization {
    const AUTHORIZE_BY_FIRST_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::AUTHORIZE_BY_FIRST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const AUTHORIZE_BY_LAST_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::AUTHORIZE_BY_LAST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const CHECK_EMAIL_FOR_EXISTING_: &'static str = concatcp!(
        ApplicationUser__Authorization::CHECK_EMAIL_FOR_EXISTING,
        ActionRoute::CONCATENATING_PART
    );
    const CHECK_NICKNAME_FOR_EXISTING_: &'static str = concatcp!(
        ApplicationUser__Authorization::CHECK_NICKNAME_FOR_EXISTING,
        ActionRoute::CONCATENATING_PART
    );
    const DEAUTHORIZE_FROM_ALL_DEVICES_: &'static str = concatcp!(
        ApplicationUser__Authorization::DEAUTHORIZE_FROM_ALL_DEVICES,
        ActionRoute::CONCATENATING_PART
    );
    const DEAUTHORIZE_FROM_ONE_DEVICE_: &'static str = concatcp!(
        ApplicationUser__Authorization::DEAUTHORIZE_FROM_ONE_DEVICE,
        ActionRoute::CONCATENATING_PART
    );
    const REFRESH_ACCESS_TOKEN_: &'static str = concatcp!(
        ApplicationUser__Authorization::REFRESH_ACCESS_TOKEN,
        ActionRoute::CONCATENATING_PART
    );
    const REGISTER_BY_FIRST_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::REGISTER_BY_FIRST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const REGISTER_BY_LAST_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::REGISTER_BY_LAST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const REGISTER_BY_SECOND_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::REGISTER_BY_SECOND_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const RESET_PASSWORD_BY_FIRST_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::RESET_PASSWORD_BY_FIRST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const RESET_PASSWORD_BY_LAST_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::RESET_PASSWORD_BY_LAST_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const RESET_PASSWORD_BY_SECOND_STEP_: &'static str = concatcp!(
        ApplicationUser__Authorization::RESET_PASSWORD_BY_SECOND_STEP,
        ActionRoute::CONCATENATING_PART
    );
    const SEND_EMAIL_FOR_AUTHORIZE_: &'static str = concatcp!(
        ApplicationUser__Authorization::SEND_EMAIL_FOR_AUTHORIZE,
        ActionRoute::CONCATENATING_PART
    );
    const SEND_EMAIL_FOR_REGISTER_: &'static str = concatcp!(
        ApplicationUser__Authorization::SEND_EMAIL_FOR_REGISTER,
        ActionRoute::CONCATENATING_PART
    );
    const SEND_EMAIL_FOR_RESET_PASSWORD_: &'static str = concatcp!(
        ApplicationUser__Authorization::SEND_EMAIL_FOR_RESET_PASSWORD,
        ActionRoute::CONCATENATING_PART
    );
}
pub struct Channel__Base {
    pub get_one_by_id: &'static str,
    pub get_many_by_name_in_subscription: &'static str,
    pub get_many_by_subscription: &'static str,
    pub get_many_public_by_name: &'static str,
    pub create: &'static str,
    pub check_name_for_existing: &'static str,
    pub check_linked_name_for_existing: &'static str,
    #[cfg(feature = "manual_testing")]
    pub get_one_by_id_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub get_many_by_name_in_subscription_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub get_many_by_subscription_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub get_many_public_by_name_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub create_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub check_name_for_existing_: &'static str,
    #[cfg(feature = "manual_testing")]
    pub check_linked_name_for_existing_: &'static str,
}
impl Channel__Base {
    const GET_MANY_BY_NAME_IN_SUBSCRIPTIONS: &'static str = "/1/18";
    const GET_MANY_BY_SUBSCRIPTION: &'static str = "/1/19";
    const GET_MANY_PUBLIC_BY_NAME: &'static str = "/1/20";
    const GET_ONE_BY_ID: &'static str = "/1/17";
    const CREATE: &'static str = "/1/22";
    const CHECK_NAME_FOR_EXISTING: &'static str = "/1/23";
    const CHECK_LINKED_NAME_FOR_EXISTING: &'static str = "/1/24";
}
#[cfg(feature = "manual_testing")]
impl Channel__Base {
    const GET_MANY_BY_NAME_IN_SUBSCRIPTIONS_: &'static str = concatcp!(
        Channel__Base::GET_MANY_BY_NAME_IN_SUBSCRIPTIONS,
        ActionRoute::CONCATENATING_PART
    );
    const GET_MANY_BY_SUBSCRIPTION_: &'static str = concatcp!(
        Channel__Base::GET_MANY_BY_SUBSCRIPTION,
        ActionRoute::CONCATENATING_PART
    );
    const GET_MANY_PUBLIC_BY_NAME_: &'static str = concatcp!(
        Channel__Base::GET_MANY_PUBLIC_BY_NAME,
        ActionRoute::CONCATENATING_PART
    );
    const GET_ONE_BY_ID_: &'static str = concatcp!(
        Channel__Base::GET_ONE_BY_ID,
        ActionRoute::CONCATENATING_PART
    );
    const CREATE_: &'static str = concatcp!(
        Channel__Base::CREATE,
        ActionRoute::CONCATENATING_PART
    );
    const CHECK_NAME_FOR_EXISTING_: &'static str = concatcp!(
        Channel__Base::CHECK_NAME_FOR_EXISTING,
        ActionRoute::CONCATENATING_PART
    );
    const CHECK_LINKED_NAME_FOR_EXISTING_: &'static str = concatcp!(
        Channel__Base::CHECK_LINKED_NAME_FOR_EXISTING,
        ActionRoute::CONCATENATING_PART
    );
}
pub struct ChannelSubscription__Base {
    pub create: &'static str,
    #[cfg(feature = "manual_testing")]
    pub create_: &'static str,
}
impl ChannelSubscription__Base {
    const CREATE: &'static str = "/1/21";
}
#[cfg(feature = "manual_testing")]
impl ChannelSubscription__Base {
    const CREATE_: &'static str = concatcp!(
        ChannelSubscription__Base::CREATE,
        ActionRoute::CONCATENATING_PART
    );
}
pub enum ActionRoute_ {
    ApplicationUser__Authorization {
        application_user___authorization: ApplicationUser__Authorization_,
    },
    Channel__Base {
        channel___base: Channel__Base_,
    },
    ChannelSubscription__Base {
        channel_subscription___base: ChannelSubscription__Base_,
    },
}
pub enum ApplicationUser__Authorization_ {
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
pub enum Channel__Base_ {
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
pub enum ChannelSubscription__Base_ {
    Create,
    #[cfg(feature = "manual_testing")]
    Create_,
}
