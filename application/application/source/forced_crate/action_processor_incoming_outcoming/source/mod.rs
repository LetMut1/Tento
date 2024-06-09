#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    clippy::collapsible_else_if,
    clippy::collapsible_match,
    clippy::explicit_into_iter_loop,
    clippy::module_inception,
    clippy::needless_continue,
    clippy::needless_lifetimes,
    clippy::needless_return,
    clippy::new_without_default,
    clippy::redundant_pattern_matching,
    clippy::single_match_else,
    clippy::string_add,
    clippy::too_many_arguments,
    clippy::trait_duplication_in_bounds,
    clippy::unused_unit,
    clippy::empty_enum,
    clippy::let_unit_value,
    clippy::let_and_return,
    clippy::manual_range_contains,
    clippy::enum_variant_names,
    clippy::type_complexity,
    clippy::explicit_auto_deref,
    clippy::redundant_static_lifetimes,
    clippy::manual_map
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

pub mod action_processor;

use entity::channel_outer_link::ChannelOuterLink_Address;
use entity::channel_outer_link::ChannelOuterLink_Alias;
use entity::channel::Channel_ViewingQuantity;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Common1 {
    pub channel: Channel1,
    pub is_application_user_subscribed: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Channel1 {
    pub channel_id: i64,
    pub channel_name: String,
    pub channel_linked_name: String,
    pub channel_access_modifier: i16,
    pub channel_visability_modifier: i16,
    pub channel_cover_image_path: Option<String>,
    pub channel_background_image_path: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Channel2 {
    pub channel_owner: i64,
    pub channel_name: String,
    pub channel_linked_name: String,
    pub channel_description: Option<String>,
    pub channel_access_modifier: i16,
    pub channel_visability_modifier: i16,
    pub channel_orientation: Vec<i16>,
    pub channel_cover_image_path: Option<String>,
    pub channel_background_image_path: Option<String>,
    pub channel_subscribers_quantity: i64,
    pub channel_marks_quantity: i64,
    pub channel_viewing_quantity: Channel_ViewingQuantity,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelInnerLink1 {
    pub channel_inner_link_to: i64,
}

#[derive(Serialize, Deserialize)]
pub struct ChannelOuterLink1 {
    pub channel_outer_link_alias: ChannelOuterLink_Alias,
    pub channel_outer_link_address: ChannelOuterLink_Address,
}