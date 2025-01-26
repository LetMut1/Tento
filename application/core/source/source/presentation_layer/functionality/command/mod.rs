use crate::{
    application_layer::functionality::command_processor::{
        CommandProcessor,
        CreateFixtures,
        RemoveIncompliteState,
        RunServer,
    },
    infrastructure_layer::data::aggregate_error::AggregateError,
};
use clap::{
    Arg,
    Command as Command_,
};
pub struct Command;
impl Command {
    pub fn process() -> Result<(), AggregateError> {
        const COMMAND_RUN_SERVER: &'static str = "run_server";
        const COMMAND_CREATE_FIXTURES: &'static str = "create_fixtures";
        const COMMAND_REMOVE_INCOMPLITE_STATE: &'static str = "remove_incomplite_state";
        const ARGUMENT_ENVIRONMENT_FILE_PATH: &'static str = "environment_configuration_file_path";
        let arg_matches = clap::command!()
            .arg_required_else_help(true)
            .arg(Arg::new(ARGUMENT_ENVIRONMENT_FILE_PATH).required(true).long(ARGUMENT_ENVIRONMENT_FILE_PATH))
            .subcommand_required(true)
            .subcommand(Command_::new(COMMAND_RUN_SERVER))
            .subcommand(Command_::new(COMMAND_CREATE_FIXTURES))
            .subcommand(Command_::new(COMMAND_REMOVE_INCOMPLITE_STATE))
            .get_matches();
        let environment_configuration_file_path = crate::option_return_result_logic_unreachable_state!(
            arg_matches.get_one::<String>(ARGUMENT_ENVIRONMENT_FILE_PATH)
        );
        let subcommand_arg_matches = crate::option_return_result_logic_unreachable_state!(arg_matches.subcommand());
        return match subcommand_arg_matches {
            (COMMAND_RUN_SERVER, _) => CommandProcessor::<RunServer>::process(environment_configuration_file_path.as_str()),
            (COMMAND_CREATE_FIXTURES, _) => CommandProcessor::<CreateFixtures>::process(environment_configuration_file_path.as_str()),
            (COMMAND_REMOVE_INCOMPLITE_STATE, _) => CommandProcessor::<RemoveIncompliteState>::process(),
            _ => {
               Result::Err(crate::new_logic_unreachable_state!())
            }
        };
    }
}
