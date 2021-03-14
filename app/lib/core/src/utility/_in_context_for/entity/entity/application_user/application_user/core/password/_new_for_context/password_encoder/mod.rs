use argon2::Config;
use crate::entity::entity::application_user::application_user::core::password_hash::PasswordHash;
use crate::entity::entity::application_user::application_user::core::password::Password;
use uuid::Uuid;

pub struct PasswordEncoder;

impl<'outer> PasswordEncoder {      // TODO отрабатывает за 320 млсекунд, как увеличить скорость, https://users.rust-lang.org/t/which-crate-should-i-use-for-argon2/26090  // TODO CREATE CUSTOM CONFIG ?
    pub fn encode(password: &'outer Password) -> String {
        let config: Config = Config::default(); 

        return argon2::hash_encoded(password.get_value().as_bytes(), Uuid::new_v4().as_bytes(), &config).unwrap();  // TODO error
    }

    pub fn is_valid(password: &'outer str, password_hash: &'outer PasswordHash) -> bool {
        return argon2::verify_encoded(password_hash.get_value(), password.as_bytes()).unwrap();  // TODO error
    }
}