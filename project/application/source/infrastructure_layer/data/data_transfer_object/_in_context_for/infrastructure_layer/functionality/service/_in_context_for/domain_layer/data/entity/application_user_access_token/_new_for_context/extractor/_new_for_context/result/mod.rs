use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken;

pub enum Result {
    ApplicationUserAccessToken {
        application_user_access_token: ApplicationUserAccessToken<'static>
    },
    ApplicationUserAccessTokenAlreadyExpired,
    /// Not yet used due to the fact that there is no such flow yet. More
    /// information in ApplicationUserAccessTokenBlackList entity.
    ApplicationUserAccessTokenInApplicationUserAccessTokenBlackList
}