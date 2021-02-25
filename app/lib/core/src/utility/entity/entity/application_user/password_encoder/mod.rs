use argon2::Config;
use uuid::Uuid;

pub struct PasswordEncoder;

impl<'outer> PasswordEncoder {      // TODO отрабатывает за 320 млсекунд, как увеличить скорость, https://users.rust-lang.org/t/which-crate-should-i-use-for-argon2/26090  // TODO CREATE CUSTOM CONFIG ?
    pub fn encode(password: &'outer String) -> String {
        let config: Config = Config::default(); 

        return argon2::hash_encoded(password.as_bytes(), Uuid::new_v4().as_bytes(), &config).unwrap();  // TODO error
    }

    pub fn is_valid(password: &'outer String, password_hash: &'outer String) -> bool {
        return argon2::verify_encoded(password_hash.as_str(), password.as_bytes()).unwrap();  // TODO error
    }
}