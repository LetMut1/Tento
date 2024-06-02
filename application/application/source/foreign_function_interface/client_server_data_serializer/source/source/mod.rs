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

use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_first_step::Incoming as ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_first_step::Outcoming as ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_first_step::Precedent as ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_last_step::Incoming as ApplicationUser__Authorization___AuthorizeByLastStep___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_last_step::Outcoming as ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::authorize_by_last_step::Precedent as ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::check_email_for_existing::Incoming as ApplicationUser__Authorization___CheckEmailForExisting___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::check_email_for_existing::Outcoming as ApplicationUser__Authorization___CheckEmailForExisting___Outcoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::check_nickname_for_existing::Incoming as ApplicationUser__Authorization___CheckNicknameForExisting___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::check_nickname_for_existing::Outcoming as ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::deauthorize_from_all_devices::Incoming as ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::deauthorize_from_all_devices::Precedent as ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::deauthorize_from_one_device::Incoming as ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::deauthorize_from_one_device::Precedent as ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::refresh_access_token::Incoming as ApplicationUser__Authorization___RefreshAccessToken___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::refresh_access_token::Outcoming as ApplicationUser__Authorization___RefreshAccessToken___Outcoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::refresh_access_token::Precedent as ApplicationUser__Authorization___RefreshAccessToken___Precedent_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_first_step::Incoming as ApplicationUser__Authorization___RegisterByFirstStep___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_first_step::Outcoming as ApplicationUser__Authorization___RegisterByFirstStep___Outcoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_first_step::Precedent as ApplicationUser__Authorization___RegisterByFirstStep___Precedent_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_last_step::Incoming as ApplicationUser__Authorization___RegisterByLastStep___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_last_step::Outcoming as ApplicationUser__Authorization___RegisterByLastStep___Outcoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_last_step::Precedent as ApplicationUser__Authorization___RegisterByLastStep___Precedent_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_second_step::Incoming as ApplicationUser__Authorization___RegisterBySecondStep___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::register_by_second_step::Precedent as ApplicationUser__Authorization___RegisterBySecondStep___Precedent_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_first_step::Incoming as ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_first_step::Outcoming as ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_first_step::Precedent as ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_last_step::Incoming as ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_last_step::Precedent as ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_second_step::Incoming as ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::reset_password_by_second_step::Precedent as ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_authorize::Incoming as ApplicationUser__Authorization___SendEmailForAuthorize___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_authorize::Outcoming as ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_authorize::Precedent as ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_register::Incoming as ApplicationUser__Authorization___SendEmailForRegister___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_register::Outcoming as ApplicationUser__Authorization___SendEmailForRegister___Outcoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_register::Precedent as ApplicationUser__Authorization___SendEmailForRegister___Precedent_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_reset_password::Incoming as ApplicationUser__Authorization___SendEmailForResetPassword___Incoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_reset_password::Outcoming as ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization::send_email_for_reset_password::Precedent as ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_;
use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_name_in_subscriptions::Incoming as Channel__Base___GetManyByNameInSubscriptions___Incoming_;
use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_name_in_subscriptions::Outcoming as Channel__Base___GetManyByNameInSubscriptions___Outcoming_;
use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_name_in_subscriptions::Precedent as Channel__Base___GetManyByNameInSubscriptions___Precedent_;
use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_subscription::Incoming as Channel__Base___GetManyBySubscription___Incoming_;
use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_subscription::Outcoming as Channel__Base___GetManyBySubscription___Outcoming_;
use action_processor_incoming_outcoming::action_processor::channel___base::get_many_by_subscription::Precedent as Channel__Base___GetManyBySubscription___Precedent_;
use action_processor_incoming_outcoming::action_processor::channel___base::get_many_public_by_name::Incoming as Channel__Base___GetManyPublicByName___Incoming_;
use action_processor_incoming_outcoming::action_processor::channel___base::get_many_public_by_name::Outcoming as Channel__Base___GetManyPublicByName___Outcoming_;
use action_processor_incoming_outcoming::action_processor::channel___base::get_many_public_by_name::Precedent as Channel__Base___GetManyPublicByName___Precedent_;
use action_processor_incoming_outcoming::action_processor::channel___base::get_one_by_id::Incoming as Channel__Base___GetOneById___Incoming_;
use action_processor_incoming_outcoming::action_processor::channel___base::get_one_by_id::Outcoming as Channel__Base___GetOneById___Outcoming_;
use action_processor_incoming_outcoming::action_processor::channel___base::get_one_by_id::Precedent as Channel__Base___GetOneById___Precedent_;
use action_processor_incoming_outcoming::action_processor::channel_subscription___base::create::Incoming as ChannelSubscription__Base___Create___Incoming_;
use action_processor_incoming_outcoming::action_processor::channel_subscription___base::create::Precedent as ChannelSubscription__Base___Create___Precedent_;
use entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use entity::channel::Channel_Id;
use entity::channel::Channel_Name;
use libc::c_char;
use libc::c_long;
use libc::c_short;
use libc::c_uchar;
use libc::size_t;
use message_pack_serializer::Serializer as Serializer_;
use serde::Deserialize;
use serde::Serialize;
use std::boxed::Box;
use std::default::Default;
use std::error::Error as StdError;
use std::ffi::CStr;
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::forget;
use std::ptr;
use std::ptr::slice_from_raw_parts_mut;
use std::result::Result;
use std::slice;
use std::slice::from_raw_parts;
use unified_report::Data;
use unified_report::UnifiedReport;
use void::Void;







// TODO rust binary ffi optimize for size.  !!!!!!!!!!!!
// https://arusahni.net/blog/2020/03/optimizing-rust-binary-size.html
// https://github.com/johnthagen/min-sized-rust !!!!!!!!!!!!!!!!!!!!!!!!!!!!!

// TODO access_modifier/visability_modifier посмотреть, как на бэкенде лежат в бд и отдаются. Здесь сделать структуру
// TODO можно ли сериализовать Incoming не со String, а со &str для подготовки converter, чтобы избежать аллокации в стринг. На большой стренге это будет сильно замедлять.







#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Result<T> {
    pub data: T,
    // If false, then it means an error occurred.
    pub is_data: bool,
}

impl<T> C_Result<T> {
    fn data(data: T) -> Self {
        return Self {
            data,
            is_data: true,
        };
    }

    fn into_raw(self) -> *mut Self {
        return Box::into_raw(
            Box::new(
                self
            )
        );
    }
}

