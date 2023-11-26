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
use entity::application_user_access_refresh_token_encrypted::ApplicationUserAccessRefreshTokenEncrypted;
use entity::application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted;
use entity::application_user_authorization_token::ApplicationUserAuthorizationToken_Value;
use entity::application_user_device::ApplicationUserDevice_Id;
use entity::application_user_registration_token::ApplicationUserRegistrationToken_Value;
use entity::application_user_reset_password_token::ApplicationUserResetPasswordToken_Value;
use entity::application_user::ApplicationUser_Email;
use entity::application_user::ApplicationUser_Id;
use entity::application_user::ApplicationUser_Nickname;
use entity::application_user::ApplicationUser_Password;
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
use std::error::Error;
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
use libc::c_double;
use libc::c_int;






#[no_mangle]
pub extern "C" fn f1(a: c_int) -> c_int {
    return a;
}

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_first_step____serialize(a: c_int) -> c_int {
    return a;
}

#[no_mangle]
pub extern "C" fn f2(a: bool) -> bool {
    return a;
}

#[no_mangle]
pub extern "C" fn f3(a: c_double) -> c_double {
    return a;
}

#[no_mangle]
pub extern "C" fn f4(a: A) -> c_int {
    return a.a;
}

#[no_mangle]
pub extern "C" fn f5(a: A) -> B {
    return B { a: a.a };
}

#[no_mangle]
pub extern "C" fn f6(a: A, b: B) -> bool {
    return a.a == b.a;
}

#[no_mangle]
pub extern "C" fn f7(a: *const c_int,) -> c_int {
    let a_ = unsafe { *a };

    return a_;
}

#[no_mangle]
pub extern "C" fn f8(a: *mut c_int,) -> c_int {
    let a_ = unsafe { *a };

    return a_;
}

#[no_mangle]
pub extern "C" fn f9(a: *const A,) -> c_int {
    let a_ = unsafe { *a };

    return a_.a;
}

#[no_mangle]
pub extern "C" fn f10(a: *mut A,) -> c_int {
    let a_ = unsafe { *a };

    return a_.a;
}

#[no_mangle]
pub extern "C" fn f11(a: *mut A, b: *mut B,) -> bool {
    let a_ = unsafe { *a };

    let b_ = unsafe { *b };

    if a_.a == b_.a {
        return true;
    }

    return false;
}

#[no_mangle]
pub extern "C" fn f12(a: *mut C, b: c_int,) -> C {
    let mut a_ = unsafe { *a };

    a_.a = b;

    return a_;
}

