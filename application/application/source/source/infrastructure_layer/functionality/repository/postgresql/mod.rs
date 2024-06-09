pub mod application_user;
pub mod application_user_access_refresh_token;
pub mod application_user_authorization_token;
pub mod application_user_device;
pub mod application_user_registration_token;
pub mod application_user_reset_password_token;
pub mod channel;
pub mod channel_inner_link;
pub mod channel_outer_link;
pub mod channel_subscription;
pub mod common;

use std::marker::PhantomData;

pub struct PostgresqlRepository<E> {
    _entity: PhantomData<E>,
}

pub mod by {
    pub struct By1<'a> {
        pub application_user_nickname: &'a str,
    }

    pub struct By2<'a> {
        pub application_user_email: &'a str,
    }

    pub struct By3 {
        pub application_user_id: i64,
    }

    pub struct By4<'a> {
        pub application_user_id: i64,
        pub application_user_device_id: &'a str,
    }

    pub struct By5<'a> {
        pub application_user_email: &'a str,
        pub application_user_device_id: &'a str,
    }

    pub struct By6 {
        pub channel_id: i64,
    }

    pub struct By7<'a> {
        pub channel_name: &'a str,
    }

    pub struct By8 {
        pub channel_inner_link_from: i64,
    }

    pub struct By9 {
        pub channel_outer_link_from: i64,
    }

    pub struct By10 {
        pub application_user_id: i64,
        pub channel_id: i64,
    }

    pub struct By11<'a> {
        pub application_user_id: i64,
        pub channel_name: &'a str,
        pub requery_channel_name: &'a Option<String>,
        pub channel_visability_modifier: i16,
    }

    pub struct By12<'a> {
        pub application_user_id: i64,
        pub channel_name: &'a str,
        pub requery_channel_name: &'a Option<String>,
    }

    pub struct By13 {
        pub application_user_id: i64,
        pub requery_channel_id: Option<i64>,
    }
}

pub mod update {
    pub struct Update1<'a> {
        pub application_user_password_hash: &'a str,
    }

    pub struct Update2<'a> {
        pub application_user_access_token_id: &'a str,
        pub application_user_access_refresh_token_obfuscation_value: &'a str,
        pub application_user_access_refresh_token_expires_at: i64,
        pub application_user_access_refresh_token_updated_at: i64,
    }

    pub struct Update3<'a> {
        pub application_user_authorization_token_value: &'a str,
        pub application_user_authorization_token_wrong_enter_tries_quantity: i16,
        pub application_user_authorization_token_expires_at: i64,
        pub application_user_authorization_token_can_be_resent_from: i64,
    }

    pub struct Update4<'a> {
        pub application_user_authorization_token_value: &'a str,
        pub application_user_authorization_token_wrong_enter_tries_quantity: i16,
        pub application_user_authorization_token_expires_at: i64,
    }

    pub struct Update5 {
        pub application_user_authorization_token_can_be_resent_from: i64,
    }

    pub struct Update6 {
        pub application_user_authorization_token_wrong_enter_tries_quantity: i16,
    }

    pub struct Update7<'a> {
        pub application_user_registration_token_value: &'a str,
        pub application_user_registration_token_wrong_enter_tries_quantity: i16,
        pub application_user_registration_token_is_approved: bool,
        pub application_user_registration_token_expires_at: i64,
        pub application_user_registration_token_can_be_resent_from: i64,
    }

    pub struct Update8 {
        pub application_user_registration_token_can_be_resent_from: i64,
    }

    pub struct Update9<'a> {
        pub application_user_registration_token_value: &'a str,
        pub application_user_registration_token_wrong_enter_tries_quantity: i16,
        pub application_user_registration_token_is_approved: bool,
        pub application_user_registration_token_expires_at: i64,
    }

    pub struct Update10 {
        pub application_user_registration_token_wrong_enter_tries_quantity: i16,
    }

    pub struct Update11 {
        pub application_user_registration_token_is_approved: bool,
    }

    pub struct Update12<'a> {
        pub application_user_reset_password_token_value: &'a str,
        pub application_user_reset_password_token_wrong_enter_tries_quantity: i16,
        pub application_user_reset_password_token_is_approved: bool,
        pub application_user_reset_password_token_expires_at: i64,
        pub application_user_reset_password_token_can_be_resent_from: i64,
    }

    pub struct Update13 {
        pub application_user_reset_password_token_can_be_resent_from: i64,
    }

    pub struct Update14<'a> {
        pub application_user_reset_password_token_value: &'a str,
        pub application_user_reset_password_token_wrong_enter_tries_quantity: i16,
        pub application_user_reset_password_token_is_approved: bool,
        pub application_user_reset_password_token_expires_at: i64,
    }

    pub struct Update15 {
        pub application_user_reset_password_token_wrong_enter_tries_quantity: i16,
    }

    pub struct Update16 {
        pub application_user_reset_password_token_is_approved: bool,
    }
}

pub mod insert {
    pub struct Insert1 {
        pub application_user_email: String,
        pub application_user_nickname: String,
        pub application_user_password_hash: String,
    }

    pub struct Insert2<'a> {
        pub application_user_id: i64,
        pub application_user_device_id: &'a str,
        pub application_user_access_token_id: &'a str,
        pub application_user_access_refresh_token_obfuscation_value: String,
        pub application_user_access_refresh_token_expires_at: i64,
        pub application_user_access_refresh_token_updated_at: i64,
    }

    pub struct Insert3<'a> {
        pub application_user_id: i64,
        pub application_user_device_id: &'a str,
        pub application_user_authorization_token_value: String,
        pub application_user_authorization_token_wrong_enter_tries_quantity: i16,
        pub application_user_authorization_token_expires_at: i64,
        pub application_user_authorization_token_can_be_resent_from: i64,
    }

    pub struct Insert4 {
        pub application_user_device_id: String,
        pub application_user_id: i64,
    }

    pub struct Insert5<'a> {
        pub application_user_email: &'a str,
        pub application_user_device_id: &'a str,
        pub application_user_registration_token_value: String,
        pub application_user_registration_token_wrong_enter_tries_quantity: i16,
        pub application_user_registration_token_is_approved: bool,
        pub application_user_registration_token_expires_at: i64,
        pub application_user_registration_token_can_be_resent_from: i64,
    }

    pub struct Insert6<'a> {
        pub application_user_id: i64,
        pub application_user_device_id: &'a str,
        pub application_user_reset_password_token_value: String,
        pub application_user_reset_password_token_wrong_enter_tries_quantity: i16,
        pub application_user_reset_password_token_is_approved: bool,
        pub application_user_reset_password_token_expires_at: i64,
        pub application_user_reset_password_token_can_be_resent_from: i64,
    }

    pub struct Insert7 {
        pub channel_owner: i64,
        pub channel_name: String,
        pub channel_linked_name: String,
        pub channel_description: Option<String>,
        pub channel_access_modifier: i16,
        pub channel_visability_modifier: i16,
        pub channel_orientation: Vec<i16>,
        pub channel_cover_image_path: Option<String>,
        pub channel_background_image_path: Option<String>,
        pub channel_subscribers_quantity: i64,
        pub channel_marks_quantity: i64,
        pub channel_viewing_quantity: i64,
    }

    pub struct Insert8 {
        pub channel_inner_link_from: i64,
        pub channel_inner_link_to: i64,
    }

    pub struct Insert9 {
        pub channel_outer_link_from: i64,
        pub channel_outer_link_alias: String,
        pub channel_outer_link_address: String,
    }

    pub struct Insert10 {
        pub application_user_id: i64,
        pub channel_id: i64,
    }
}
