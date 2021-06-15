use argon2::Config;
use crate::entity::entity::application_user::_core::password_hash::PasswordHash;
use crate::entity::entity::application_user::_core::password::Password;
use uuid::Uuid;

pub struct PasswordEncoder;

impl PasswordEncoder {      // TODO отрабатывает за 320 млсекунд, как увеличить скорость, https://users.rust-lang.org/t/which-crate-should-i-use-for-argon2/26090  // TODO CREATE CUSTOM CONFIG ?
    pub fn encode<'outer_a>(password: &'outer_a Password) -> String {
        let config: Config = Config::default(); 

        return argon2::hash_encoded(password.get_value().as_bytes(), Uuid::new_v4().as_bytes(), &config).unwrap();  // TODO error
    }

    pub fn is_valid<'outer_a>(password: &'outer_a Password, password_hash: &'outer_a PasswordHash) -> bool {
        return argon2::verify_encoded(password_hash.get_value(), password.get_value().as_bytes()).unwrap();  // TODO error
    }
}