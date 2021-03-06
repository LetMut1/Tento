use crate::entity::core::date_time::DateTime;
use crate::entity::core::uuid_v4::UuidV4;
use crate::entity::entity::application_user_registration_confirmation_token::core::value::Value;
use crate::entity::entity::application_user::application_user::ApplicationUser;
use maybe_owned::MaybeOwned;

pub struct ApplicationUserRegistrationConfirmationToken<'this, 'outer: 'this> {   // TODO Redis disc
    id: UuidV4<'outer>,
    application_user_id: MaybeOwned<'outer, UuidV4<'outer>>,
    value: Value<'this>,
    expired_at: DateTime<'this>
}

impl<'this, 'outer: 'this> ApplicationUserRegistrationConfirmationToken<'this, 'outer> {
    // pub fn new(application_user: &'outer ApplicationUser<'outer>) -> Self {
    //     return Self {
    //         id: UuidV4::new(),
    //         application_user_id: MaybeOwned::Borrowed(application_user.get_id()),
    //         value: UuidV4::new(),    // TODO какое значени лучше генерировать
    //         expired_at:
    //     };
    // }
}