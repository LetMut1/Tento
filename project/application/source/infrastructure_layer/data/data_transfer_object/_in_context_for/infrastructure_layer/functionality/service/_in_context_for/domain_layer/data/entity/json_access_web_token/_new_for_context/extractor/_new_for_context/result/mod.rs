use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;

pub enum Result {
    ApplicationUserAccessToken {
        application_user_access_token: ApplicationUserAccessToken<'static>
    },
    ApplicationUserAccessTokenAlreadyExpired,
    ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList
}