impl<T> C_Result<T>
where
    T: Default
{
    fn error() -> Self {
        return Self {
            data: T::default(),
            is_data: false,
        };
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Option<T> {
    pub data: T,
    // If false, then it means it it None.
    pub is_data: bool,
}

impl<T> C_Option<T> {
    fn data(data: T) -> Self {
        return Self {
            data,
            is_data: true,
        };
    }
}

impl<T> C_Option<T>
where
    T: Default
{
    fn none() -> Self {
        return Self {
            data: T::default(),
            is_data: false,
        };
    }
}

impl<T> Default for C_Option<T>
where
    T: Default
{
    fn default() -> Self {
        return Self::none();
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct C_UnifiedReport<D, P> {
    pub target: C_Data<D>,
    pub precedent: P,
    // If false, then it means we have to work with precedent.
    pub is_target: bool,
}

impl<D, P> C_UnifiedReport<D, P>
where
    P: Default
{
    fn target(target: C_Data<D>) -> Self {
        return Self {
            target,
            precedent: P::default(),
            is_target: true,
        };
    }
}

impl<D, P> C_UnifiedReport<D, P>
where
    D: Default
{
    fn precedent(precedent: P) -> Self {
        return Self {
            target: C_Data::<D>::default(),
            precedent,
            is_target: false,
        };
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct C_Data<T> {
    pub filled: T,
    // If false, then it means data is empty.
    pub is_filled: bool,
}

impl<T> C_Data<T> {
    fn filled(filled: T) -> Self {
        return C_Data {
            filled,
            is_filled: true,
        };
    }
}

impl<T> C_Data<T>
where
    T: Default
{
    fn empty() -> Self {
        return C_Data {
            filled: T::default(),
            is_filled: false,
        };
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_String {
    pub pointer: *mut c_char,
}

impl C_String {
    fn to_string(self) -> Result<String, Box<dyn StdError + 'static>> {
        if self.pointer.is_null() {
            return Err("There should not be a null-pointer.".into());
        }

        let c_str = unsafe {
            CStr::from_ptr(self.pointer as *const _)
        };

        let c_string = c_str.to_str()?.to_string();

        return Ok(c_string);
    }
}

impl Default for C_String {
    fn default() -> Self {
        return Self {
            pointer: ptr::null_mut()
        };
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C_Vector<T> {
    pointer: *mut T,
    length: size_t,
}

impl<T> Default for C_Vector<T> {
    fn default() -> Self {
        return Self {
            pointer: ptr::null_mut(),
            length: 0,
        };
    }
}

// Struct for simulating Void type. That is, we will use this structure
// in those moments when we would like to use the classic Void type.
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct C_Void {
    _inner: bool,
}

impl C_Void {
    fn new() -> Self {
        return Self {
            ..Default::default()
        };
    }
}

struct Allocator<S> {
    _subject: PhantomData<S>,
}

impl Allocator<C_String> {
    fn allocate(string: String) -> C_String {
        let c_string = unsafe {
            CString::from_vec_unchecked(string.into_bytes())
        };

        C_String {
            pointer: c_string.into_raw()
        }
    }

    fn deallocate(c_string: C_String) -> () {
        if c_string.pointer.is_null() {
            return ();
        }

        let _ = unsafe {
            CString::from_raw(c_string.pointer)
        };

        return ();
    }
}

impl<T> Allocator<C_Vector<T>> {
    fn allocate(vector: Vec<T>) -> C_Vector<T> {
        let mut boxed_slice = vector.into_boxed_slice();

        let self_ = C_Vector {
            pointer: boxed_slice.as_mut_ptr(),
            length: boxed_slice.len()
        };

        forget(boxed_slice);

        return self_;
    }

    fn deallocate(c_vector: C_Vector<T>) -> () {
        if c_vector.pointer.is_null() {
            return ();
        }

        let pointer = slice_from_raw_parts_mut(c_vector.pointer, c_vector.length);

        let _ = unsafe {
            Box::from_raw(pointer)
        };

        return ();
    }
}

impl<T> Allocator<C_Result<C_Vector<T>>> {
    fn deallocate(c_result: *mut C_Result<C_Vector<T>>) -> () {
        if c_result.is_null() {
            return ();
        }

        let c_result_ = unsafe {
            Box::from_raw(c_result)
        };

        if c_result_.is_data {
            Allocator::<C_Vector<T>>::deallocate(c_result_.data);
        }

        return ();
    }
}

struct Transformer<S> {
    _subject: PhantomData<S>,
}

struct ServerRequestData;

struct ServerResponseData;

impl Transformer<ServerResponseData> {
    fn transform<F, APO1, APP1, APO2, APP2>(
        vector_of_bytes: *mut C_Vector<c_uchar>,
        converter: F
    ) -> *mut C_Result<C_UnifiedReport<APO2, APP2>>
    where
        F: FnOnce(UnifiedReport<APO1, APP1>) -> Result<C_UnifiedReport<APO2, APP2>, Box<dyn StdError + 'static>>,
        APO1: for<'de> Deserialize<'de>,
        APP1: for<'de> Deserialize<'de>,
        APO2: Default,
        APP2: Default,
    {
        if vector_of_bytes.is_null() {
            return C_Result::error().into_raw();
        }

        let vector_ = unsafe {
            *vector_of_bytes
        };

        if vector_.pointer.is_null() || vector_.length == 0 {
            return C_Result::error().into_raw();
        }

        let data = unsafe {
            slice::from_raw_parts::<u8>(vector_.pointer as *mut u8, vector_.length )
        };

        let unified_report = match Serializer_::deserialize::<'_, UnifiedReport<APO1, APP1>>(data) {
            Ok(unified_report_) => unified_report_,
            Err(_) => {
                return C_Result::error().into_raw();
            }
        };

        let c_unified_report = match converter(unified_report) {
            Ok(c_unified_report_) => c_unified_report_,
            Err(_) => {
                return C_Result::error().into_raw();
            }
        };

        let c_result = C_Result::data(c_unified_report);

        return c_result.into_raw();
    }
}

impl Transformer<ServerRequestData> {
    fn transform<I1, F, I2>(
        incoming: *mut I1,
        converter: F,
    ) -> *mut C_Result<C_Vector<c_uchar>>
    where
        I1: Copy,
        F: FnOnce(I1) -> Result<I2, Box<dyn StdError + 'static>>,
        I2: Serialize,
    {
        if incoming.is_null() {
            return C_Result::error().into_raw();
        }

        let incoming_ = unsafe {
            *incoming
        };

        let incoming__ = match converter(incoming_) {
            Ok(incoming___) => incoming___,
            Err(_) => {
                return C_Result::error().into_raw();
            }
        };

        let data = match Serializer_::serialize(&incoming__) {
            Ok(data_) => data_,
            Err(_) => {
                return C_Result::error().into_raw();
            }
        };

        let c_vector = Allocator::<C_Vector<_>>::allocate(data);

        let c_result = C_Result::data(c_vector);

        return c_result.into_raw();
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Common1 {
    pub channel: Channel1,
    pub is_application_user_subscribed: bool,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Channel1 {
    pub channel_id: c_long,
    pub channel_name: C_String,
    pub channel_linked_name: C_String,
    pub channel_access_modifier: c_short,
    pub channel_visability_modifier: c_short,
    pub channel_cover_image_path: C_Option<C_String>,
    pub channel_background_image_path: C_Option<C_String>,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Channel2 {
    pub channel_owner: c_long,
    pub channel_name: C_String,
    pub channel_linked_name: C_String,
    pub channel_description: C_Option<C_String>,
    pub channel_access_modifier: c_short,
    pub channel_visability_modifier: c_short,
    pub channel_orientation: C_Vector<c_short>,
    pub channel_cover_image_path: C_Option<C_String>,
    pub channel_background_image_path: C_Option<C_String>,
    pub channel_subscribers_quantity: c_long,
    pub channel_marks_quantity: c_long,
    pub channel_viewing_quantity: c_long,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ChannelInnerLink1 {
    pub channel_inner_link_to: c_long,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ChannelOuterLink1 {
    pub channel_outer_link_alias: C_String,
    pub channel_outer_link_address: C_String,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming {
    pub application_user_device_id: C_String,
    pub application_user_email_or_application_user_nickname: C_String,
    pub application_user_password: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_first_step____serialize(
    incoming: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming| -> Result<ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming_ {
            application_user_device_id: incoming.application_user_device_id.to_string()?,
            application_user_email_or_application_user_nickname: incoming.application_user_email_or_application_user_nickname.to_string()?,
            application_user_password: incoming.application_user_password.to_string()?,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_first_step____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming, ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming {
    pub application_user_id: c_long,
    pub verification_message_sent: bool,
    pub application_user_authorization_token_can_be_resent_from: c_long,
    pub application_user_authorization_token_wrong_enter_tries_quantity: c_short,
    pub application_user_authorization_token_wrong_enter_tries_quantity_limit: c_short,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent {
    pub application_user__wrong_email_or_nickname_or_password: bool,
}


#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_first_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result {
    let converter = move |unified_report: UnifiedReport<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming_, ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming, ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming {
                            application_user_id: data__.application_user_id,
                            verification_message_sent: data__.verification_message_sent,
                            application_user_authorization_token_can_be_resent_from: data__.application_user_authorization_token_can_be_resent_from,
                            application_user_authorization_token_wrong_enter_tries_quantity: data__.application_user_authorization_token_wrong_enter_tries_quantity,
                            application_user_authorization_token_wrong_enter_tries_quantity_limit: data__.application_user_authorization_token_wrong_enter_tries_quantity_limit,
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                match precedent {
                    ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent_::ApplicationUser_WrongEmailOrNicknameOrPassword => {}
                };

                let precedent_ = ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent {
                    application_user__wrong_email_or_nickname_or_password: true,
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_first_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___AuthorizeByLastStep___Incoming {
    pub application_user_id: c_long,
    pub application_user_device_id: C_String,
    pub application_user_authorization_token_value: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_last_step____serialize(
    incoming: *mut ApplicationUser__Authorization___AuthorizeByLastStep___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___AuthorizeByLastStep___Incoming| -> Result<ApplicationUser__Authorization___AuthorizeByLastStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___AuthorizeByLastStep___Incoming_ {
            application_user_id: incoming.application_user_id,
            application_user_device_id: incoming.application_user_device_id.to_string()?,
            application_user_authorization_token_value: incoming.application_user_authorization_token_value.to_string()?,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_last_step____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___AuthorizeByLastStep___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming, ApplicationUser__Authorization___AuthorizeByLastStep___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming {
    pub application_user_access_token_encrypted: C_String,
    pub application_user_access_refresh_token_encrypted: C_String,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___AuthorizeByLastStep___Precedent {
    pub application_user_authorization_token__not_found: bool,
    pub application_user_authorization_token__already_expired: bool,
    pub application_user_authorization_token__wrong_value: ApplicationUserAuthorizationToken_WrongValue,
    pub application_user__not_found: bool,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUserAuthorizationToken_WrongValue {
    pub is_exist: bool,
    pub application_user_authorization_token_wrong_enter_tries_quantity: c_short,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_last_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result {
    let converter = move |unified_report: UnifiedReport<ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming_, ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming, ApplicationUser__Authorization___AuthorizeByLastStep___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming {
                            application_user_access_token_encrypted: Allocator::<C_String>::allocate(data__.application_user_access_token_encrypted),
                            application_user_access_refresh_token_encrypted: Allocator::<C_String>::allocate(data__.application_user_access_refresh_token_encrypted),
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_::ApplicationUserAuthorizationToken_AlreadyExpired => {
                        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent {
                            application_user_authorization_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_::ApplicationUserAuthorizationToken_NotFound => {
                        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent {
                            application_user_authorization_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_::ApplicationUserAuthorizationToken_WrongValue { application_user_authorization_token_wrong_enter_tries_quantity } => {
                        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent {
                            application_user_authorization_token__wrong_value: ApplicationUserAuthorizationToken_WrongValue {
                                is_exist: true,
                                application_user_authorization_token_wrong_enter_tries_quantity,
                            },
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_::ApplicationUser_NotFound => {
                        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent {
                            application_user__not_found: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_last_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(c_result)
    };

    if result_.is_data {
        if result_.data.is_target {
            if result_.data.target.is_filled {
                Allocator::<C_String>::deallocate(result_.data.target.filled.application_user_access_token_encrypted);

                Allocator::<C_String>::deallocate(result_.data.target.filled.application_user_access_refresh_token_encrypted);
            }
        }
    }

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___CheckEmailForExisting___Incoming {
    pub application_user_email: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____check_email_for_existing____serialize(
    incoming: *mut ApplicationUser__Authorization___CheckEmailForExisting___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___CheckEmailForExisting___Incoming| -> Result<ApplicationUser__Authorization___CheckEmailForExisting___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___CheckEmailForExisting___Incoming_ {
            application_user_email: incoming.application_user_email.to_string()?,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____check_email_for_existing____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___CheckEmailForExisting___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___CheckEmailForExisting___Outcoming, C_Void>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___CheckEmailForExisting___Outcoming {
    pub result: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____check_email_for_existing____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___CheckEmailForExisting___C_Result {
    let converter = move |unified_report: UnifiedReport<ApplicationUser__Authorization___CheckEmailForExisting___Outcoming_, Void>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___CheckEmailForExisting___Outcoming, C_Void>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___CheckEmailForExisting___Outcoming {
                            result: data__.result,
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent: _ } => {
                C_UnifiedReport::precedent(C_Void::new())
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____check_email_for_existing____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___CheckEmailForExisting___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___CheckNicknameForExisting___Incoming {
    pub application_user_nickname: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____check_nickname_for_existing____serialize(
    incoming: *mut ApplicationUser__Authorization___CheckNicknameForExisting___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___CheckNicknameForExisting___Incoming| -> Result<ApplicationUser__Authorization___CheckNicknameForExisting___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___CheckNicknameForExisting___Incoming_ {
            application_user_nickname: incoming.application_user_nickname.to_string()?,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____check_nickname_for_existing____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___CheckNicknameForExisting___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming, C_Void>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming {
    pub result: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____check_nickname_for_existing____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___CheckNicknameForExisting___C_Result {
    let converter = move |unified_report: UnifiedReport<ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming_, Void>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming, C_Void>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming {
                            result: data__.result,
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent: _ } => {
                C_UnifiedReport::precedent(C_Void::new())
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____check_nickname_for_existing____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___CheckNicknameForExisting___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming {
    pub application_user_access_token_encrypted: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_all_devices____serialize(
    incoming: *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming| -> Result<ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming_ {
            application_user_access_token_encrypted: incoming.application_user_access_token_encrypted.to_string()?,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_all_devices____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_all_devices____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result {
    let converter = move |unified_report: UnifiedReport<Void, ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_>| -> Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: _ } => {
                        C_Data::filled(C_Void::new())
                    }
                };

                C_UnifiedReport::target(data)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
                        ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent {
                            application_user_access_token__in_application_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_all_devices____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

type ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming {
    pub application_user_access_token_encrypted: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_one_device____serialize(
    incoming: *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming| -> Result<ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming_ {
            application_user_access_token_encrypted: incoming.application_user_access_token_encrypted.to_string()?,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_one_device____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_one_device____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result {
    let converter = move |unified_report: UnifiedReport<Void, ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_>| -> Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: _ } => {
                        C_Data::filled(C_Void::new())
                    }
                };

                C_UnifiedReport::target(data)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
                        ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent {
                            application_user_access_token__in_application_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_one_device____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___RefreshAccessToken___Incoming {
    pub application_user_access_token_encrypted: C_String,
    pub application_user_access_refresh_token_encrypted: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____refresh_access_token____serialize(
    incoming: *mut ApplicationUser__Authorization___RefreshAccessToken___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___RefreshAccessToken___Incoming| -> Result<ApplicationUser__Authorization___RefreshAccessToken___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___RefreshAccessToken___Incoming_ {
            application_user_access_token_encrypted: incoming.application_user_access_token_encrypted.to_string()?,
            application_user_access_refresh_token_encrypted: incoming.application_user_access_refresh_token_encrypted.to_string()?,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____refresh_access_token____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___RefreshAccessToken___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___RefreshAccessToken___Outcoming, ApplicationUser__Authorization___RefreshAccessToken___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___RefreshAccessToken___Outcoming {
    pub application_user_access_token_encrypted: C_String,
    pub application_user_access_refresh_token_encrypted: C_String,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___RefreshAccessToken___Precedent {
    pub application_user_access_refresh_token__not_found: bool,
    pub application_user_access_refresh_token__already_expired: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____refresh_access_token____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result {
    let converter = move |unified_report: UnifiedReport<ApplicationUser__Authorization___RefreshAccessToken___Outcoming_, ApplicationUser__Authorization___RefreshAccessToken___Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___RefreshAccessToken___Outcoming, ApplicationUser__Authorization___RefreshAccessToken___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___RefreshAccessToken___Outcoming {
                            application_user_access_token_encrypted: Allocator::<C_String>::allocate(data__.application_user_access_token_encrypted),
                            application_user_access_refresh_token_encrypted: Allocator::<C_String>::allocate(data__.application_user_access_refresh_token_encrypted),
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ApplicationUser__Authorization___RefreshAccessToken___Precedent_::ApplicationUserAccessRefreshToken_AlreadyExpired => {
                        ApplicationUser__Authorization___RefreshAccessToken___Precedent {
                            application_user_access_refresh_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___RefreshAccessToken___Precedent_::ApplicationUserAccessRefreshToken_NotFound => {
                        ApplicationUser__Authorization___RefreshAccessToken___Precedent {
                            application_user_access_refresh_token__not_found: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____refresh_access_token____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(c_result)
    };

    if result_.is_data {
        if result_.data.is_target {
            if result_.data.target.is_filled {
                Allocator::<C_String>::deallocate(result_.data.target.filled.application_user_access_token_encrypted);

                Allocator::<C_String>::deallocate(result_.data.target.filled.application_user_access_refresh_token_encrypted);
            }
        }
    }

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___RegisterByFirstStep___Incoming {
    pub application_user_email: C_String,
    pub application_user_device_id: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_first_step____serialize(
    incoming: *mut ApplicationUser__Authorization___RegisterByFirstStep___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___RegisterByFirstStep___Incoming| -> Result<ApplicationUser__Authorization___RegisterByFirstStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___RegisterByFirstStep___Incoming_ {
            application_user_email: incoming.application_user_email.to_string()?,
            application_user_device_id: incoming.application_user_device_id.to_string()?,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_first_step____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___RegisterByFirstStep___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___RegisterByFirstStep___Outcoming, ApplicationUser__Authorization___RegisterByFirstStep___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___RegisterByFirstStep___Outcoming {
    pub verification_message_sent: bool,
    pub application_user_registration_token_can_be_resent_from: c_long,
    pub application_user_registration_token_wrong_enter_tries_quantity: c_short,
    pub application_user_registration_token_wrong_enter_tries_quantity_limit: c_short,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___RegisterByFirstStep___Precedent {
    pub application_user__email_already_exist: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_first_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result {
    let converter = move |unified_report: UnifiedReport<ApplicationUser__Authorization___RegisterByFirstStep___Outcoming_, ApplicationUser__Authorization___RegisterByFirstStep___Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___RegisterByFirstStep___Outcoming, ApplicationUser__Authorization___RegisterByFirstStep___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___RegisterByFirstStep___Outcoming {
                            verification_message_sent: data__.verification_message_sent,
                            application_user_registration_token_can_be_resent_from: data__.application_user_registration_token_can_be_resent_from.0,
                            application_user_registration_token_wrong_enter_tries_quantity: data__.application_user_registration_token_wrong_enter_tries_quantity.0,
                            application_user_registration_token_wrong_enter_tries_quantity_limit: data__.application_user_registration_token_wrong_enter_tries_quantity_limit,
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                match precedent {
                    ApplicationUser__Authorization___RegisterByFirstStep___Precedent_::ApplicationUser_EmailAlreadyExist => {}
                };

                let precedent_ = ApplicationUser__Authorization___RegisterByFirstStep___Precedent {
                    application_user__email_already_exist: true,
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_first_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___RegisterBySecondStep___Incoming {
    pub application_user_email: C_String,
    pub application_user_device_id: C_String,
    pub application_user_registration_token_value: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_second_step____serialize(
    incoming: *mut ApplicationUser__Authorization___RegisterBySecondStep___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___RegisterBySecondStep___Incoming| -> Result<ApplicationUser__Authorization___RegisterBySecondStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___RegisterBySecondStep___Incoming_ {
            application_user_email: incoming.application_user_email.to_string()?,
            application_user_device_id: incoming.application_user_device_id.to_string()?,
            application_user_registration_token_value: incoming.application_user_registration_token_value.to_string()?,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_second_step____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___RegisterBySecondStep___C_Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___RegisterBySecondStep___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___RegisterBySecondStep___Precedent {
    pub application_user_registration_token__not_found: bool,
    pub application_user_registration_token__already_expired: bool,
    pub application_user_registration_token__already_approved: bool,
    pub application_user_registration_token__wrong_value: ApplicationUserRegistrationToken_WrongValue,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUserRegistrationToken_WrongValue {
    pub is_exist: bool,
    pub application_user_registration_token_wrong_enter_tries_quantity: c_short,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_second_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___RegisterBySecondStep___C_Result {
    let converter = move |unified_report: UnifiedReport<Void, ApplicationUser__Authorization___RegisterBySecondStep___Precedent_>| -> Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___RegisterBySecondStep___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: _ } => {
                        C_Data::filled(C_Void::new())
                    }
                };

                C_UnifiedReport::target(data)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ApplicationUser__Authorization___RegisterBySecondStep___Precedent_::ApplicationUserRegistrationToken_NotFound => {
                        ApplicationUser__Authorization___RegisterBySecondStep___Precedent {
                            application_user_registration_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___RegisterBySecondStep___Precedent_::ApplicationUserRegistrationToken_AlreadyExpired => {
                        ApplicationUser__Authorization___RegisterBySecondStep___Precedent {
                            application_user_registration_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___RegisterBySecondStep___Precedent_::ApplicationUserRegistrationToken_AlreadyApproved => {
                        ApplicationUser__Authorization___RegisterBySecondStep___Precedent {
                            application_user_registration_token__already_approved: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___RegisterBySecondStep___Precedent_::ApplicationUserRegistrationToken_WrongValue { application_user_registration_token_wrong_enter_tries_quantity } => {
                        ApplicationUser__Authorization___RegisterBySecondStep___Precedent {
                            application_user_registration_token__wrong_value: ApplicationUserRegistrationToken_WrongValue {
                                is_exist: true,
                                application_user_registration_token_wrong_enter_tries_quantity: application_user_registration_token_wrong_enter_tries_quantity.0,
                            },
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_second_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___RegisterBySecondStep___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___RegisterByLastStep___Incoming {
    pub application_user_device_id: C_String,
    pub application_user_nickname: C_String,
    pub application_user_password: C_String,
    pub application_user_email: C_String,
    pub application_user_registration_token_value: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_last_step____serialize(
    incoming: *mut ApplicationUser__Authorization___RegisterByLastStep___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___RegisterByLastStep___Incoming| -> Result<ApplicationUser__Authorization___RegisterByLastStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___RegisterByLastStep___Incoming_ {
            application_user_device_id: incoming.application_user_device_id.to_string()?,
            application_user_email: incoming.application_user_email.to_string()?,
            application_user_nickname: incoming.application_user_nickname.to_string()?,
            application_user_password: incoming.application_user_password.to_string()?,
            application_user_registration_token_value: incoming.application_user_registration_token_value.to_string()?,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_last_step____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___RegisterByLastStep___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___RegisterByLastStep___Outcoming, ApplicationUser__Authorization___RegisterByLastStep___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___RegisterByLastStep___Outcoming {
    pub application_user_access_token_encrypted: C_String,
    pub application_user_access_refresh_token_encrypted: C_String,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___RegisterByLastStep___Precedent {
    pub application_user__nickname_already_exist: bool,
    pub application_user__email_already_exist: bool,
    pub application_user_registration_token__not_found: bool,
    pub application_user_registration_token__already_expired: bool,
    pub application_user_registration_token__is_not_approved: bool,
    pub application_user_registration_token__wrong_value: bool
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_last_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result {
    let converter = move |unified_report: UnifiedReport<ApplicationUser__Authorization___RegisterByLastStep___Outcoming_, ApplicationUser__Authorization___RegisterByLastStep___Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___RegisterByLastStep___Outcoming, ApplicationUser__Authorization___RegisterByLastStep___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___RegisterByLastStep___Outcoming {
                            application_user_access_token_encrypted: Allocator::<C_String>::allocate(data__.application_user_access_token_encrypted),
                            application_user_access_refresh_token_encrypted: Allocator::<C_String>::allocate(data__.application_user_access_refresh_token_encrypted),
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ApplicationUser__Authorization___RegisterByLastStep___Precedent_::ApplicationUser_NicknameAlreadyExist => {
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent {
                            application_user__nickname_already_exist: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___RegisterByLastStep___Precedent_::ApplicationUser_EmailAlreadyExist => {
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent {
                            application_user__email_already_exist: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___RegisterByLastStep___Precedent_::ApplicationUserRegistrationToken_NotFound => {
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent {
                            application_user_registration_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___RegisterByLastStep___Precedent_::ApplicationUserRegistrationToken_AlreadyExpired => {
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent {
                            application_user_registration_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___RegisterByLastStep___Precedent_::ApplicationUserRegistrationToken_IsNotApproved => {
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent {
                            application_user_registration_token__is_not_approved: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___RegisterByLastStep___Precedent_::ApplicationUserRegistrationToken_WrongValue => {
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent {
                            application_user_registration_token__wrong_value: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_last_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(c_result)
    };

    if result_.is_data {
        if result_.data.is_target {
            if result_.data.target.is_filled {
                Allocator::<C_String>::deallocate(result_.data.target.filled.application_user_access_token_encrypted);

                Allocator::<C_String>::deallocate(result_.data.target.filled.application_user_access_refresh_token_encrypted);
            }
        }
    }

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming {
    pub application_user_email: C_String,
    pub application_user_device_id: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_first_step____serialize(
    incoming: *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming| -> Result<ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming_ {
            application_user_email: incoming.application_user_email.to_string()?,
            application_user_device_id: incoming.application_user_device_id.to_string()?,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_first_step____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming, ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming {
    pub application_user_id: c_long,
    pub verification_message_sent: bool,
    pub application_user_reset_password_token_can_be_resent_from: c_long,
    pub application_user_reset_password_token_wrong_enter_tries_quantity: c_short,
    pub application_user_reset_password_token_wrong_enter_tries_quantity_limit: c_short,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent {
    pub application_user__not_found: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_first_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result {
    let converter = move |unified_report: UnifiedReport<ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming_, ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming, ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming {
                            application_user_id: data__.application_user_id,
                            verification_message_sent: data__.verification_message_sent,
                            application_user_reset_password_token_can_be_resent_from: data__.application_user_reset_password_token_can_be_resent_from.0,
                            application_user_reset_password_token_wrong_enter_tries_quantity: data__.application_user_reset_password_token_wrong_enter_tries_quantity.0,
                            application_user_reset_password_token_wrong_enter_tries_quantity_limit: data__.application_user_reset_password_token_wrong_enter_tries_quantity_limit,
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                match precedent {
                    ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_::ApplicationUser_NotFound => {}
                };

                let precedent_ = ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent {
                    application_user__not_found: true
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_first_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming {
    pub application_user_id: c_long,
    pub application_user_device_id: C_String,
    pub application_user_reset_password_token_value: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_second_step____serialize(
    incoming: *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming| -> Result<ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming_ {
            application_user_id: incoming.application_user_id,
            application_user_device_id: incoming.application_user_device_id.to_string()?,
            application_user_reset_password_token_value: ApplicationUserResetPasswordToken_Value(incoming.application_user_reset_password_token_value.to_string()?),
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_second_step____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent {
    pub application_user_reset_password_token__not_found: bool,
    pub application_user_reset_password_token__already_expired: bool,
    pub application_user_reset_password_token__already_approved: bool,
    pub application_user_reset_password_token__wrong_value: ApplicationUserResetPasswordToken_WrongValue,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUserResetPasswordToken_WrongValue {
    pub is_exist: bool,
    pub application_user_reset_password_token_wrong_enter_tries_quantity: c_short,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_second_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result {
    let converter = move |unified_report: UnifiedReport<Void, ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_>| -> Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: _ } => {
                        C_Data::filled(C_Void::new())
                    }
                };

                C_UnifiedReport::target(data)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_::ApplicationUserResetPasswordToken_NotFound => {
                        ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent {
                            application_user_reset_password_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_::ApplicationUserResetPasswordToken_AlreadyExpired => {
                        ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent {
                            application_user_reset_password_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_::ApplicationUserResetPasswordToken_AlreadyApproved => {
                        ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent {
                            application_user_reset_password_token__already_approved: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_::ApplicationUserResetPasswordToken_WrongValue { application_user_reset_password_token_wrong_enter_tries_quantity } => {
                        ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent {
                            application_user_reset_password_token__wrong_value: ApplicationUserResetPasswordToken_WrongValue {
                                is_exist: true,
                                application_user_reset_password_token_wrong_enter_tries_quantity: application_user_reset_password_token_wrong_enter_tries_quantity.0,
                            },
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_second_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming {
    pub application_user_id: c_long,
    pub application_user_device_id: C_String,
    pub application_user_password: C_String,
    pub application_user_reset_password_token_value: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_last_step____serialize(
    incoming: *mut ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming| -> Result<ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming_ {
            application_user_id: incoming.application_user_id,
            application_user_device_id: incoming.application_user_device_id.to_string()?,
            application_user_password: incoming.application_user_password.to_string()?,
            application_user_reset_password_token_value: ApplicationUserResetPasswordToken_Value(incoming.application_user_reset_password_token_value.to_string()?),
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_last_step____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent {
    pub application_user__not_found: bool,
    pub application_user_reset_password_token__not_found: bool,
    pub application_user_reset_password_token__already_expired: bool,
    pub application_user_reset_password_token__is_not_approved: bool,
    pub application_user_reset_password_token__wrong_value: bool
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_last_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result {
    let converter = move |unified_report: UnifiedReport<Void, ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_>| -> Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: _ } => {
                        C_Data::filled(C_Void::new())
                    }
                };

                C_UnifiedReport::target(data)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_::ApplicationUser_NotFound => {
                        ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent {
                            application_user__not_found: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_::ApplicationUserResetPasswordToken_NotFound => {
                        ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent {
                            application_user_reset_password_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_::ApplicationUserResetPasswordToken_AlreadyExpired => {
                        ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent {
                            application_user_reset_password_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_::ApplicationUserResetPasswordToken_IsNotApproved => {
                        ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent {
                            application_user_reset_password_token__is_not_approved: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_::ApplicationUserResetPasswordToken_WrongValue => {
                        ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent {
                            application_user_reset_password_token__wrong_value: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_last_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___SendEmailForRegister___Incoming {
    pub application_user_email: C_String,
    pub application_user_device_id: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_register____serialize(
    incoming: *mut ApplicationUser__Authorization___SendEmailForRegister___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___SendEmailForRegister___Incoming| -> Result<ApplicationUser__Authorization___SendEmailForRegister___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___SendEmailForRegister___Incoming_ {
            application_user_email: incoming.application_user_email.to_string()?,
            application_user_device_id: incoming.application_user_device_id.to_string()?,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_register____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___SendEmailForRegister___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForRegister___Outcoming, ApplicationUser__Authorization___SendEmailForRegister___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___SendEmailForRegister___Outcoming {
    pub application_user_registration_token_can_be_resent_from: c_long,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___SendEmailForRegister___Precedent {
    pub application_user_registration_token__not_found: bool,
    pub application_user_registration_token__already_expired: bool,
    pub application_user_registration_token__already_approved: bool,
    pub application_user_registration_token__time_to_resend_has_not_come: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_register____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result {
    let converter = move |unified_report: UnifiedReport<ApplicationUser__Authorization___SendEmailForRegister___Outcoming_, ApplicationUser__Authorization___SendEmailForRegister___Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForRegister___Outcoming, ApplicationUser__Authorization___SendEmailForRegister___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___SendEmailForRegister___Outcoming {
                            application_user_registration_token_can_be_resent_from: data__.application_user_registration_token_can_be_resent_from.0,
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ApplicationUser__Authorization___SendEmailForRegister___Precedent_::ApplicationUserRegistrationToken_NotFound => {
                        ApplicationUser__Authorization___SendEmailForRegister___Precedent {
                            application_user_registration_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___SendEmailForRegister___Precedent_::ApplicationUserRegistrationToken_AlreadyExpired => {
                        ApplicationUser__Authorization___SendEmailForRegister___Precedent {
                            application_user_registration_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___SendEmailForRegister___Precedent_::ApplicationUserRegistrationToken_AlreadyApproved => {
                        ApplicationUser__Authorization___SendEmailForRegister___Precedent {
                            application_user_registration_token__already_approved: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___SendEmailForRegister___Precedent_::ApplicationUserRegistrationToken_TimeToResendHasNotCome => {
                        ApplicationUser__Authorization___SendEmailForRegister___Precedent {
                            application_user_registration_token__time_to_resend_has_not_come: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_register____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___SendEmailForAuthorize___Incoming {
    pub application_user_device_id: C_String,
    pub application_user_id: c_long,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_authorize____serialize(
    incoming: *mut ApplicationUser__Authorization___SendEmailForAuthorize___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___SendEmailForAuthorize___Incoming| -> Result<ApplicationUser__Authorization___SendEmailForAuthorize___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___SendEmailForAuthorize___Incoming_ {
            application_user_device_id: incoming.application_user_device_id.to_string()?,
            application_user_id: incoming.application_user_id,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_authorize____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___SendEmailForAuthorize___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming, ApplicationUser__Authorization___SendEmailForAuthorize___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming {
    pub application_user_authorization_token_can_be_resent_from: c_long,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___SendEmailForAuthorize___Precedent {
    pub application_user__not_found: bool,
    pub application_user_authorization_token__not_found: bool,
    pub application_user_authorization_token__already_expired: bool,
    pub application_user_authorization_token__time_to_resend_has_not_come: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_authorize____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result {
    let converter = move |unified_report: UnifiedReport<ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming_, ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming, ApplicationUser__Authorization___SendEmailForAuthorize___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming {
                            application_user_authorization_token_can_be_resent_from: data__.application_user_authorization_token_can_be_resent_from,
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_::ApplicationUser_NotFound => {
                        ApplicationUser__Authorization___SendEmailForAuthorize___Precedent {
                            application_user__not_found: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_::ApplicationUserAuthorizationToken_NotFound => {
                        ApplicationUser__Authorization___SendEmailForAuthorize___Precedent {
                            application_user_authorization_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_::ApplicationUserAuthorizationToken_AlreadyExpired => {
                        ApplicationUser__Authorization___SendEmailForAuthorize___Precedent {
                            application_user_authorization_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_::ApplicationUserAuthorizationToken_TimeToResendHasNotCome => {
                        ApplicationUser__Authorization___SendEmailForAuthorize___Precedent {
                            application_user_authorization_token__time_to_resend_has_not_come: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_authorize____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___SendEmailForResetPassword___Incoming {
    pub application_user_id: c_long,
    pub application_user_device_id: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_reset_password____serialize(
    incoming: *mut ApplicationUser__Authorization___SendEmailForResetPassword___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___SendEmailForResetPassword___Incoming| -> Result<ApplicationUser__Authorization___SendEmailForResetPassword___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___SendEmailForResetPassword___Incoming_ {
            application_user_id: incoming.application_user_id,
            application_user_device_id: incoming.application_user_device_id.to_string()?,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_reset_password____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ApplicationUser__Authorization___SendEmailForResetPassword___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming, ApplicationUser__Authorization___SendEmailForResetPassword___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming {
    pub application_user_resep_password_token_can_be_resent_from: c_long,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ApplicationUser__Authorization___SendEmailForResetPassword___Precedent {
    pub application_user__not_found: bool,
    pub application_user_reset_password_token__not_found: bool,
    pub application_user_reset_password_token__already_expired: bool,
    pub application_user_reset_password_token__already_approved: bool,
    pub application_user_reset_password_token__time_to_resend_has_not_come: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_reset_password____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result {
    let converter = move |unified_report: UnifiedReport<ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming_, ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming, ApplicationUser__Authorization___SendEmailForResetPassword___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming {
                            application_user_resep_password_token_can_be_resent_from: data__.application_user_reset_password_token_can_be_resent_from.0,
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_::ApplicationUser_NotFound => {
                        ApplicationUser__Authorization___SendEmailForResetPassword___Precedent {
                            application_user__not_found: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_::ApplicationUserResetPasswordToken_NotFound => {
                        ApplicationUser__Authorization___SendEmailForResetPassword___Precedent {
                            application_user_reset_password_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_::ApplicationUserResetPasswordToken_AlreadyExpired => {
                        ApplicationUser__Authorization___SendEmailForResetPassword___Precedent {
                            application_user_reset_password_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_::ApplicationUserResetPasswordToken_AlreadyApproved => {
                        ApplicationUser__Authorization___SendEmailForResetPassword___Precedent {
                            application_user_reset_password_token__already_approved: true,
                            ..Default::default()
                        }
                    }
                    ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_::ApplicationUserResetPasswordToken_TimeToResendHasNotCome => {
                        ApplicationUser__Authorization___SendEmailForResetPassword___Precedent {
                            application_user_reset_password_token__time_to_resend_has_not_come: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_reset_password____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Channel__Base___GetManyByNameInSubscriptions___Incoming {
    pub application_user_access_token_encrypted: C_String,
    pub channel_name: C_String,
    pub requery_channel_name: C_Option<C_String>,
    pub limit: c_short,
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_by_name_in_subscriptions____serialize(
    incoming: *mut Channel__Base___GetManyByNameInSubscriptions___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: Channel__Base___GetManyByNameInSubscriptions___Incoming| -> Result<Channel__Base___GetManyByNameInSubscriptions___Incoming_, Box<dyn StdError + 'static>> {
        let requery_channel_name = if incoming.requery_channel_name.is_data {
            Some(
                Channel_Name(
                    incoming.requery_channel_name.data.to_string()?
                )
            )
        } else {
            None
        };

        let incoming_ = Channel__Base___GetManyByNameInSubscriptions___Incoming_ {
            application_user_access_token_encrypted: incoming.application_user_access_token_encrypted.to_string()?,
            channel_name: Channel_Name(incoming.channel_name.to_string()?),
            requery_channel_name,
            limit: incoming.limit,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_by_name_in_subscriptions____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type Channel__Base___GetManyByNameInSubscriptions___C_Result = C_Result<C_UnifiedReport<Channel__Base___GetManyByNameInSubscriptions___Outcoming, Channel__Base___GetManyByNameInSubscriptions___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Channel__Base___GetManyByNameInSubscriptions___Outcoming {
    pub common_registry: C_Vector<Common1>,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Channel__Base___GetManyByNameInSubscriptions___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_by_name_in_subscriptions____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut Channel__Base___GetManyByNameInSubscriptions___C_Result {
    let converter = move |unified_report: UnifiedReport<Channel__Base___GetManyByNameInSubscriptions___Outcoming_, Channel__Base___GetManyByNameInSubscriptions___Precedent_>| -> Result<C_UnifiedReport<Channel__Base___GetManyByNameInSubscriptions___Outcoming, Channel__Base___GetManyByNameInSubscriptions___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let mut common_registry: Vec<Common1> = vec![];

                        '_a: for common_1 in data__.common_registry {
                            let channel_cover_image_path = match common_1.channel.channel_cover_image_path {
                                Some(channel_cover_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel_cover_image_path_.0)),
                                None => C_Option::none()
                            };

                            let channel_background_image_path = match common_1.channel.channel_background_image_path {
                                Some(channel_background_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel_background_image_path_.0)),
                                None => C_Option::none()
                            };

                            let common_1_ = Common1 {
                                channel: Channel1 {
                                    channel_id: common_1.channel.channel_id.0,
                                    channel_name: Allocator::<C_String>::allocate(common_1.channel.channel_name.0),
                                    channel_linked_name: Allocator::<C_String>::allocate(common_1.channel.channel_linked_name.0),
                                    channel_access_modifier: common_1.channel.channel_access_modifier.0,
                                    channel_visability_modifier: common_1.channel.channel_visability_modifier.0,
                                    channel_cover_image_path,
                                    channel_background_image_path,
                                },
                                is_application_user_subscribed: common_1.is_application_user_subscribed,
                            };

                            common_registry.push(common_1_);
                        }

                        let outcoming = Channel__Base___GetManyByNameInSubscriptions___Outcoming {
                            common_registry: Allocator::<C_Vector<_>>::allocate(common_registry),
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    Channel__Base___GetManyByNameInSubscriptions___Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        Channel__Base___GetManyByNameInSubscriptions___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Channel__Base___GetManyByNameInSubscriptions___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
                        Channel__Base___GetManyByNameInSubscriptions___Precedent {
                            application_user_access_token__in_application_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_by_name_in_subscriptions____deserialize____deallocate(
    c_result: *mut Channel__Base___GetManyByNameInSubscriptions___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(c_result)
    };

    if result_.is_data {
        if result_.data.is_target {
            if result_.data.target.is_filled {
                let common_registry = unsafe {
                    from_raw_parts(result_.data.target.filled.common_registry.pointer, result_.data.target.filled.common_registry.length )
                };

                for common in common_registry {
                    Allocator::<C_String>::deallocate(common.channel.channel_name);

                    Allocator::<C_String>::deallocate(common.channel.channel_linked_name);

                    if common.channel. channel_background_image_path.is_data {
                        Allocator::<C_String>::deallocate(common.channel. channel_background_image_path.data);
                    }

                    if common.channel. channel_cover_image_path.is_data {
                        Allocator::<C_String>::deallocate(common.channel. channel_cover_image_path.data);
                    }
                }

                Allocator::<C_Vector<_>>::deallocate(result_.data.target.filled.common_registry);
            }
        }
    }

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Channel__Base___GetManyBySubscription___Incoming {
    pub application_user_access_token_encrypted: C_String,
    pub requery_channel_id: C_Option<c_long>,
    pub limit: c_short,
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_by_subscription____serialize(
    incoming: *mut Channel__Base___GetManyBySubscription___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: Channel__Base___GetManyBySubscription___Incoming| -> Result<Channel__Base___GetManyBySubscription___Incoming_, Box<dyn StdError + 'static>> {
        let requery_channel_id = if incoming.requery_channel_id.is_data {
            Some(
                Channel_Id(
                    incoming.requery_channel_id.data
                )
            )
        } else {
            None
        };

        let incoming_ = Channel__Base___GetManyBySubscription___Incoming_ {
            application_user_access_token_encrypted: incoming.application_user_access_token_encrypted.to_string()?,
            requery_channel_id,
            limit: incoming.limit,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_by_subscription____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type Channel__Base___GetManyBySubscription___C_Result = C_Result<C_UnifiedReport<Channel__Base___GetManyBySubscription___Outcoming, Channel__Base___GetManyBySubscription___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Channel__Base___GetManyBySubscription___Outcoming {
    pub common_registry: C_Vector<Common1>,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Channel__Base___GetManyBySubscription___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_by_subscription____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut Channel__Base___GetManyBySubscription___C_Result {
    let converter = move |unified_report: UnifiedReport<Channel__Base___GetManyBySubscription___Outcoming_, Channel__Base___GetManyBySubscription___Precedent_>| -> Result<C_UnifiedReport<Channel__Base___GetManyBySubscription___Outcoming, Channel__Base___GetManyBySubscription___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let mut common_registry: Vec<Common1> = vec![];

                        '_a: for common_1 in data__.common_registry {
                            let channel_cover_image_path = match common_1.channel.channel_cover_image_path {
                                Some(channel_cover_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel_cover_image_path_.0)),
                                None => C_Option::none()
                            };

                            let channel_background_image_path = match common_1.channel.channel_background_image_path {
                                Some(channel_background_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel_background_image_path_.0)),
                                None => C_Option::none()
                            };

                            let common_1_ = Common1 {
                                channel: Channel1 {
                                    channel_id: common_1.channel.channel_id.0,
                                    channel_name: Allocator::<C_String>::allocate(common_1.channel.channel_name.0),
                                    channel_linked_name: Allocator::<C_String>::allocate(common_1.channel.channel_linked_name.0),
                                    channel_access_modifier: common_1.channel.channel_access_modifier.0,
                                    channel_visability_modifier: common_1.channel.channel_visability_modifier.0,
                                    channel_cover_image_path,
                                    channel_background_image_path,
                                },
                                is_application_user_subscribed: common_1.is_application_user_subscribed,
                            };

                            common_registry.push(common_1_);
                        }

                        let outcoming = Channel__Base___GetManyBySubscription___Outcoming {
                            common_registry: Allocator::<C_Vector<_>>::allocate(common_registry),
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    Channel__Base___GetManyBySubscription___Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        Channel__Base___GetManyBySubscription___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Channel__Base___GetManyBySubscription___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
                        Channel__Base___GetManyBySubscription___Precedent {
                            application_user_access_token__in_application_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_by_subscription____deserialize____deallocate(
    c_result: *mut Channel__Base___GetManyBySubscription___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(c_result)
    };

    if result_.is_data {
        if result_.data.is_target {
            if result_.data.target.is_filled {
                let common_registry = unsafe {
                    from_raw_parts(result_.data.target.filled.common_registry.pointer, result_.data.target.filled.common_registry.length )
                };

                for common in common_registry {
                    Allocator::<C_String>::deallocate(common.channel.channel_name);

                    Allocator::<C_String>::deallocate(common.channel.channel_linked_name);

                    if common.channel. channel_background_image_path.is_data {
                        Allocator::<C_String>::deallocate(common.channel. channel_background_image_path.data);
                    }

                    if common.channel. channel_cover_image_path.is_data {
                        Allocator::<C_String>::deallocate(common.channel. channel_cover_image_path.data);
                    }
                }

                Allocator::<C_Vector<_>>::deallocate(result_.data.target.filled.common_registry);
            }
        }
    }

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Channel__Base___GetManyPublicByName___Incoming {
    pub application_user_access_token_encrypted: C_String,
    pub channel_name: C_String,
    pub requery_channel_name: C_Option<C_String>,
    pub limit: c_short,
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_public_by_name____serialize(
    incoming: *mut Channel__Base___GetManyPublicByName___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: Channel__Base___GetManyPublicByName___Incoming| -> Result<Channel__Base___GetManyPublicByName___Incoming_, Box<dyn StdError + 'static>> {
        let requery_channel_name = if incoming.requery_channel_name.is_data {
            Some(
                Channel_Name(
                    incoming.requery_channel_name.data.to_string()?
                )
            )
        } else {
            None
        };

        let incoming_ = Channel__Base___GetManyPublicByName___Incoming_ {
            application_user_access_token_encrypted: incoming.application_user_access_token_encrypted.to_string()?,
            channel_name: Channel_Name(incoming.channel_name.to_string()?),
            requery_channel_name,
            limit: incoming.limit,
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_public_by_name____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type Channel__Base___GetManyPublicByName___C_Result = C_Result<C_UnifiedReport<Channel__Base___GetManyPublicByName___Outcoming, Channel__Base___GetManyPublicByName___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Channel__Base___GetManyPublicByName___Outcoming {
    pub common_registry: C_Vector<Common1>,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Channel__Base___GetManyPublicByName___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_public_by_name____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut Channel__Base___GetManyPublicByName___C_Result {
    let converter = move |unified_report: UnifiedReport<Channel__Base___GetManyPublicByName___Outcoming_, Channel__Base___GetManyPublicByName___Precedent_>| -> Result<C_UnifiedReport<Channel__Base___GetManyPublicByName___Outcoming, Channel__Base___GetManyPublicByName___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let mut common_registry: Vec<Common1> = vec![];

                        '_a: for common_1 in data__.common_registry {
                            let channel_cover_image_path = match common_1.channel.channel_cover_image_path {
                                Some(channel_cover_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel_cover_image_path_.0)),
                                None => C_Option::none()
                            };

                            let channel_background_image_path = match common_1.channel.channel_background_image_path {
                                Some(channel_background_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel_background_image_path_.0)),
                                None => C_Option::none()
                            };

                            let common_1_ = Common1 {
                                channel: Channel1 {
                                    channel_id: common_1.channel.channel_id.0,
                                    channel_name: Allocator::<C_String>::allocate(common_1.channel.channel_name.0),
                                    channel_linked_name: Allocator::<C_String>::allocate(common_1.channel.channel_linked_name.0),
                                    channel_access_modifier: common_1.channel.channel_access_modifier.0,
                                    channel_visability_modifier: common_1.channel.channel_visability_modifier.0,
                                    channel_cover_image_path,
                                    channel_background_image_path,
                                },
                                is_application_user_subscribed: common_1.is_application_user_subscribed,
                            };

                            common_registry.push(common_1_);
                        }

                        let outcoming = Channel__Base___GetManyPublicByName___Outcoming {
                            common_registry: Allocator::<C_Vector<_>>::allocate(common_registry),
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    Channel__Base___GetManyPublicByName___Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        Channel__Base___GetManyPublicByName___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Channel__Base___GetManyPublicByName___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
                        Channel__Base___GetManyPublicByName___Precedent {
                            application_user_access_token__in_application_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_public_by_name____deserialize____deallocate(
    c_result: *mut Channel__Base___GetManyPublicByName___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(c_result)
    };

    if result_.is_data {
        if result_.data.is_target {
            if result_.data.target.is_filled {
                let common_registry = unsafe {
                    from_raw_parts(result_.data.target.filled.common_registry.pointer, result_.data.target.filled.common_registry.length )
                };

                for common_1 in common_registry {
                    Allocator::<C_String>::deallocate(common_1.channel.channel_name);

                    Allocator::<C_String>::deallocate(common_1.channel.channel_linked_name);

                    if common_1.channel. channel_background_image_path.is_data {
                        Allocator::<C_String>::deallocate(common_1.channel. channel_background_image_path.data);
                    }

                    if common_1.channel. channel_cover_image_path.is_data {
                        Allocator::<C_String>::deallocate(common_1.channel. channel_cover_image_path.data);
                    }
                }

                Allocator::<C_Vector<_>>::deallocate(result_.data.target.filled.common_registry);
            }
        }
    }

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Channel__Base___GetOneById___Incoming {
    pub application_user_access_token_encrypted: C_String,
    pub channel_id: c_long,
}

#[no_mangle]
pub extern "C" fn channel___base____get_one_by_id____serialize(
    incoming: *mut Channel__Base___GetOneById___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: Channel__Base___GetOneById___Incoming| -> Result<Channel__Base___GetOneById___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = Channel__Base___GetOneById___Incoming_ {
            application_user_access_token_encrypted: incoming.application_user_access_token_encrypted.to_string()?,
            channel_id: Channel_Id(incoming.channel_id),
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn channel___base____get_one_by_id____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type Channel__Base___GetOneById___C_Result = C_Result<C_UnifiedReport<Channel__Base___GetOneById___Outcoming, Channel__Base___GetOneById___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Channel__Base___GetOneById___Outcoming {
    pub channel: Channel2,
    pub channel_inner_link_registry: C_Vector<ChannelInnerLink1>,
    pub channel_outer_link_registry: C_Vector<ChannelOuterLink1>,
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Channel__Base___GetOneById___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
    pub channel__not_found: bool,
    pub channel__is_close: bool,
}

#[no_mangle]
pub extern "C" fn channel___base____get_one_by_id____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut Channel__Base___GetOneById___C_Result {
    let converter = move |unified_report: UnifiedReport<Channel__Base___GetOneById___Outcoming_, Channel__Base___GetOneById___Precedent_>| -> Result<C_UnifiedReport<Channel__Base___GetOneById___Outcoming, Channel__Base___GetOneById___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let channel_description = match data__.channel.channel_description {
                            Some(channel_description_) => C_Option::data(Allocator::<C_String>::allocate(channel_description_.0)),
                            None => C_Option::none()
                        };

                        let channel_cover_image_path = match data__.channel.channel_cover_image_path {
                            Some(channel_cover_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel_cover_image_path_.0)),
                            None => C_Option::none()
                        };

                        let channel_background_image_path = match data__.channel.channel_background_image_path {
                            Some(channel_background_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel_background_image_path_.0)),
                            None => C_Option::none()
                        };

                        let channel_2 = Channel2 {
                            channel_owner: data__.channel.channel_owner,
                            channel_name: Allocator::<C_String>::allocate(data__.channel.channel_name.0),
                            channel_linked_name: Allocator::<C_String>::allocate(data__.channel.channel_linked_name.0),
                            channel_description,
                            channel_access_modifier: data__.channel.channel_access_modifier.0,
                            channel_visability_modifier: data__.channel.channel_visability_modifier.0,
                            channel_orientation: Allocator::<C_Vector<_>>::allocate(data__.channel.channel_orientation.0),
                            channel_cover_image_path,
                            channel_background_image_path,
                            channel_subscribers_quantity: data__.channel.channel_subscribers_quantity.0,
                            channel_marks_quantity: data__.channel.channel_marks_quantity.0,
                            channel_viewing_quantity: data__.channel. channel_viewing_quantity.0,
                        };

                        let mut channel_inner_link_registry: Vec<ChannelInnerLink1> = vec![];

                        '_a: for channel_inner_link_1 in data__.channel_inner_link_registry {
                            let channel_inner_link_1_ = ChannelInnerLink1 {
                                channel_inner_link_to: channel_inner_link_1.channel_inner_link_to.0
                            };

                            channel_inner_link_registry.push(channel_inner_link_1_);
                        }

                        let mut channel_outer_link_registry: Vec<ChannelOuterLink1> = vec![];

                        '_a: for channel_outer_link_1 in data__.channel_outer_link_registry {
                            let channel_outer_link_1_ = ChannelOuterLink1 {
                                channel_outer_link_alias: Allocator::<C_String>::allocate(channel_outer_link_1.channel_outer_link_alias.0),
                                channel_outer_link_address: Allocator::<C_String>::allocate(channel_outer_link_1.channel_outer_link_address.0)
                            };

                            channel_outer_link_registry.push(channel_outer_link_1_);
                        }

                        let outcoming = Channel__Base___GetOneById___Outcoming {
                            channel: channel_2,
                            channel_inner_link_registry: Allocator::<C_Vector<_>>::allocate(channel_inner_link_registry),
                            channel_outer_link_registry: Allocator::<C_Vector<_>>::allocate(channel_outer_link_registry),
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    Channel__Base___GetOneById___Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        Channel__Base___GetOneById___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Channel__Base___GetOneById___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
                        Channel__Base___GetOneById___Precedent {
                            application_user_access_token__in_application_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
                    Channel__Base___GetOneById___Precedent_::Channel_NotFound => {
                        Channel__Base___GetOneById___Precedent {
                            channel__not_found: true,
                            ..Default::default()
                        }
                    }
                    Channel__Base___GetOneById___Precedent_::Channel_IsClose => {
                        Channel__Base___GetOneById___Precedent {
                            channel__is_close: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn channel___base____get_one_by_id____deserialize____deallocate(
    c_result: *mut Channel__Base___GetOneById___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(c_result)
    };

    if result_.is_data {
        if result_.data.is_target {
            if result_.data.target.is_filled {
                Allocator::<C_String>::deallocate(result_.data.target.filled.channel.channel_name);

                Allocator::<C_String>::deallocate(result_.data.target.filled.channel.channel_linked_name);

                if result_.data.target.filled.channel.channel_description.is_data {
                    Allocator::<C_String>::deallocate(result_.data.target.filled.channel.channel_description.data);
                }

                if result_.data.target.filled.channel.channel_background_image_path.is_data {
                    Allocator::<C_String>::deallocate(result_.data.target.filled.channel.channel_background_image_path.data);
                }

                if result_.data.target.filled.channel.channel_cover_image_path.is_data {
                    Allocator::<C_String>::deallocate(result_.data.target.filled.channel.channel_cover_image_path.data);
                }

                Allocator::<C_Vector<_>>::deallocate(result_.data.target.filled.channel.channel_orientation);

                Allocator::<C_Vector<_>>::deallocate(result_.data.target.filled.channel_inner_link_registry);

                let channel_outer_link_registry = unsafe {
                    from_raw_parts(result_.data.target.filled.channel_outer_link_registry.pointer, result_.data.target.filled.channel_outer_link_registry.length)
                };

                for channel_outer_link_1 in channel_outer_link_registry {
                    Allocator::<C_String>::deallocate(channel_outer_link_1.channel_outer_link_alias);

                    Allocator::<C_String>::deallocate(channel_outer_link_1.channel_outer_link_address);
                }

                Allocator::<C_Vector<_>>::deallocate(result_.data.target.filled.channel_outer_link_registry);
            }
        }
    }

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelSubscription__Base___Create___Incoming {
    pub application_user_access_token_encrypted: C_String,
    pub channel_id: c_long,
}

#[no_mangle]
pub extern "C" fn channel_subscription___base____create____serialize(
    incoming: *mut ChannelSubscription__Base___Create___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ChannelSubscription__Base___Create___Incoming| -> Result<ChannelSubscription__Base___Create___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ChannelSubscription__Base___Create___Incoming_ {
            application_user_access_token_encrypted: incoming.application_user_access_token_encrypted.to_string()?,
            channel_id: Channel_Id(incoming.channel_id),
        };

        return Ok(incoming_);
    };

    return Transformer::<ServerRequestData>::transform(incoming, converter);
}

#[no_mangle]
pub extern "C" fn channel_subscription___base____create____serialize____deallocate(
    c_result: *mut C_Result<C_Vector<c_uchar>>
) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);

    return ();
}

type ChannelSubscription__Base___Create___C_Result = C_Result<C_UnifiedReport<C_Void, ChannelSubscription__Base___Create___Precedent>>;

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ChannelSubscription__Base___Create___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
    pub channel__not_found: bool,
    pub channel__is_close: bool,
    pub application_user__is_channel_owner: bool,
}

#[no_mangle]
pub extern "C" fn channel_subscription___base____create____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ChannelSubscription__Base___Create___C_Result {
    let converter = move |unified_report: UnifiedReport<Void, ChannelSubscription__Base___Create___Precedent_>| -> Result<C_UnifiedReport<C_Void, ChannelSubscription__Base___Create___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: _ } => {
                        C_Data::filled(C_Void::new())
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ChannelSubscription__Base___Create___Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        ChannelSubscription__Base___Create___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    ChannelSubscription__Base___Create___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
                        ChannelSubscription__Base___Create___Precedent {
                            application_user_access_token__in_application_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
                    ChannelSubscription__Base___Create___Precedent_::Channel_NotFound => {
                        ChannelSubscription__Base___Create___Precedent {
                            channel__not_found: true,
                            ..Default::default()
                        }
                    }
                    ChannelSubscription__Base___Create___Precedent_::Channel_IsClose => {
                        ChannelSubscription__Base___Create___Precedent {
                            channel__is_close: true,
                            ..Default::default()
                        }
                    }
                    ChannelSubscription__Base___Create___Precedent_::ApplicationUser_IsChannelOwner => {
                        ChannelSubscription__Base___Create___Precedent {
                            application_user__is_channel_owner: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Transformer::<ServerResponseData>::transform(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn channel_subscription___base____create____deserialize____deallocate(
    c_result: *mut ChannelSubscription__Base___Create___C_Result
) -> () {
    if c_result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(c_result)
    };

    return ();
}

// All tests should be executed using the `Valgrind` utility as `cargo valgrind test ...' command.
#[cfg(test)]
mod test {
    use super::*;

    mod deallocation {
        use super::*;

        const STRING_LITERAL: &'static str = "qwerty";

        mod server_response_data_deserialization {
            use auditor::Auditor;
            use entity::application_user_registration_token::ApplicationUserRegistrationToken_CanBeResentFrom;
            use entity::application_user_registration_token::ApplicationUserRegistrationToken_WrongEnterTriesQuantity;
            use entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_CanBeResentFrom;
            use entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_WrongEnterTriesQuantity;
            use entity::channel_outer_link::ChannelOuterLink_Address;
            use entity::channel_outer_link::ChannelOuterLink_Alias;
            use entity::channel::Channel_AccessModifier;
            use entity::channel::Channel_BackgroundImagePath;
            use entity::channel::Channel_CoverImagePath;
            use entity::channel::Channel_Description;
            use entity::channel::Channel_Id;
            use entity::channel::Channel_LinkedName;
            use entity::channel::Channel_MarksQuantity;
            use entity::channel::Channel_Name;
            use entity::channel::Channel_Orientation;
            use entity::channel::Channel_SubscribersQuantity;
            use entity::channel::Channel_ViewingQuantity;
            use entity::channel::Channel_VisabilityModifier;
            use error::Error;
            use formatter::Formatter;
            use super::*;

            fn run_by_template<'a, T, E, A, D>(
                data: &'a T,
                allocator: A,
                deallocator: D,
            ) -> Result<(), Box<dyn StdError + 'static>>
            where
                T: Serialize,
                A: FnOnce(*mut C_Vector<c_uchar>) -> *mut C_Result<E>,
                E: Copy,
                D: FnOnce(*mut C_Result<E>) -> (),
            {
                let registry = match Serializer_::serialize(data) {
                    Ok(registry_) => registry_,
                    Err(error_auditor) => {
                        return Err(
                            Formatter::<Auditor<Error>>::format(&error_auditor).into()
                        );
                    }
                };

                let c_vector = Allocator::<C_Vector<_>>::allocate(registry);

                let c_vector_ = ((&c_vector) as *const _) as *mut _;

                let c_result = allocator(c_vector_);

                let c_result_ = unsafe {
                    *c_result
                };

                if !c_result_.is_data {
                    return Err("Function inner error.".into());
                }

                deallocator(c_result);

                if c_vector_.is_null() {
                    return Err("The pointer must continue to point to the data.".into());
                }

                Allocator::<C_Vector<_>>::deallocate(c_vector);

                return Ok(());
            }

            // Needed to test all `unified_report::UnifiedReport` variants.
            mod unified_report {
                use super::*;
                use action_processor_incoming_outcoming::Channel1 as Channel1_;
                use action_processor_incoming_outcoming::ChannelInnerLink1 as ChannelInnerLink1_;
                use action_processor_incoming_outcoming::ChannelOuterLink1 as ChannelOuterLink1_;
                use action_processor_incoming_outcoming::Common1 as Common1_;
                use action_processor_incoming_outcoming::Channel2 as Channel2_;

                #[test]
                fn target_empty____application_user___authorization____authorize_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming_, ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result {
                        return application_user___authorization____authorize_by_first_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result| -> () {
                        application_user___authorization____authorize_by_first_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____authorize_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming_ {
                        application_user_id: 0,
                        verification_message_sent: false,
                        application_user_authorization_token_can_be_resent_from: 0,
                        application_user_authorization_token_wrong_enter_tries_quantity: 0,
                        application_user_authorization_token_wrong_enter_tries_quantity_limit: 0,
                    };

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming_, ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent_>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result {
                        return application_user___authorization____authorize_by_first_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result| -> () {
                        application_user___authorization____authorize_by_first_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____authorize_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let precedent = ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent_::ApplicationUser_WrongEmailOrNicknameOrPassword;

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming_, ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result {
                        return application_user___authorization____authorize_by_first_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result| -> () {
                        application_user___authorization____authorize_by_first_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_empty____application_user___authorization____authorize_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming_, ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result {
                        return application_user___authorization____authorize_by_last_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result| -> () {
                        application_user___authorization____authorize_by_last_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____authorize_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming_ {
                        application_user_access_token_encrypted: STRING_LITERAL.to_string(),
                        application_user_access_refresh_token_encrypted: STRING_LITERAL.to_string(),
                    };

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming_, ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result {
                        return application_user___authorization____authorize_by_last_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result| -> () {
                        application_user___authorization____authorize_by_last_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                fn _precedent____application_user___authorization____authorize_by_last_step(precedent: ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming_, ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result {
                        return application_user___authorization____authorize_by_last_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result| -> () {
                        application_user___authorization____authorize_by_last_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____authorize_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_> = vec![];

                    precedent_registry.push(ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_::ApplicationUserAuthorizationToken_NotFound);

                    precedent_registry.push(ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_::ApplicationUserAuthorizationToken_AlreadyExpired);

                    precedent_registry.push(
                        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_::ApplicationUserAuthorizationToken_WrongValue {
                            application_user_authorization_token_wrong_enter_tries_quantity: 0,
                        }
                    );

                    precedent_registry.push(ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_::ApplicationUser_NotFound);

                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____authorize_by_last_step(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____application_user___authorization____check_email_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___CheckEmailForExisting___Outcoming_, Void>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___CheckEmailForExisting___C_Result {
                        return application_user___authorization____check_email_for_existing____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___CheckEmailForExisting___C_Result| -> () {
                        application_user___authorization____check_email_for_existing____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____check_email_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___CheckEmailForExisting___Outcoming_ {
                        result: false,
                    };

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___CheckEmailForExisting___Outcoming_, Void>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___CheckEmailForExisting___C_Result {
                        return application_user___authorization____check_email_for_existing____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___CheckEmailForExisting___C_Result| -> () {
                        application_user___authorization____check_email_for_existing____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____check_email_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }

                #[test]
                fn target_empty____application_user___authorization____check_nickname_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming_, Void>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___CheckNicknameForExisting___C_Result {
                        return application_user___authorization____check_nickname_for_existing____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___CheckNicknameForExisting___C_Result| -> () {
                        application_user___authorization____check_nickname_for_existing____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____check_nickname_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming_ {
                        result: false,
                    };

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming_, Void>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___CheckNicknameForExisting___C_Result {
                        return application_user___authorization____check_nickname_for_existing____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___CheckNicknameForExisting___C_Result| -> () {
                        application_user___authorization____check_nickname_for_existing____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____check_nickname_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }

                #[test]
                fn target_empty____application_user___authorization____deauthorize_from_all_devices() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result {
                        return application_user___authorization____deauthorize_from_all_devices____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result| -> () {
                        application_user___authorization____deauthorize_from_all_devices____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____deauthorize_from_all_devices() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }

                fn _precedent____application_user___authorization____deauthorize_from_all_devices(precedent: ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result {
                        return application_user___authorization____deauthorize_from_all_devices____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result| -> () {
                        application_user___authorization____deauthorize_from_all_devices____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____deauthorize_from_all_devices() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_> = vec![];

                    precedent_registry.push(ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_::ApplicationUserAccessToken_AlreadyExpired);

                    precedent_registry.push(ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList);

                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____deauthorize_from_all_devices(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____application_user___authorization____deauthorize_from_one_device() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result {
                        return application_user___authorization____deauthorize_from_one_device____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result| -> () {
                        application_user___authorization____deauthorize_from_one_device____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____deauthorize_from_one_device() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }

                fn _precedent____application_user___authorization____deauthorize_from_one_device(precedent: ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result {
                        return application_user___authorization____deauthorize_from_one_device____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result| -> () {
                        application_user___authorization____deauthorize_from_one_device____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____deauthorize_from_one_device() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_> = vec![];

                    precedent_registry.push(ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_::ApplicationUserAccessToken_AlreadyExpired);

                    precedent_registry.push(ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList);

                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____deauthorize_from_one_device(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____application_user___authorization____refresh_access_token() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___RefreshAccessToken___Outcoming_, ApplicationUser__Authorization___RefreshAccessToken___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result {
                        return application_user___authorization____refresh_access_token____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result| -> () {
                        application_user___authorization____refresh_access_token____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____refresh_access_token() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___RefreshAccessToken___Outcoming_ {
                        application_user_access_token_encrypted: STRING_LITERAL.to_string(),
                        application_user_access_refresh_token_encrypted: STRING_LITERAL.to_string(),
                    };

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___RefreshAccessToken___Outcoming_, ApplicationUser__Authorization___RefreshAccessToken___Precedent_>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result {
                        return application_user___authorization____refresh_access_token____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result| -> () {
                        application_user___authorization____refresh_access_token____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                fn _precedent____application_user___authorization____refresh_access_token(precedent: ApplicationUser__Authorization___RefreshAccessToken___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___RefreshAccessToken___Outcoming_, ApplicationUser__Authorization___RefreshAccessToken___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result {
                        return application_user___authorization____refresh_access_token____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result| -> () {
                        application_user___authorization____refresh_access_token____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____refresh_access_token() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___RefreshAccessToken___Precedent_> = vec![];

                    precedent_registry.push(ApplicationUser__Authorization___RefreshAccessToken___Precedent_::ApplicationUserAccessRefreshToken_NotFound);

                    precedent_registry.push(ApplicationUser__Authorization___RefreshAccessToken___Precedent_::ApplicationUserAccessRefreshToken_AlreadyExpired);

                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____refresh_access_token(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____application_user___authorization____register_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___RegisterByFirstStep___Outcoming_, ApplicationUser__Authorization___RegisterByFirstStep___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result {
                        return application_user___authorization____register_by_first_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result| -> () {
                        application_user___authorization____register_by_first_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____register_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___RegisterByFirstStep___Outcoming_ {
                        verification_message_sent: false,
                        application_user_registration_token_can_be_resent_from: ApplicationUserRegistrationToken_CanBeResentFrom(0),
                        application_user_registration_token_wrong_enter_tries_quantity: ApplicationUserRegistrationToken_WrongEnterTriesQuantity(0),
                        application_user_registration_token_wrong_enter_tries_quantity_limit: 0,
                    };

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___RegisterByFirstStep___Outcoming_, ApplicationUser__Authorization___RegisterByFirstStep___Precedent_>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result {
                        return application_user___authorization____register_by_first_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result| -> () {
                        application_user___authorization____register_by_first_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____register_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let precedent = ApplicationUser__Authorization___RegisterByFirstStep___Precedent_::ApplicationUser_EmailAlreadyExist;

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___RegisterByFirstStep___Outcoming_, ApplicationUser__Authorization___RegisterByFirstStep___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result {
                        return application_user___authorization____register_by_first_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result| -> () {
                        application_user___authorization____register_by_first_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_empty____application_user___authorization____register_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___RegisterBySecondStep___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterBySecondStep___C_Result {
                        return application_user___authorization____register_by_second_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterBySecondStep___C_Result| -> () {
                        application_user___authorization____register_by_second_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____register_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }

                fn _precedent____application_user___authorization____register_by_second_step(precedent: ApplicationUser__Authorization___RegisterBySecondStep___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___RegisterBySecondStep___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterBySecondStep___C_Result {
                        return application_user___authorization____register_by_second_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterBySecondStep___C_Result| -> () {
                        application_user___authorization____register_by_second_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____register_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___RegisterBySecondStep___Precedent_> = vec![];

                    precedent_registry.push(ApplicationUser__Authorization___RegisterBySecondStep___Precedent_::ApplicationUserRegistrationToken_NotFound);

                    precedent_registry.push(ApplicationUser__Authorization___RegisterBySecondStep___Precedent_::ApplicationUserRegistrationToken_AlreadyExpired);

                    precedent_registry.push(ApplicationUser__Authorization___RegisterBySecondStep___Precedent_::ApplicationUserRegistrationToken_AlreadyApproved);

                    precedent_registry.push(
                        ApplicationUser__Authorization___RegisterBySecondStep___Precedent_::ApplicationUserRegistrationToken_WrongValue {
                            application_user_registration_token_wrong_enter_tries_quantity: ApplicationUserRegistrationToken_WrongEnterTriesQuantity(0),
                        }
                    );

                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____register_by_second_step(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____application_user___authorization____register_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___RegisterByLastStep___Outcoming_, ApplicationUser__Authorization___RegisterByLastStep___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result {
                        return application_user___authorization____register_by_last_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result| -> () {
                        application_user___authorization____register_by_last_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____register_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___RegisterByLastStep___Outcoming_ {
                        application_user_access_token_encrypted: STRING_LITERAL.to_string(),
                        application_user_access_refresh_token_encrypted: STRING_LITERAL.to_string(),
                    };

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___RegisterByLastStep___Outcoming_, ApplicationUser__Authorization___RegisterByLastStep___Precedent_>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result {
                        return application_user___authorization____register_by_last_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result| -> () {
                        application_user___authorization____register_by_last_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                fn _precedent____application_user___authorization____register_by_last_step(precedent: ApplicationUser__Authorization___RegisterByLastStep___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___RegisterByLastStep___Outcoming_, ApplicationUser__Authorization___RegisterByLastStep___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result {
                        return application_user___authorization____register_by_last_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result| -> () {
                        application_user___authorization____register_by_last_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____register_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___RegisterByLastStep___Precedent_> = vec![];

                    precedent_registry.push(ApplicationUser__Authorization___RegisterByLastStep___Precedent_::ApplicationUser_NicknameAlreadyExist);

                    precedent_registry.push(ApplicationUser__Authorization___RegisterByLastStep___Precedent_::ApplicationUser_EmailAlreadyExist);

                    precedent_registry.push(ApplicationUser__Authorization___RegisterByLastStep___Precedent_::ApplicationUserRegistrationToken_NotFound);

                    precedent_registry.push(ApplicationUser__Authorization___RegisterByLastStep___Precedent_::ApplicationUserRegistrationToken_AlreadyExpired);

                    precedent_registry.push(ApplicationUser__Authorization___RegisterByLastStep___Precedent_::ApplicationUserRegistrationToken_IsNotApproved);

                    precedent_registry.push(ApplicationUser__Authorization___RegisterByLastStep___Precedent_::ApplicationUserRegistrationToken_WrongValue);

                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____register_by_last_step(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____application_user___authorization____reset_password_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming_, ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result {
                        return application_user___authorization____reset_password_by_first_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_first_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____reset_password_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming_ {
                        application_user_id: 0,
                        verification_message_sent: false,
                        application_user_reset_password_token_can_be_resent_from: ApplicationUserResetPasswordToken_CanBeResentFrom(0),
                        application_user_reset_password_token_wrong_enter_tries_quantity: ApplicationUserResetPasswordToken_WrongEnterTriesQuantity(0),
                        application_user_reset_password_token_wrong_enter_tries_quantity_limit: 0,
                    };

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming_, ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result {
                        return application_user___authorization____reset_password_by_first_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_first_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____reset_password_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let precedent = ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_::ApplicationUser_NotFound;

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming_, ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result {
                        return application_user___authorization____reset_password_by_first_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_first_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_empty____application_user___authorization____reset_password_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result {
                        return application_user___authorization____reset_password_by_second_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_second_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____reset_password_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }

                fn _precedent____application_user___authorization____reset_password_by_second_step(precedent: ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result {
                        return application_user___authorization____reset_password_by_second_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_second_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____reset_password_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_> = vec![];

                    precedent_registry.push(ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_::ApplicationUserResetPasswordToken_NotFound);

                    precedent_registry.push(ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_::ApplicationUserResetPasswordToken_AlreadyExpired);

                    precedent_registry.push(ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_::ApplicationUserResetPasswordToken_AlreadyApproved);

                    precedent_registry.push(
                        ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_::ApplicationUserResetPasswordToken_WrongValue {
                            application_user_reset_password_token_wrong_enter_tries_quantity: ApplicationUserResetPasswordToken_WrongEnterTriesQuantity(0),
                        }
                    );

                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____reset_password_by_second_step(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____application_user___authorization____reset_password_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result {
                        return application_user___authorization____reset_password_by_last_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_last_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____reset_password_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }

                fn _precedent____application_user___authorization____reset_password_by_last_step(precedent: ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result {
                        return application_user___authorization____reset_password_by_last_step____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_last_step____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____reset_password_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_> = vec![];

                    precedent_registry.push(ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_::ApplicationUser_NotFound);

                    precedent_registry.push(ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_::ApplicationUserResetPasswordToken_NotFound);

                    precedent_registry.push(ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_::ApplicationUserResetPasswordToken_AlreadyExpired);

                    precedent_registry.push(ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_::ApplicationUserResetPasswordToken_IsNotApproved);

                    precedent_registry.push(ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_::ApplicationUserResetPasswordToken_WrongValue);

                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____reset_password_by_last_step(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____application_user___authorization____send_email_for_register() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___SendEmailForRegister___Outcoming_, ApplicationUser__Authorization___SendEmailForRegister___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result {
                        return application_user___authorization____send_email_for_register____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result| -> () {
                        application_user___authorization____send_email_for_register____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____send_email_for_register() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___SendEmailForRegister___Outcoming_ {
                        application_user_registration_token_can_be_resent_from: ApplicationUserRegistrationToken_CanBeResentFrom(0),
                    };

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___SendEmailForRegister___Outcoming_, ApplicationUser__Authorization___SendEmailForRegister___Precedent_>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result {
                        return application_user___authorization____send_email_for_register____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result| -> () {
                        application_user___authorization____send_email_for_register____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                fn _precedent____application_user___authorization____send_email_for_register(precedent: ApplicationUser__Authorization___SendEmailForRegister___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___SendEmailForRegister___Outcoming_, ApplicationUser__Authorization___SendEmailForRegister___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result {
                        return application_user___authorization____send_email_for_register____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result| -> () {
                        application_user___authorization____send_email_for_register____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____send_email_for_register() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___SendEmailForRegister___Precedent_> = vec![];

                    precedent_registry.push(ApplicationUser__Authorization___SendEmailForRegister___Precedent_::ApplicationUserRegistrationToken_NotFound);

                    precedent_registry.push(ApplicationUser__Authorization___SendEmailForRegister___Precedent_::ApplicationUserRegistrationToken_AlreadyExpired);

                    precedent_registry.push(ApplicationUser__Authorization___SendEmailForRegister___Precedent_::ApplicationUserRegistrationToken_AlreadyApproved);

                    precedent_registry.push(ApplicationUser__Authorization___SendEmailForRegister___Precedent_::ApplicationUserRegistrationToken_TimeToResendHasNotCome);

                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____send_email_for_register(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____application_user___authorization____send_email_for_authorize() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming_, ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result {
                        return application_user___authorization____send_email_for_authorize____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result| -> () {
                        application_user___authorization____send_email_for_authorize____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____send_email_for_authorize() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming_ {
                        application_user_authorization_token_can_be_resent_from: 0,
                    };

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming_, ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result {
                        return application_user___authorization____send_email_for_authorize____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result| -> () {
                        application_user___authorization____send_email_for_authorize____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                fn _precedent____application_user___authorization____send_email_for_authorize(precedent: ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming_, ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result {
                        return application_user___authorization____send_email_for_authorize____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result| -> () {
                        application_user___authorization____send_email_for_authorize____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____send_email_for_authorize() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_> = vec![];

                    precedent_registry.push(ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_::ApplicationUser_NotFound);

                    precedent_registry.push(ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_::ApplicationUserAuthorizationToken_NotFound);

                    precedent_registry.push(ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_::ApplicationUserAuthorizationToken_AlreadyExpired);

                    precedent_registry.push(ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_::ApplicationUserAuthorizationToken_TimeToResendHasNotCome);

                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____send_email_for_authorize(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____application_user___authorization____send_email_for_reset_password() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming_, ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result {
                        return application_user___authorization____send_email_for_reset_password____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result| -> () {
                        application_user___authorization____send_email_for_reset_password____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____application_user___authorization____send_email_for_reset_password() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming_ {
                        application_user_reset_password_token_can_be_resent_from: ApplicationUserResetPasswordToken_CanBeResentFrom(0),
                    };

                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming_, ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result {
                        return application_user___authorization____send_email_for_reset_password____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result| -> () {
                        application_user___authorization____send_email_for_reset_password____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                fn _precedent____application_user___authorization____send_email_for_reset_password(precedent: ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming_, ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result {
                        return application_user___authorization____send_email_for_reset_password____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result| -> () {
                        application_user___authorization____send_email_for_reset_password____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____application_user___authorization____send_email_for_reset_password() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_> = vec![];

                    precedent_registry.push(ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_::ApplicationUser_NotFound);

                    precedent_registry.push(ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_::ApplicationUserResetPasswordToken_NotFound);

                    precedent_registry.push(ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_::ApplicationUserResetPasswordToken_AlreadyExpired);

                    precedent_registry.push(ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_::ApplicationUserResetPasswordToken_AlreadyApproved);

                    precedent_registry.push(ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_::ApplicationUserResetPasswordToken_TimeToResendHasNotCome);

                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____send_email_for_reset_password(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____channel___base____get_many_by_name_in_subscriptions() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Channel__Base___GetManyByNameInSubscriptions___Outcoming_, Channel__Base___GetManyByNameInSubscriptions___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyByNameInSubscriptions___C_Result {
                        return channel___base____get_many_by_name_in_subscriptions____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut Channel__Base___GetManyByNameInSubscriptions___C_Result| -> () {
                        channel___base____get_many_by_name_in_subscriptions____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____channel___base____get_many_by_name_in_subscriptions() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut common_registry: Vec<Common1_> = vec![];

                    '_a: for _ in 1..=5 {
                        let common_1 = Common1_ {
                            channel: Channel1_ {
                                channel_id: Channel_Id(0),
                                channel_name: Channel_Name(STRING_LITERAL.to_string()),
                                channel_linked_name: Channel_LinkedName(STRING_LITERAL.to_string()),
                                channel_access_modifier: Channel_AccessModifier(0),
                                channel_visability_modifier: Channel_VisabilityModifier(0),
                                channel_background_image_path: Some(Channel_BackgroundImagePath(STRING_LITERAL.to_string())),
                                channel_cover_image_path: Some(Channel_CoverImagePath(STRING_LITERAL.to_string())),
                            },
                            is_application_user_subscribed: false,
                        };

                        common_registry.push(common_1);
                    }

                    let outcoming = Channel__Base___GetManyByNameInSubscriptions___Outcoming_ {
                       common_registry
                    };

                    let unified_report = UnifiedReport::<Channel__Base___GetManyByNameInSubscriptions___Outcoming_, Channel__Base___GetManyByNameInSubscriptions___Precedent_>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyByNameInSubscriptions___C_Result {
                        return channel___base____get_many_by_name_in_subscriptions____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut Channel__Base___GetManyByNameInSubscriptions___C_Result| -> () {
                        channel___base____get_many_by_name_in_subscriptions____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                fn _precedent____channel___base____get_many_by_name_in_subscriptions(precedent: Channel__Base___GetManyByNameInSubscriptions___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Channel__Base___GetManyByNameInSubscriptions___Outcoming_, Channel__Base___GetManyByNameInSubscriptions___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyByNameInSubscriptions___C_Result {
                        return channel___base____get_many_by_name_in_subscriptions____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut Channel__Base___GetManyByNameInSubscriptions___C_Result| -> () {
                        channel___base____get_many_by_name_in_subscriptions____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____channel___base____get_many_by_name_in_subscriptions() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<Channel__Base___GetManyByNameInSubscriptions___Precedent_> = vec![];

                    precedent_registry.push(Channel__Base___GetManyByNameInSubscriptions___Precedent_::ApplicationUserAccessToken_AlreadyExpired);

                    precedent_registry.push(Channel__Base___GetManyByNameInSubscriptions___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList);

                    '_a: for precedent in precedent_registry {
                        _precedent____channel___base____get_many_by_name_in_subscriptions(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____channel___base____get_many_by_subscription() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Channel__Base___GetManyBySubscription___Outcoming_, Channel__Base___GetManyBySubscription___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyBySubscription___C_Result {
                        return channel___base____get_many_by_subscription____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut Channel__Base___GetManyBySubscription___C_Result| -> () {
                        channel___base____get_many_by_subscription____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____channel___base____get_many_by_subscription() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut common_registry: Vec<Common1_> = vec![];

                    '_a: for _ in 1..=2 {
                        let common_1 = Common1_ {
                            channel: Channel1_ {
                                channel_id: Channel_Id(0),
                                channel_name: Channel_Name(STRING_LITERAL.to_string()),
                                channel_linked_name: Channel_LinkedName(STRING_LITERAL.to_string()),
                                channel_access_modifier: Channel_AccessModifier(0),
                                channel_visability_modifier: Channel_VisabilityModifier(0),
                                channel_background_image_path: Some(Channel_BackgroundImagePath(STRING_LITERAL.to_string())),
                                channel_cover_image_path: Some(Channel_CoverImagePath(STRING_LITERAL.to_string())),
                            },
                            is_application_user_subscribed: false,
                        };

                        common_registry.push(common_1);
                    }

                    let outcoming = Channel__Base___GetManyBySubscription___Outcoming_ {
                       common_registry
                    };

                    let unified_report = UnifiedReport::<Channel__Base___GetManyBySubscription___Outcoming_, Channel__Base___GetManyBySubscription___Precedent_>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyBySubscription___C_Result {
                        return channel___base____get_many_by_subscription____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut Channel__Base___GetManyBySubscription___C_Result| -> () {
                        channel___base____get_many_by_subscription____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                fn _precedent____channel___base____get_many_by_subscription(precedent: Channel__Base___GetManyBySubscription___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Channel__Base___GetManyBySubscription___Outcoming_, Channel__Base___GetManyBySubscription___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyBySubscription___C_Result {
                        return channel___base____get_many_by_subscription____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut Channel__Base___GetManyBySubscription___C_Result| -> () {
                        channel___base____get_many_by_subscription____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____channel___base____get_many_by_subscription() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<Channel__Base___GetManyBySubscription___Precedent_> = vec![];

                    precedent_registry.push(Channel__Base___GetManyBySubscription___Precedent_::ApplicationUserAccessToken_AlreadyExpired);

                    precedent_registry.push(Channel__Base___GetManyBySubscription___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList);

                    '_a: for precedent in precedent_registry {
                        _precedent____channel___base____get_many_by_subscription(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____channel___base____get_many_public_by_name() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Channel__Base___GetManyPublicByName___Outcoming_, Channel__Base___GetManyPublicByName___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyPublicByName___C_Result {
                        return channel___base____get_many_public_by_name____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut Channel__Base___GetManyPublicByName___C_Result| -> () {
                        channel___base____get_many_public_by_name____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____channel___base____get_many_public_by_name() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut common_registry: Vec<Common1_> = vec![];

                    '_a: for _ in 1..=5 {
                        let common_1 = Common1_ {
                            channel: Channel1_ {
                                channel_id: Channel_Id(0),
                                channel_name: Channel_Name(STRING_LITERAL.to_string()),
                                channel_linked_name: Channel_LinkedName(STRING_LITERAL.to_string()),
                                channel_access_modifier: Channel_AccessModifier(0),
                                channel_visability_modifier: Channel_VisabilityModifier(0),
                                channel_background_image_path: Some(Channel_BackgroundImagePath(STRING_LITERAL.to_string())),
                                channel_cover_image_path: Some(Channel_CoverImagePath(STRING_LITERAL.to_string())),
                            },
                            is_application_user_subscribed: false,
                        };

                        common_registry.push(common_1);
                    }

                    let outcoming = Channel__Base___GetManyPublicByName___Outcoming_ {
                       common_registry
                    };

                    let unified_report = UnifiedReport::<Channel__Base___GetManyPublicByName___Outcoming_, Channel__Base___GetManyPublicByName___Precedent_>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyPublicByName___C_Result {
                        return channel___base____get_many_public_by_name____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut Channel__Base___GetManyPublicByName___C_Result| -> () {
                        channel___base____get_many_public_by_name____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                fn _precedent____channel___base____get_many_public_by_name(precedent: Channel__Base___GetManyPublicByName___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Channel__Base___GetManyPublicByName___Outcoming_, Channel__Base___GetManyPublicByName___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyPublicByName___C_Result {
                        return channel___base____get_many_public_by_name____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut Channel__Base___GetManyPublicByName___C_Result| -> () {
                        channel___base____get_many_public_by_name____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____channel___base____get_many_public_by_name() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<Channel__Base___GetManyPublicByName___Precedent_> = vec![];

                    precedent_registry.push(Channel__Base___GetManyPublicByName___Precedent_::ApplicationUserAccessToken_AlreadyExpired);

                    precedent_registry.push(Channel__Base___GetManyPublicByName___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList);

                    '_a: for precedent in precedent_registry {
                        _precedent____channel___base____get_many_public_by_name(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____channel___base____get_one_by_id() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Channel__Base___GetOneById___Outcoming_, Channel__Base___GetOneById___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetOneById___C_Result {
                        return channel___base____get_one_by_id____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut Channel__Base___GetOneById___C_Result| -> () {
                        channel___base____get_one_by_id____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____channel___base____get_one_by_id() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut channel_inner_link_registry: Vec<ChannelInnerLink1_> = vec![];

                    '_a: for _ in 1..=5 {
                        let channel_inner_link_1 = ChannelInnerLink1_ {
                            channel_inner_link_to: Channel_Id(0),
                        };

                        channel_inner_link_registry.push(channel_inner_link_1);
                    }

                    let mut channel_outer_link_registry: Vec<ChannelOuterLink1_> = vec![];

                    '_a: for _ in 1..=5 {
                        let channel_outer_link_1 = ChannelOuterLink1_ {
                            channel_outer_link_alias: ChannelOuterLink_Alias(STRING_LITERAL.to_string()),
                            channel_outer_link_address: ChannelOuterLink_Address(STRING_LITERAL.to_string()),
                        };

                        channel_outer_link_registry.push(channel_outer_link_1);
                    }

                    let channel_2 = Channel2_ {
                        channel_owner: 0,
                        channel_name: Channel_Name(STRING_LITERAL.to_string()),
                        channel_linked_name: Channel_LinkedName(STRING_LITERAL.to_string()),
                        channel_description: Some(Channel_Description(STRING_LITERAL.to_string())),
                        channel_access_modifier: Channel_AccessModifier(0),
                        channel_visability_modifier: Channel_VisabilityModifier(0),
                        channel_orientation: Channel_Orientation(vec![0, 0, 0]),
                        channel_background_image_path: Some(Channel_BackgroundImagePath(STRING_LITERAL.to_string())),
                        channel_cover_image_path: Some(Channel_CoverImagePath(STRING_LITERAL.to_string())),
                        channel_subscribers_quantity: Channel_SubscribersQuantity(0),
                        channel_marks_quantity: Channel_MarksQuantity(0),
                        channel_viewing_quantity: Channel_ViewingQuantity(0),
                    };

                    let outcoming = Channel__Base___GetOneById___Outcoming_ {
                       channel: channel_2,
                       channel_inner_link_registry,
                       channel_outer_link_registry,
                    };

                    let unified_report = UnifiedReport::<Channel__Base___GetOneById___Outcoming_, Channel__Base___GetOneById___Precedent_>::target_filled(outcoming);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetOneById___C_Result {
                        return channel___base____get_one_by_id____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut Channel__Base___GetOneById___C_Result| -> () {
                        channel___base____get_one_by_id____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                fn _precedent____channel___base____get_one_by_id(precedent: Channel__Base___GetOneById___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Channel__Base___GetOneById___Outcoming_, Channel__Base___GetOneById___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetOneById___C_Result {
                        return channel___base____get_one_by_id____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut Channel__Base___GetOneById___C_Result| -> () {
                        channel___base____get_one_by_id____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____channel___base____get_one_by_id() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<Channel__Base___GetOneById___Precedent_> = vec![];

                    precedent_registry.push(Channel__Base___GetOneById___Precedent_::ApplicationUserAccessToken_AlreadyExpired);

                    precedent_registry.push(Channel__Base___GetOneById___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList);

                    precedent_registry.push(Channel__Base___GetOneById___Precedent_::Channel_NotFound);

                    precedent_registry.push(Channel__Base___GetOneById___Precedent_::Channel_IsClose);

                    '_a: for precedent in precedent_registry {
                        _precedent____channel___base____get_one_by_id(precedent)?;
                    }

                    return Ok(());
                }

                #[test]
                fn target_empty____channel_subscription___base____create() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ChannelSubscription__Base___Create___Precedent_>::target_empty();

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ChannelSubscription__Base___Create___C_Result {
                        return channel_subscription___base____create____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ChannelSubscription__Base___Create___C_Result| -> () {
                        channel_subscription___base____create____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn target_filled____channel_subscription___base____create() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }

                fn _precedent____channel_subscription___base____create(precedent: ChannelSubscription__Base___Create___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ChannelSubscription__Base___Create___Precedent_>::precedent(precedent);

                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ChannelSubscription__Base___Create___C_Result {
                        return channel_subscription___base____create____deserialize(vector_of_bytes);
                    };

                    let deallocator = move |c_result: *mut ChannelSubscription__Base___Create___C_Result| -> () {
                        channel_subscription___base____create____deserialize____deallocate(c_result);

                        return ();
                    };

                    return run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    );
                }

                #[test]
                fn precedent____channel_subscription___base____create() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ChannelSubscription__Base___Create___Precedent_> = vec![];

                    precedent_registry.push(ChannelSubscription__Base___Create___Precedent_::ApplicationUserAccessToken_AlreadyExpired);

                    precedent_registry.push(ChannelSubscription__Base___Create___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList);

                    precedent_registry.push(ChannelSubscription__Base___Create___Precedent_::Channel_NotFound);

                    precedent_registry.push(ChannelSubscription__Base___Create___Precedent_::Channel_IsClose);

                    precedent_registry.push(ChannelSubscription__Base___Create___Precedent_::ApplicationUser_IsChannelOwner);

                    '_a: for precedent in precedent_registry {
                        _precedent____channel_subscription___base____create(precedent)?;
                    }

                    return Ok(());
                }
            }
        }

        mod server_request_data_serialization {
            use super::*;

            fn run_by_template<'a, I, A, D>(
                incoming: &'a I,
                allocator: A,
                deallocator: D,
            ) -> Result<(), Box<dyn StdError + 'static>>
            where
                I: Copy,
                A: FnOnce(*mut I) -> *mut C_Result<C_Vector<c_uchar>>,
                D: FnOnce(*mut C_Result<C_Vector<c_uchar>>) -> (),
            {
                let incoming_ = (incoming as *const _) as *mut _;

                let c_result = allocator(incoming_);

                let c_result_ = unsafe {
                    *c_result
                };

                if !c_result_.is_data {
                    return Err("Function inner error.".into());
                }

                deallocator(c_result);

                if incoming_.is_null() {
                    return Err("The pointer must continue to point to the data.".into());
                }

                return Ok(());
            }

            #[test]
            fn application_user___authorization____authorize_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming {
                    application_user_device_id: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_email_or_application_user_nickname: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_password: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____authorize_by_first_step____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____authorize_by_first_step____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_device_id);

                Allocator::<C_String>::deallocate(incoming.application_user_email_or_application_user_nickname);

                Allocator::<C_String>::deallocate(incoming.application_user_password);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____authorize_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___AuthorizeByLastStep___Incoming {
                    application_user_id: 0,
                    application_user_device_id: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_authorization_token_value: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___AuthorizeByLastStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____authorize_by_last_step____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____authorize_by_last_step____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_device_id);

                Allocator::<C_String>::deallocate(incoming.application_user_authorization_token_value);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____check_email_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___CheckEmailForExisting___Incoming {
                    application_user_email: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___CheckEmailForExisting___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____check_email_for_existing____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____check_email_for_existing____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_email);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____check_nickname_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___CheckNicknameForExisting___Incoming {
                    application_user_nickname: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___CheckNicknameForExisting___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____check_nickname_for_existing____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____check_nickname_for_existing____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_nickname);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____deauthorize_from_all_devices() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming {
                    application_user_access_token_encrypted: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____deauthorize_from_all_devices____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____deauthorize_from_all_devices____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_access_token_encrypted);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____deauthorize_from_one_device() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming {
                    application_user_access_token_encrypted: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____deauthorize_from_one_device____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____deauthorize_from_one_device____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_access_token_encrypted);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____refresh_access_token() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___RefreshAccessToken___Incoming {
                    application_user_access_token_encrypted: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_access_refresh_token_encrypted: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___RefreshAccessToken___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____refresh_access_token____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____refresh_access_token____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_access_token_encrypted);

                Allocator::<C_String>::deallocate(incoming.application_user_access_refresh_token_encrypted);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____register_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___RegisterByFirstStep___Incoming {
                    application_user_email: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_device_id: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___RegisterByFirstStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____register_by_first_step____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____register_by_first_step____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_email);

                Allocator::<C_String>::deallocate(incoming.application_user_device_id);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____register_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___RegisterBySecondStep___Incoming {
                    application_user_email: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_device_id: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_registration_token_value: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___RegisterBySecondStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____register_by_second_step____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____register_by_second_step____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_email);

                Allocator::<C_String>::deallocate(incoming.application_user_device_id);

                Allocator::<C_String>::deallocate(incoming.application_user_registration_token_value);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____register_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___RegisterByLastStep___Incoming {
                    application_user_device_id: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_nickname: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_password: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_email: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_registration_token_value: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___RegisterByLastStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____register_by_last_step____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____register_by_last_step____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_device_id);

                Allocator::<C_String>::deallocate(incoming.application_user_nickname);

                Allocator::<C_String>::deallocate(incoming.application_user_password);

                Allocator::<C_String>::deallocate(incoming.application_user_email);

                Allocator::<C_String>::deallocate(incoming.application_user_registration_token_value);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____reset_password_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming {
                    application_user_email: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_device_id: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____reset_password_by_first_step____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____reset_password_by_first_step____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_email);

                Allocator::<C_String>::deallocate(incoming.application_user_device_id);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____reset_password_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming {
                    application_user_id: 0,
                    application_user_device_id: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_reset_password_token_value: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____reset_password_by_second_step____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____reset_password_by_second_step____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_device_id);

                Allocator::<C_String>::deallocate(incoming.application_user_reset_password_token_value);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____reset_password_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming {
                    application_user_id: 0,
                    application_user_device_id: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_password: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_reset_password_token_value: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____reset_password_by_last_step____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____reset_password_by_last_step____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_device_id);

                Allocator::<C_String>::deallocate(incoming.application_user_password);

                Allocator::<C_String>::deallocate(incoming.application_user_reset_password_token_value);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____send_email_for_register() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___SendEmailForRegister___Incoming {
                    application_user_email: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_device_id: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___SendEmailForRegister___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____send_email_for_register____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____send_email_for_register____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_email);

                Allocator::<C_String>::deallocate(incoming.application_user_device_id);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____send_email_for_authorize() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___SendEmailForAuthorize___Incoming {
                    application_user_device_id: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    application_user_id: 0,
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___SendEmailForAuthorize___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____send_email_for_authorize____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____send_email_for_authorize____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_device_id);

                return Ok(());
            }

            #[test]
            fn application_user___authorization____send_email_for_reset_password() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___SendEmailForResetPassword___Incoming {
                    application_user_id: 0,
                    application_user_device_id: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                };

                let allocator = move |incoming: *mut ApplicationUser__Authorization___SendEmailForResetPassword___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____send_email_for_reset_password____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____send_email_for_reset_password____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_device_id);

                return Ok(());
            }

            #[test]
            fn channel___base____get_many_by_name_in_subscriptions() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel__Base___GetManyByNameInSubscriptions___Incoming {
                    application_user_access_token_encrypted: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    channel_name: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    requery_channel_name: C_Option::data(
                        Allocator::<C_String>::allocate(STRING_LITERAL.to_string())
                    ),
                    limit: 0,
                };

                let allocator = move |incoming: *mut Channel__Base___GetManyByNameInSubscriptions___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return channel___base____get_many_by_name_in_subscriptions____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    channel___base____get_many_by_name_in_subscriptions____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_access_token_encrypted);

                Allocator::<C_String>::deallocate(incoming.channel_name);

                Allocator::<C_String>::deallocate(incoming.requery_channel_name.data);

                return Ok(());
            }

            #[test]
            fn channel___base____get_many_by_subscription() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel__Base___GetManyBySubscription___Incoming {
                    application_user_access_token_encrypted: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    requery_channel_id: C_Option::data(0),
                    limit: 0,
                };

                let allocator = move |incoming: *mut Channel__Base___GetManyBySubscription___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return channel___base____get_many_by_subscription____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    channel___base____get_many_by_subscription____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_access_token_encrypted);

                return Ok(());
            }

            #[test]
            fn channel___base____get_many_public_by_name() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel__Base___GetManyPublicByName___Incoming {
                    application_user_access_token_encrypted: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    channel_name: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    requery_channel_name: C_Option::data(Allocator::<C_String>::allocate(STRING_LITERAL.to_string())),
                    limit: 0,
                };

                let allocator = move |incoming: *mut Channel__Base___GetManyPublicByName___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return channel___base____get_many_public_by_name____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    channel___base____get_many_public_by_name____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_access_token_encrypted);

                Allocator::<C_String>::deallocate(incoming.channel_name);

                Allocator::<C_String>::deallocate(incoming.requery_channel_name.data);

                return Ok(());
            }

            #[test]
            fn channel___base____get_one_by_id() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel__Base___GetOneById___Incoming {
                    application_user_access_token_encrypted: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    channel_id: 0,
                };

                let allocator = move |incoming: *mut Channel__Base___GetOneById___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return channel___base____get_one_by_id____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    channel___base____get_one_by_id____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_access_token_encrypted);

                return Ok(());
            }

            #[test]
            fn channel_subscription___base____create() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ChannelSubscription__Base___Create___Incoming {
                    application_user_access_token_encrypted: Allocator::<C_String>::allocate(STRING_LITERAL.to_string()),
                    channel_id: 0,
                };

                let allocator = move |incoming: *mut ChannelSubscription__Base___Create___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return channel_subscription___base____create____serialize(incoming);
                };

                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    channel_subscription___base____create____serialize____deallocate(c_result);

                    return ();
                };

                run_by_template(
                    &incoming,
                    allocator,
                    deallocator
                )?;

                Allocator::<C_String>::deallocate(incoming.application_user_access_token_encrypted);

                return Ok(());
            }
        }
    }
}