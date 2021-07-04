use crate::domain_layer::entity::entity::application_user::_core::password::Password;

pub struct Validator;

impl Validator {
    pub fn is_valid<'outer_a>(password: &'outer_a Password) -> bool {
        return !password.get_value().contains(' ') && password.get_value().chars().count() > 7;     // TODO усилить пароль (ввести обязательность цифр,  и так далее)
    }
}