pub mod application_user_registration_token__wrong_enter_tries_quantity;
pub mod application_user_reset_password_token__wrong_enter_tries_quantity;

use std::marker::PhantomData;

pub struct Incrementor<S> {
    _subject: PhantomData<S>,
}
