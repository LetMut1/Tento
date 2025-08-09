// It is necessary to specify permanent target endian in purpose to obtain a permanent hash-function result.
#![cfg(
    all(
        target_endian = "little",
        target_family = "unix"
    )
)]
mod application_layer;
mod domain_layer;
mod infrastructure_layer;
mod presentation_layer;
#[allow(unused_imports)]
pub(crate) use self::infrastructure_layer::data::aggregate_error::{
    new_indefinite_argument,
    new_invalid_argument,
    new_logic,
    new_logic_invalid_socket_address,
    new_logic_out_of_range,
    new_logic_unreachable_state,
    new_logic_value_already_exist,
    new_logic_value_does_not_exist,
    new_runtime,
    option_into_logic_out_of_range,
    option_into_logic_value_does_not_exist,
    option_return_logic_invalid_socket_address,
    option_return_logic_out_of_range,
    option_return_logic_unreachable_state,
    option_return_logic_value_does_not_exist,
    result_into_indefinite_argument,
    result_into_logic,
    result_into_runtime,
    result_return_indefinite_argument,
    result_return_logic,
    result_return_runtime,
};
use self::{
    infrastructure_layer::{
        data::aggregate_error::AggregateError,
        functionality::service::formatter::Formatter,
    },
    presentation_layer::functionality::command::Command,
};
pub const BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY: u64 = 5;
pub const BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_QUANTITY: usize = 5000;
pub const BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY: u64 = 3;
pub const BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_QUANTITY: usize = 10;
const _: () = {
    static_assertions::const_assert!(BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY > 0);
    static_assertions::const_assert!(BACKGROUND_COMMON_DATABASE_TASK_EXECUTION_QUANTITY > 0);
    static_assertions::const_assert!(BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_INTERVAL_SECONDS_QUANTITY > 0);
    static_assertions::const_assert!(BACKGROUND_COMMON_EMAIL_SENDING_TASK_EXECUTION_QUANTITY > 0);
    ()
};
// The type is 'Result<(), ()>' but not '()' to return a success/error exit code but not only success exit code.
fn main() -> Result<(), ()> {
    if let Result::Err(aggregate_error) = Command::process() {
        println!("{}", Formatter::<AggregateError>::format(&aggregate_error));
        return Result::Err(());
    }
    return Result::Ok(());
}
