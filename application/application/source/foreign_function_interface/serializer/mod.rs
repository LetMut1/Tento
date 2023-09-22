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







// TODO cargo build --release --lib --target aarch64-apple-ios
// TODO cargo build --release --lib --target aarch64-apple-ios-sim
// TODO cargo build --release --lib --target armv7-linux-androideabi


// TODO Везде в unsafe функциях (slice::from_raw_parts, ...) ядра посмотреть требования к параметрам и попробовать сделать проверку.






use libc::c_int;
use libc::c_short;
use libc::c_long;
use libc::c_double;
use libc::c_uchar;
use libc::c_char;
use std::ffi::CString;
use std::slice;
use libc::size_t;
use std::default::Default;
use std::ptr;
use unified_report::UnifiedReport as UnifiedReport_;
use unified_report::Data as Data_;
use action_processor_incoming_outcoming::action_processor::application_user___authorization;
use action_processor_incoming_outcoming::action_processor::channel___base;
use action_processor_incoming_outcoming::action_processor::channel_subscription___base;
use message_pack_serializer::Serializer;

#[no_mangle]
pub extern "C" fn f1(a: c_int) -> c_int {
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
                string: string.into_raw(),
                _private: 0
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn string_deallocate_f2(struct_with_string: *mut StructWithString1) -> () {
    if struct_with_string.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(struct_with_string)
    };

    return ();
}

#[repr(C)]
pub struct StructWithString1 {
    pub is_exist: bool,
    pub string: *mut c_char,
    _private: c_char,
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
                            _private: 0
                        },
                        error: Error2 {
                            is_exist: true,
                        },
                        _private: 0
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
                    _private: 0
                },
                error: Error2 {
                    is_exist: false,
                },
                _private: 0
            }
        )
    )
}

#[no_mangle]
pub extern "C" fn string_deallocate_f3(struct_with_string: *mut StructWithString2) -> () {
    if struct_with_string.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(struct_with_string)
    };

    return ();
}

#[repr(C)]
pub struct StructWithString2 {
    pub struct_with_string: StructWithString1,
    pub error: Error2,
    _private: c_char
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
        slice::from_raw_parts::<u8>(pointer_to_first_element_of_registry, registry_length as usize)
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


































#[repr(C)]
pub struct Result<T> {
    pub data: T,
    // If false, then it means an error occurred. In general,
    // if everything is integrated correctly, there can be no error.
    pub is_data: bool,
}

impl<T> Result<T> {
    fn data(data: T) -> Self {
        return Self {
            data,
            is_data: true,
        };
    }
}

impl<T> Result<T>
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
#[derive(Default)]
pub struct UnifiedReport<D, P> {
    pub target: Data<D>,
    pub precedent: P,
    // If false, then it means we have to work with precedent.
    pub is_target: bool,
}

impl<D, P> UnifiedReport<D, P>
where
    P: Default
{
    fn target(target: Data<D>) -> Self {
        return Self {
            target,
            precedent: P::default(),
            is_target: true,
        };
    }
}

impl<D, P> UnifiedReport<D, P>
where
    D: Default
{
    fn precedent(precedent: P) -> Self {
        return Self {
            target: Data::<D>::default(),
            precedent,
            is_target: false,
        };
    }
}

#[repr(C)]
#[derive(Default)]
pub struct Data<T> {
    pub filled: T,
    // If false, then it means data is empty.
    pub is_filled: bool,
}

impl<T> Data<T> {
    fn filled(filled: T) -> Self {
        return Data {
            filled,
            is_filled: true,
        };
    }
}

impl<T> Data<T>
where
    T: Default
{
    fn empty() -> Self {
        return Data {
            filled: T::default(),
            is_filled: false,
        };
    }
}

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_first_step____deserialize(
    pointer_to_first_element_of_registry: *mut c_uchar,
    registry_length: size_t
) -> *mut ApplicationUser__Authorization___AuthorizeByFirstStep___Result {
    type Outcoming_ = application_user___authorization::authorize_by_first_step::Outcoming;

    type Precedent_ = application_user___authorization::authorize_by_first_step::Precedent;

    if pointer_to_first_element_of_registry.is_null() || registry_length == 0 {
        return Box::into_raw(
            Box::new(
                Result::error()
            )
        );
    }

    let data = unsafe {
        slice::from_raw_parts::<u8>(pointer_to_first_element_of_registry as *mut u8, registry_length as usize)
    };

    let unified_report = match Serializer::deserialize::<'_, UnifiedReport_<Outcoming_, Precedent_>>(data) {
        Ok(unified_report_) => unified_report_,
        Err(_) => {
            return Box::into_raw(
                Box::new(
                    Result::error()
                )
            );
        }
    };

    let unified_report_ = match unified_report {
        UnifiedReport_::Target { data } => {
            let data_ = match data {
                Data_::Empty => {
                    Data::<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming>::empty()
                }
                Data_::Filled { data: data__ } => {
                    let outcoming = ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming {
                        application_user_id: data__.application_user_id.0 as c_long,
                        verification_message_sent: data__.verification_message_sent,
                        application_user_authorization_token_can_be_resent_from: data__.application_user_authorization_token_can_be_resent_from.0 as c_long,
                        application_user_authorization_token_wrong_enter_tries_quantity: data__.application_user_authorization_token_wrong_enter_tries_quantity.0 as c_short,
                        application_user_authorization_token_wrong_enter_tries_quantity_limit: data__.application_user_authorization_token_wrong_enter_tries_quantity_limit as c_short,
                    };

                    Data::filled(outcoming)
                }
            };

            UnifiedReport::target(data_)
        }
        UnifiedReport_::Precedent { precedent } => {
            match precedent {
                Precedent_::ApplicationUser_WrongEmailOrNicknameOrPassword => {}
            };

            let precedent_ = ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent {
                application_user__wrong_email_or_nickname_or_password: true,
            };

            UnifiedReport::precedent(precedent_)
        }
    };

    return Box::into_raw(
        Box::new(
            Result::data(unified_report_)
        )
    );
}

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_first_step____deallocate(
    result: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}

type ApplicationUser__Authorization___AuthorizeByFirstStep___Result = Result<UnifiedReport<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming, ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming {
    pub application_user_id: c_long,
    pub verification_message_sent: bool,
    pub application_user_authorization_token_can_be_resent_from: c_long,
    pub application_user_authorization_token_wrong_enter_tries_quantity: c_short,
    pub application_user_authorization_token_wrong_enter_tries_quantity_limit: c_short,
}

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent {
    pub application_user__wrong_email_or_nickname_or_password: bool,
}
