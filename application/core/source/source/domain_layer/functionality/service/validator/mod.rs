mod channel__description;
mod channel__id;
mod channel__linked_name;
mod channel__name;
mod channel_publication1__images_pathes;
mod channel_publication1__text;
mod channel_publication1_commentary__id;
mod channel_publication1_commentary__text;
mod user__email;
mod user__id;
mod user__nickname;
mod user__password;
mod user_authorization_token__value;
mod user_device__id;
mod user_registration_token__value;
mod user_reset_password_token__value;
mod user__obfuscated_id;
use std::marker::PhantomData;
pub struct Validator<S> {
    _subject: PhantomData<S>,
}
