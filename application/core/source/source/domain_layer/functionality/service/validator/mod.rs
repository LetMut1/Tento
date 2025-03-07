mod channel__description;
mod channel__id;
mod channel__linked_name;
mod channel__name;
mod channel__orientation;
mod user__email;
mod user__id;
mod user__nickname;
mod user__password;
mod user_authorization_token__value;
mod user_device__id;
mod user_registration_token__value;
mod channel_publication1__id;
mod user_reset_password_token__value;
mod channel_publication1__images_pathes;
mod channel_publication1__text;
use std::marker::PhantomData;
pub struct Validator<S> {
    _subject: PhantomData<S>,
}
