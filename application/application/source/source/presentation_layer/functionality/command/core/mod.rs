#![allow(
    clippy::collapsible_else_if,
    clippy::collapsible_match,
    clippy::explicit_into_iter_loop,
    clippy::module_inception,
    clippy::needless_continue,
    clippy::needless_lifetimes,
    clippy::needless_return,
    clippy::new_without_default,
    clippy::redundant_pattern_matching,
    clippy::redundant_static_lifetimes,
    clippy::single_match_else,
    clippy::string_add,
    clippy::too_many_arguments,
    clippy::trait_duplication_in_bounds,
    clippy::unused_unit,
    clippy::empty_enum,
    clippy::let_unit_value
)]
#![deny(
    clippy::unnecessary_cast,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::fallible_impl_from,
    clippy::float_cmp_const,
    clippy::from_iter_instead_of_collect,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wild_err_arm,
    clippy::mem_forget,
    clippy::missing_enforced_import_renames,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_for_each,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::rc_mutex,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::string_add_assign,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values
)]
use application::{
    application_layer::functionality::command_processor::{
        create_fixtures::CreateFixtures,
        remove_incomplite_state::RemoveIncompliteState,
        run_server::RunServer,
        CommandProcessor,
    },
    infrastructure_layer::{
        data::{
            alternative_workflow::AlternativeWorkflow,
            auditor::Backtrace,
        },
        functionality::service::formatter::Formatter,
    },
};
use clap::{
    command,
    Command,
};
use std::error::Error as StdError;
const RUN_SERVER: &'static str = "run_server";
const CREATE_FIXTURES: &'static str = "create_fixtures";
const REMOVE_INCOMPLITE_STATE: &'static str = "remove_incomplite_state";
fn main() -> Result<(), Box<dyn StdError + 'static>> {
    if let Err(error) = Processor::process() {
        println!("{}", &error);
        return Err(error);
    }
    return Ok(());
}
struct Processor;
impl Processor {
    fn process() -> Result<(), Box<dyn StdError + 'static>> {
        let arg_matches = command!()
            .arg_required_else_help(true)
            .subcommand_required(true)
            .subcommand(Command::new(RUN_SERVER))
            .subcommand(Command::new(CREATE_FIXTURES))
            .get_matches();
        let subcommand_arg_matches = match arg_matches.subcommand() {
            Some(subcommand_arg_matches_) => subcommand_arg_matches_,
            None => {
                return Err("Exhausted list of subcommands and subcommand_required prevents `None`.".into());
            }
        };
        let alternative_workflow = match subcommand_arg_matches {
            (RUN_SERVER, _) => {
                let alternative_workflow_ = match CommandProcessor::<RunServer>::process() {
                    Ok(_) => None,
                    Err(alternative_workflow__) => Some(alternative_workflow__),
                };
                alternative_workflow_
            }
            (CREATE_FIXTURES, _) => {
                let alternative_workflow_ = match CommandProcessor::<CreateFixtures>::process() {
                    Ok(_) => None,
                    Err(alternative_workflow__) => Some(alternative_workflow__),
                };
                alternative_workflow_
            }
            (REMOVE_INCOMPLITE_STATE, _) => {
                let alternative_workflow_ = match CommandProcessor::<RemoveIncompliteState>::process() {
                    Ok(_) => None,
                    Err(alternative_workflow__) => Some(alternative_workflow__),
                };
                alternative_workflow_
            }
            _ => {
                Some(
                    AlternativeWorkflow::new_internal_error_runtime_(
                        "Unexpected subcommand.".into(),
                        Backtrace::new(
                            line!(),
                            file!(),
                        ),
                    ),
                )
            }
        };
        match alternative_workflow {
            Some(alternative_workflow_) => {
                return Err(Formatter::<AlternativeWorkflow>::format(&alternative_workflow_).into());
            }
            None => {
                return Ok(());
            }
        }
    }
}