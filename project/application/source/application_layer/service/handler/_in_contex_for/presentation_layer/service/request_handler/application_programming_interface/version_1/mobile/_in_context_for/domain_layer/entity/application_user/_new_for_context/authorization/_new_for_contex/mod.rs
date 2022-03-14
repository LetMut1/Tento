pub mod check_email_for_existing;
pub mod check_nickaname_for_existing;
pub mod log_in_by_first_step;
pub mod log_in_by_last_step;
pub mod log_out_from_all_devices;
pub mod log_out;
pub mod refresh_json_access_web_token;
pub mod register_by_first_step;
pub mod register_by_last_step;
pub mod reset_password_by_first_step;
pub mod reset_password_by_last_step;
pub mod send_email_for_log_in;
pub mod send_email_for_register;
pub mod send_email_for_reset_password;

#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod check_email_for_existing_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod check_nickaname_for_existing_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod log_in_by_first_step_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod log_in_by_last_step_;
// #[cfg(feature="facilitate_non_automatic_functional_testing")]
// pub mod log_out_;
// #[cfg(feature="facilitate_non_automatic_functional_testing")]
// pub mod log_out_from_all_devices_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod refresh_json_access_web_token_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod register_by_first_step_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod register_by_last_step_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod reset_password_by_first_step_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod reset_password_by_last_step_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod send_email_for_log_in_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod send_email_for_register_;
#[cfg(feature="facilitate_non_automatic_functional_testing")]
pub mod send_email_for_reset_password_;