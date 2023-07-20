use std::marker::PhantomData;

pub struct PostgresqlRepository<E> {
    _entity: PhantomData<E>,
}

pub mod by {
    use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
    use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
    use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
    use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
    use crate::domain_layer::data::entity::channel::Channel_Id;
    use crate::domain_layer::data::entity::channel::Channel_VisabilityModifier;
    use crate::domain_layer::data::entity::channel::Channel_Name;

    pub struct By1<'a> {
        pub application_user_nickname: &'a ApplicationUser_Nickname,
    }

    pub struct By2<'a> {
        pub application_user_email: &'a ApplicationUser_Email,
    }

    pub struct By3 {
        pub application_user_id: ApplicationUser_Id,
    }

    pub struct By4<'a> {
        pub application_user_id: ApplicationUser_Id,
        pub application_user_device_id: &'a ApplicationUserDevice_Id,
    }

    pub struct By5<'a> {
        pub application_user_email: &'a ApplicationUser_Email,
        pub application_user_device_id: &'a ApplicationUserDevice_Id,
    }

    pub struct By6 {
        pub channel_id: Channel_Id,
    }

    pub struct By7<'a> {
        pub channel_name: &'a Channel_Name,
    }

    pub struct By8 {
        pub channel_inner_link_from: Channel_Id,
    }

    pub struct By9 {
        pub channel_outer_link_from: Channel_Id,
    }

    pub struct By10 {
        pub application_user_id: ApplicationUser_Id,
        pub channel_id: Channel_Id,
    }

    pub struct By11<'a> {
        pub application_user_id: ApplicationUser_Id,
        pub channel_name: &'a Channel_Name,
        pub requery_channel_name: &'a Option<Channel_Name>,
        pub channel_visability_modifier: Channel_VisabilityModifier,
    }

    pub struct By12<'a> {
        pub application_user_id: ApplicationUser_Id,
        pub channel_name: &'a Channel_Name,
        pub requery_channel_name: &'a Option<Channel_Name>,
    }

    pub struct By13 {
        pub application_user_id: ApplicationUser_Id,
        pub requery_channel_id: Option<Channel_Id>,
    }
}

pub mod update {
    use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ExpiresAt;
    use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_ObfuscationValue;
    use crate::domain_layer::data::entity::application_user_access_refresh_token::ApplicationUserAccessRefreshToken_UpdatedAt;
    use crate::domain_layer::data::entity::application_user_access_token::ApplicationUserAccessToken_Id;
    use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_CanBeResentFrom;
    use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_ExpiresAt;
    use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
    use crate::domain_layer::data::entity::application_user_authorization_token::ApplicationUserAuthorizationToken_WrongEnterTriesQuantity;
    use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
    use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
    use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_ExpiresAt;
    use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_IsApproved;
    use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
    use crate::domain_layer::data::entity::application_user_registration_token::ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
    use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
    use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
    use crate::domain_layer::data::entity::application_user::ApplicationUser_PasswordHash;

    pub struct Update1<'a> {
        pub application_user_password_hash: &'a ApplicationUser_PasswordHash,
    }

    pub struct Update2<'a> {
        pub application_user_access_token_id: &'a ApplicationUserAccessToken_Id,
        pub application_user_access_refresh_token_obfuscation_value: &'a ApplicationUserAccessRefreshToken_ObfuscationValue,
        pub application_user_access_refresh_token_expires_at: ApplicationUserAccessRefreshToken_ExpiresAt,
        pub application_user_access_refresh_token_updated_at: ApplicationUserAccessRefreshToken_UpdatedAt,
    }

    pub struct Update3<'a> {
        pub application_user_authorization_token_value: &'a ApplicationUserAuthorizationToken_Value,
        pub application_user_authorization_token_wrong_enter_tries_quantity: ApplicationUserAuthorizationToken_WrongEnterTriesQuantity,
        pub application_user_authorization_token_expires_at: ApplicationUserAuthorizationToken_ExpiresAt,
        pub application_user_authorization_token_can_be_resent_from: ApplicationUserAuthorizationToken_CanBeResentFrom
    }

    pub struct Update4<'a> {
        pub application_user_authorization_token_value: &'a ApplicationUserAuthorizationToken_Value,
        pub application_user_authorization_token_wrong_enter_tries_quantity: ApplicationUserAuthorizationToken_WrongEnterTriesQuantity,
        pub application_user_authorization_token_expires_at: ApplicationUserAuthorizationToken_ExpiresAt,
    }

    pub struct Update5 {
        pub application_user_authorization_token_can_be_resent_from: ApplicationUserAuthorizationToken_CanBeResentFrom
    }

    pub struct Update6 {
        pub application_user_authorization_token_wrong_enter_tries_quantity: ApplicationUserAuthorizationToken_WrongEnterTriesQuantity,
    }

    pub struct Update7<'a> {
        pub application_user_registration_token_value: &'a ApplicationUserRegistrationToken_Value,
        pub application_user_registration_token_wrong_enter_tries_quantity: ApplicationUserRegistrationToken_WrongEnterTriesQuantity,
        pub application_user_registration_token_is_approved: ApplicationUserRegistrationToken_IsApproved,
        pub application_user_registration_token_expires_at: ApplicationUserRegistrationToken_ExpiresAt,
        pub application_user_registration_token_can_be_resent_from: ApplicationUserRegistrationToken_CanBeResentFrom,
    }

    pub struct Update8 {
        pub application_user_registration_token_can_be_resent_from: ApplicationUserRegistrationToken_CanBeResentFrom,
    }

    pub struct Update9<'a> {
        pub application_user_registration_token_value: &'a ApplicationUserRegistrationToken_Value,
        pub application_user_registration_token_wrong_enter_tries_quantity: ApplicationUserRegistrationToken_WrongEnterTriesQuantity,
        pub application_user_registration_token_is_approved: ApplicationUserRegistrationToken_IsApproved,
        pub application_user_registration_token_expires_at: ApplicationUserRegistrationToken_ExpiresAt,
    }

    pub struct Update10 {
        pub application_user_registration_token_wrong_enter_tries_quantity: ApplicationUserRegistrationToken_WrongEnterTriesQuantity,
    }

    pub struct Update11 {
        pub application_user_registration_token_is_approved: ApplicationUserRegistrationToken_IsApproved,
    }
}