#[no_mangle]
pub extern "C" fn f13(a: *mut C, b: bool,) -> () {
    let mut a_ = unsafe { *a };

    a_.b = b;

    return ();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct A {
    pub a: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct B {
    pub a: c_int,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct C {
    pub a: c_int,
    pub b: bool,
}




#[no_mangle]
pub extern "C" fn string_allocate_f1() -> *mut c_char {
    let string = CString::new("qwerty").unwrap();

    return string.into_raw();
}

#[no_mangle]
pub extern "C" fn string_deallocate_f1(string: *mut c_char) -> () {
    if string.is_null() {
        return ();
    }

    let _ = unsafe {
        CString::from_raw(string)
    };

    return ();
}




#[no_mangle]
pub extern "C" fn string_allocate_f2() -> *mut StructWithString1 {
    let string = CString::new("qwerty").unwrap();

    return Box::into_raw(
        Box::new(
            StructWithString1 {
                is_exist: true,
                string: string.into_raw()
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn string_deallocate_f2(struct_with_string: *mut StructWithString1) -> () {
    if struct_with_string.is_null() {
        return ();
    }

    let struct_with_string = unsafe {
        Box::from_raw(struct_with_string)
    };

    let string = struct_with_string.string;

    if !string.is_null() {
        let _ = unsafe {
            CString::from_raw(string)
        };
    }

    return ();
}

#[repr(C)]
pub struct StructWithString1 {
    pub is_exist: bool,
    pub string: *mut c_char,
}


#[no_mangle]
pub extern "C" fn string_allocate_f22() -> *mut ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming {
    let string = CString::new("qwerty").unwrap();

    return Box::into_raw(
        Box::new(
            ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming {
                is_exist: true,
                string: string.into_raw()
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn string_deallocate_f22(struct_with_string: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming) -> () {
    if struct_with_string.is_null() {
        return ();
    }

    let struct_with_string = unsafe {
        Box::from_raw(struct_with_string)
    };

    let string = struct_with_string.string;

    if !string.is_null() {
        let _ = unsafe {
            CString::from_raw(string)
        };
    }

    return ();
}

#[repr(C)]
pub struct ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming {
    pub is_exist: bool,
    pub string: *mut c_char,
}




#[no_mangle]
pub extern "C" fn string_allocate_f3() -> *mut StructWithString2 {
    let string = match CString::new("qwerty") {
        Ok(string_) => string_,
        Err(_) => {
            return Box::into_raw(
                Box::new(
                    StructWithString2 {
                        struct_with_string: StructWithString1 {
                            is_exist: false,
                            string: ptr::null_mut(),
                        },
                        error: Error2 {
                            is_exist: true,
                        },
                    }
                )
            )
        }
    };

    return Box::into_raw(
        Box::new(
            StructWithString2 {
                struct_with_string: StructWithString1 {
                    is_exist: true,
                    string: string.into_raw(),
                },
                error: Error2 {
                    is_exist: false,
                },
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn string_deallocate_f3(struct_with_string: *mut StructWithString2) -> () {
    if struct_with_string.is_null() {
        return ();
    }

    let struct_with_string = unsafe {
        Box::from_raw(struct_with_string)
    };

    let string = struct_with_string.struct_with_string.string;

    if !string.is_null() {
        let _ = unsafe {
            CString::from_raw(string)
        };
    }

    return ();
}


#[repr(C)]
pub struct StructWithString2 {
    pub struct_with_string: StructWithString1,
    pub error: Error2,
}

#[repr(C)]
pub struct Error2 {
    pub is_exist: bool
}




#[no_mangle]
pub extern "C" fn array_slice_f1(pointer_to_first_element_of_registry: *mut c_uchar, registry_length: size_t) -> c_uchar {
    let zero = 0 as c_uchar;

    if pointer_to_first_element_of_registry.is_null() || registry_length == 0 {
        return zero;
    }

    let registry = unsafe {
        slice::from_raw_parts::<u8>(pointer_to_first_element_of_registry, registry_length )
    };

    if registry.len() == 0 {
        return zero;
    }

    let element = match registry.last() {
        Some(element_) => element_,
        None => {
            return zero;
        }
    };

    return *element as c_uchar;
}


#[repr(C)]
#[derive(Clone, Copy)]
pub struct Opaque {
    pub public: bool,
    _private: bool,
}

#[no_mangle]
pub extern "C" fn opaque_f1(opaque: *mut Opaque) -> bool {
    let opaque_ = unsafe { *opaque };

    return opaque_.public == opaque_._private;
}




#[no_mangle]
pub extern "C" fn generic_allocate_f1() -> *mut StructWithGeneric<c_char> {
    return Box::into_raw(
        Box::new(
            StructWithGeneric {
                a: 0 as c_char,
                b: true,
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn generic_deallocate_f1(struct_with_generic: *mut StructWithGeneric<c_char>) -> () {
    if struct_with_generic.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(struct_with_generic)
    };

    return ();
}




#[no_mangle]
pub extern "C" fn generic_allocate_f2() -> *mut StructWithGeneric<*mut c_char> {
    return Box::into_raw(
        Box::new(
            StructWithGeneric {
                a: CString::new("qwerty").unwrap().into_raw(),
                b: true,
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn generic_deallocate_f2(struct_with_generic: *mut StructWithGeneric<*mut c_char>) -> () {
    if struct_with_generic.is_null() {
        return ();
    }

    let struct_with_generic = unsafe {
        Box::from_raw(struct_with_generic)
    };

    let string = struct_with_generic.a;

    if !string.is_null() {
        let _ = unsafe {
            CString::from_raw(string)
        };
    }

    return ();
}

#[repr(C)]
pub struct StructWithGeneric<T> {
    pub a: T,
    pub b: bool,
}






#[repr(C)]
#[derive(Clone, Copy)]
pub struct Main1 {
    pub nested1: Nested1,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Main2 {
    pub nested1: Nested1,
    pub nested2: Nested2,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Nested1 {
    pub a: bool,
    pub b: bool,
    pub c: bool,
    pub string: *mut c_char,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Nested2 {
    pub a: bool,
}

#[no_mangle]
pub extern "C" fn main_nested_allocate_f1() -> *mut Main1 {
    return Box::into_raw(
        Box::new(
            Main1 { nested1: Nested1 { a: true, b: false, c: true, string: CString::new("qwerty").unwrap().into_raw() } }
        )
    )
}

#[no_mangle]
pub extern "C" fn main_nested_deallocate_f1(main: *mut Main1) -> () {
    if main.is_null() {
        return ();
    }

    let main = unsafe {
        Box::from_raw(main)
    };

    if !main.nested1.string.is_null() {
        let _ = unsafe {
            CString::from_raw(main.nested1.string)
        };
    }

    return ();
}

#[no_mangle]
pub extern "C" fn main_nested_allocate_f2() -> *mut Main2 {
    return Box::into_raw(
        Box::new(
            Main2 {
                nested1: Nested1 {
                    a: true,
                    b: false,
                    c: true,
                    string: CString::new("qwerty_12334_qwertyu").unwrap().into_raw()
                },
                nested2: Nested2 {
                    a: true
                }
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn main_nested_deallocate_f2(main: *mut Main2) -> () {
    if main.is_null() {
        return ();
    }

    let main = unsafe {
        Box::from_raw(main)
    };

    if !main.nested1.string.is_null() {
        let _ = unsafe {
            CString::from_raw(main.nested1.string)
        };
    }

    return ();
}












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
    fn to_string(self) -> Result<String, Box<dyn Error + 'static>> {
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
        F: FnOnce(UnifiedReport<APO1, APP1>) -> Result<C_UnifiedReport<APO2, APP2>, Box<dyn Error + 'static>>,
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
        F: FnOnce(I1) -> Result<I2, Box<dyn Error + 'static>>,
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
#[derive(Clone, Copy)]
pub struct ApplicationUser__Authorization___CheckEmailForExisting___Incoming {
    pub application_user_email: C_String,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____check_email_for_existing____serialize(
    incoming: *mut ApplicationUser__Authorization___CheckEmailForExisting___Incoming
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: ApplicationUser__Authorization___CheckEmailForExisting___Incoming| -> Result<ApplicationUser__Authorization___CheckEmailForExisting___Incoming_, Box<dyn Error + 'static>> {
        let incoming_ = ApplicationUser__Authorization___CheckEmailForExisting___Incoming_ {
            application_user_email: ApplicationUser_Email(incoming.application_user_email.to_string()?),
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
    let converter = move |unified_report: UnifiedReport<ApplicationUser__Authorization___CheckEmailForExisting___Outcoming_, Void>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___CheckEmailForExisting___Outcoming, C_Void>, Box<dyn Error + 'static>> {
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
