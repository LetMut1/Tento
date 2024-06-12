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
        pub requery_channel_name: Option<&'a str>,
        pub channel_visability_modifier: i16,
    }

    pub struct By12<'a> {
        pub application_user_id: i64,
        pub channel_name: &'a str,
        pub requery_channel_name: Option<&'a str>,
    }

    pub struct By13 {
        pub application_user_id: i64,
        pub requery_channel_id: Option<i64>,
    }
}
