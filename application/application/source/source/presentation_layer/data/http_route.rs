#[cfg(feature = "manual_testing")]
use extern_crate::const_format::concatcp;

pub struct HttpRoute;

impl HttpRoute {
    pub const APPLICATION_USER__CHECK_NICKNAME_FOR_EXISTING: &'static str = "/1/1";
    pub const APPLICATION_USER__CHECK_EMAIL_FOR_EXISTING: &'static str = "/1/2";
    pub const APPLICATION_USER__REGISTER_BY_FIRST_STEP: &'static str = "/1/3";
    pub const APPLICATION_USER__REGISTER_BY_SECOND_STEP: &'static str = "/1/4";
    pub const APPLICATION_USER__REGISTER_BY_LAST_STEP: &'static str = "/1/5";
    pub const APPLICATION_USER__SEND_EMAIL_FOR_REGISTER: &'static str = "/1/6";
    pub const APPLICATION_USER__AUTHORIZE_BY_FIRST_STEP: &'static str = "/1/7";
    pub const APPLICATION_USER__AUTHORIZE_BY_LAST_STEP: &'static str = "/1/8";
    pub const APPLICATION_USER__SEND_EMAIL_FOR_AUTHORIZE: &'static str = "/1/9";
    pub const APPLICATION_USER__RESET_PASSWORD_BY_FIRST_STEP: &'static str = "/1/10";
    pub const APPLICATION_USER__RESET_PASSWORD_BY_SECOND_STEP: &'static str = "/1/11";
    pub const APPLICATION_USER__RESET_PASSWORD_BY_LAST_STEP: &'static str = "/1/12";
    pub const APPLICATION_USER__SEND_EMAIL_FOR_RESET_PASSWORD: &'static str = "/1/13";
    pub const APPLICATION_USER__REFRESH_ACCESS_TOKEN: &'static str = "/1/14";
    pub const APPLICATION_USER__DEAUTHORIZE_FROM_ONE_DEVICE: &'static str = "/1/15";
    pub const APPLICATION_USER__DEAUTHORIZE_FROM_ALL_DEVICE: &'static str = "/1/16";
    pub const CHANNEL__GET_ONE_BY_ID: &'static str = "/1/17";
    pub const CHANNEL__GET_MANY_BY_NAME_IN_SUBSCRIPTIONS: &'static str = "/1/18";
    pub const CHANNEL__GET_MANY_BY_SUBSCRIPTION: &'static str = "/1/19";
    pub const CHANNEL__GET_MANY_PUBLIC_BY_NAME: &'static str = "/1/20";
    pub const CHANNEL_SUBSCRIPTION__CREATE: &'static str = "/1/21";
}

#[cfg(feature = "manual_testing")]
impl HttpRoute {
    const CONCATENATING_PART: &'static str = "_";
    pub const APPLICATION_USER__CHECK_NICKNAME_FOR_EXISTING_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__CHECK_NICKNAME_FOR_EXISTING,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__CHECK_EMAIL_FOR_EXISTING_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__CHECK_EMAIL_FOR_EXISTING,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__REGISTER_BY_FIRST_STEP_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__REGISTER_BY_FIRST_STEP,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__REGISTER_BY_SECOND_STEP_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__REGISTER_BY_SECOND_STEP,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__REGISTER_BY_LAST_STEP_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__REGISTER_BY_LAST_STEP,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__SEND_EMAIL_FOR_REGISTER_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__SEND_EMAIL_FOR_REGISTER,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__AUTHORIZE_BY_FIRST_STEP_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__AUTHORIZE_BY_FIRST_STEP,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__AUTHORIZE_BY_LAST_STEP_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__AUTHORIZE_BY_LAST_STEP,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__SEND_EMAIL_FOR_AUTHORIZE_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__SEND_EMAIL_FOR_AUTHORIZE,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__RESET_PASSWORD_BY_FIRST_STEP_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__RESET_PASSWORD_BY_FIRST_STEP,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__RESET_PASSWORD_BY_SECOND_STEP_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__RESET_PASSWORD_BY_SECOND_STEP,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__RESET_PASSWORD_BY_LAST_STEP_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__RESET_PASSWORD_BY_LAST_STEP,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__SEND_EMAIL_FOR_RESET_PASSWORD_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__SEND_EMAIL_FOR_RESET_PASSWORD,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__REFRESH_ACCESS_TOKEN_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__REFRESH_ACCESS_TOKEN,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__DEAUTHORIZE_FROM_ONE_DEVICE_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__DEAUTHORIZE_FROM_ONE_DEVICE,
        HttpRoute::CONCATENATING_PART
    );
    pub const APPLICATION_USER__DEAUTHORIZE_FROM_ALL_DEVICE_: &'static str = concatcp!(
        HttpRoute::APPLICATION_USER__DEAUTHORIZE_FROM_ALL_DEVICE,
        HttpRoute::CONCATENATING_PART
    );
    pub const CHANNEL__GET_ONE_BY_ID_: &'static str = concatcp!(
        HttpRoute::CHANNEL__GET_ONE_BY_ID,
        HttpRoute::CONCATENATING_PART
    );
    pub const CHANNEL__GET_MANY_BY_NAME_IN_SUBSCRIPTIONS_: &'static str = concatcp!(
        HttpRoute::CHANNEL__GET_MANY_BY_NAME_IN_SUBSCRIPTIONS,
        HttpRoute::CONCATENATING_PART
    );
    pub const CHANNEL__GET_MANY_BY_SUBSCRIPTION_: &'static str = concatcp!(
        HttpRoute::CHANNEL__GET_MANY_BY_SUBSCRIPTION,
        HttpRoute::CONCATENATING_PART
    );
    pub const CHANNEL__GET_MANY_PUBLIC_BY_NAME_: &'static str = concatcp!(
        HttpRoute::CHANNEL__GET_MANY_PUBLIC_BY_NAME,
        HttpRoute::CONCATENATING_PART
    );
    pub const CHANNEL_SUBSCRIPTION__CREATE_: &'static str = concatcp!(
        HttpRoute::CHANNEL_SUBSCRIPTION__CREATE,
        HttpRoute::CONCATENATING_PART
    );
}
