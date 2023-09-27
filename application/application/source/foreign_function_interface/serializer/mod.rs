// TODO cargo build --release --lib --target aarch64-apple-ios
// TODO cargo build --release --lib --target aarch64-apple-ios-sim
// TODO cargo build --release --lib --target armv7-linux-androideabi

// TODO access_modifier/visability_modifier посмотреть, как на бэкенде лежат в бд и отдаются. Здесь сделать структуру
// TODO unit.tests






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

use libc::c_int;
use libc::c_short;
use libc::c_long;
use libc::c_double;
use libc::c_uchar;
use libc::c_char;
use void::Void;
use std::error::Error;
use std::slice::from_raw_parts;
use std::ffi::CString;
use std::slice;
use std::mem::forget;
use libc::size_t;
use std::default::Default;
use std::result::Result;
use std::boxed::Box;
use std::ptr;
use serde::Deserialize;
use unified_report::UnifiedReport;
use std::marker::PhantomData;
use std::ptr::slice_from_raw_parts_mut;
use unified_report::Data;
use action_processor_incoming_outcoming::action_processor::application_user___authorization;
use action_processor_incoming_outcoming::action_processor::channel___base;
use action_processor_incoming_outcoming::action_processor::channel_subscription___base;
use message_pack_serializer::Serializer as Serializer_;

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





























#[repr(C)]
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
#[derive(Default)]
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
#[derive(Default)]
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
#[derive(Default)]
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
    fn allocate(vector: Vec<T>) -> C_Vector<T> {                                 // TODO Discord Ping.
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

        let pointer = slice_from_raw_parts_mut(c_vector.pointer, c_vector.length );

        let _ = unsafe {
            Box::from_raw(pointer)
        };

        return ();
    }
}

struct Serilizer;

impl Serilizer {
    fn deserialize<APO1, APP1, F, APO2, APP2>(
        vector_of_bytes: *mut C_Vector<c_uchar>,
        converter: F
    ) -> *mut C_Result<C_UnifiedReport<APO2, APP2>>
    where
        APO1: for<'de> Deserialize<'de>,
        APP1: for<'de> Deserialize<'de>,
        APO2: Default,
        APP2: Default,
        F: FnOnce(UnifiedReport<APO1, APP1>) -> Result<C_UnifiedReport<APO2, APP2>, Box<dyn Error + 'static>>
    {
        if vector_of_bytes.is_null() {
            return Box::into_raw(
                Box::new(
                    C_Result::error()
                )
            );
        }

        let vector_ = unsafe {
            *vector_of_bytes
        };

        if vector_.pointer.is_null() || vector_.length == 0 {
            return Box::into_raw(
                Box::new(
                    C_Result::error()
                )
            );
        }

        let data = unsafe {
            slice::from_raw_parts::<u8>(vector_.pointer as *mut u8, vector_.length )
        };

        let unified_report = match Serializer_::deserialize::<'_, UnifiedReport<APO1, APP1>>(data) {
            Ok(unified_report_) => unified_report_,
            Err(_) => {
                return Box::into_raw(
                    Box::new(
                        C_Result::error()
                    )
                );
            }
        };

        let unified_report_ = match converter(unified_report) {
            Ok(unified_report__) => unified_report__,
            Err(_) => {
                return Box::into_raw(
                    Box::new(
                        C_Result::error()
                    )
                );
            }
        };

        return Box::into_raw(
            Box::new(
                C_Result::data(unified_report_)
            )
        );
    }
}

#[repr(C)]
#[derive(Default)]
pub struct Common1 {
    pub channel: Channel1,
    pub is_application_user_subscribed: bool,
}

#[repr(C)]
#[derive(Default)]
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
#[derive(Default)]
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
#[derive(Default)]
pub struct ChannelInnerLink1 {
    pub channel_inner_link_to: c_long,
}

pub struct ChannelOuterLink1 {
    pub channel_outer_link_alias: C_String,
    pub channel_outer_link_address: C_String,
}

type ApplicationUser__Authorization___AuthorizeByFirstStep___Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming, ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent>>;

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

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_first_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___AuthorizeByFirstStep___Result {
    type Outcoming_ = application_user___authorization::authorize_by_first_step::Outcoming;

    type Precedent_ = application_user___authorization::authorize_by_first_step::Precedent;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming, ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent>, Box<dyn Error + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming {
                            application_user_id: data__.application_user_id.0,
                            verification_message_sent: data__.verification_message_sent,
                            application_user_authorization_token_can_be_resent_from: data__.application_user_authorization_token_can_be_resent_from.0,
                            application_user_authorization_token_wrong_enter_tries_quantity: data__.application_user_authorization_token_wrong_enter_tries_quantity.0,
                            application_user_authorization_token_wrong_enter_tries_quantity_limit: data__.application_user_authorization_token_wrong_enter_tries_quantity_limit,
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                match precedent {
                    Precedent_::ApplicationUser_WrongEmailOrNicknameOrPassword => {}
                };

                let precedent_ = ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent {
                    application_user__wrong_email_or_nickname_or_password: true,
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Serilizer::deserialize(vector_of_bytes, converter);
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

type ApplicationUser__Authorization___AuthorizeByLastStep___Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming, ApplicationUser__Authorization___AuthorizeByLastStep___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming {
    pub application_user_access_token_encrypted: C_String,
    pub application_user_access_refresh_token_encrypted: C_String,
}

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___AuthorizeByLastStep___Precedent {
    pub application_user_authorization_token__not_found: bool,
    pub application_user_authorization_token__already_expired: bool,
    pub application_user_authorization_token__wrong_value: ApplicationUserAuthorizationToken_WrongValue,
    pub application_user__not_found: bool,
}

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUserAuthorizationToken_WrongValue {
    pub is_exist: bool,
    pub application_user_authorization_token_wrong_enter_tries_quantity: c_short,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_last_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___AuthorizeByLastStep___Result {
    type Outcoming_ = application_user___authorization::authorize_by_last_step::Outcoming;

    type Precedent_ = application_user___authorization::authorize_by_last_step::Precedent;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming, ApplicationUser__Authorization___AuthorizeByLastStep___Precedent>, Box<dyn Error + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming {
                            application_user_access_token_encrypted: Allocator::<C_String>::allocate(data__.application_user_access_token_encrypted.0),
                            application_user_access_refresh_token_encrypted: Allocator::<C_String>::allocate(data__.application_user_access_refresh_token_encrypted.0),
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    Precedent_::ApplicationUserAuthorizationToken_AlreadyExpired => {
                        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent {
                            application_user_authorization_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserAuthorizationToken_NotFound => {
                        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent {
                            application_user_authorization_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserAuthorizationToken_WrongValue { application_user_authorization_token_wrong_enter_tries_quantity } => {
                        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent {
                            application_user_authorization_token__wrong_value: ApplicationUserAuthorizationToken_WrongValue {
                                is_exist: true,
                                application_user_authorization_token_wrong_enter_tries_quantity: application_user_authorization_token_wrong_enter_tries_quantity.0,
                            },
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUser_NotFound => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_last_step____deallocate(
    result: *mut ApplicationUser__Authorization___AuthorizeByLastStep___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(result)
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

type ApplicationUser__Authorization___CheckEmailForExisting___Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___CheckEmailForExisting___Outcoming, C_Void>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___CheckEmailForExisting___Outcoming {
    pub result: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____check_email_for_existing____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___CheckEmailForExisting___Result {
    type Outcoming_ = application_user___authorization::check_email_for_existing::Outcoming;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Void>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___CheckEmailForExisting___Outcoming, C_Void>, Box<dyn Error + 'static>> {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____check_email_for_existing____deallocate(
    result: *mut ApplicationUser__Authorization___CheckEmailForExisting___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}

type ApplicationUser__Authorization___CheckNicknameForExisting___Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming, C_Void>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming {
    pub result: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____check_nickname_for_existing____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___CheckNicknameForExisting___Result {
    type Outcoming_ = application_user___authorization::check_nickname_for_existing::Outcoming;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Void>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming, C_Void>, Box<dyn Error + 'static>> {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____check_nickname_for_existing____deallocate(
    result: *mut ApplicationUser__Authorization___CheckNicknameForExisting___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}

type ApplicationUser__Authorization___DeauthorizeFromAllDevices___Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_all_devices____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___Result {
    type Precedent_ = application_user___authorization::deauthorize_from_all_devices::Precedent;

    let converter = move |unified_report: UnifiedReport<Void, Precedent_>| -> Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent>, Box<dyn Error + 'static>> {
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
                    Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_all_devices____deallocate(
    result: *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}

type ApplicationUser__Authorization___DeauthorizeFromOneDevice___Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_one_device____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___Result {
    type Precedent_ = application_user___authorization::deauthorize_from_one_device::Precedent;

    let converter = move |unified_report: UnifiedReport<Void, Precedent_>| -> Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent>, Box<dyn Error + 'static>> {
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
                    Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_one_device____deallocate(
    result: *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}

type ApplicationUser__Authorization___RefreshAccessToken___Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___RefreshAccessToken___Outcoming, ApplicationUser__Authorization___RefreshAccessToken___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___RefreshAccessToken___Outcoming {
    pub application_user_access_token_encrypted: C_String,
    pub application_user_access_refresh_token_encrypted: C_String,
}

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___RefreshAccessToken___Precedent {
    pub application_user_access_refresh_token__not_found: bool,
    pub application_user_access_refresh_token__already_expired: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____refresh_access_token____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___RefreshAccessToken___Result {
    type Outcoming_ = application_user___authorization::refresh_access_token::Outcoming;

    type Precedent_ = application_user___authorization::refresh_access_token::Precedent;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___RefreshAccessToken___Outcoming, ApplicationUser__Authorization___RefreshAccessToken___Precedent>, Box<dyn Error + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___RefreshAccessToken___Outcoming {
                            application_user_access_token_encrypted: Allocator::<C_String>::allocate(data__.application_user_access_token_encrypted.0),
                            application_user_access_refresh_token_encrypted: Allocator::<C_String>::allocate(data__.application_user_access_refresh_token_encrypted.0),
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    Precedent_::ApplicationUserAccessRefreshToken_AlreadyExpired => {
                        ApplicationUser__Authorization___RefreshAccessToken___Precedent {
                            application_user_access_refresh_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserAccessRefreshToken_NotFound => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____refresh_access_token____deallocate(
    result: *mut ApplicationUser__Authorization___RefreshAccessToken___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(result)
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

type ApplicationUser__Authorization___RegisterByFirstStep___Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___RegisterByFirstStep___Outcoming, ApplicationUser__Authorization___RegisterByFirstStep___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___RegisterByFirstStep___Outcoming {
    pub verification_message_sent: bool,
    pub application_user_registration_token_can_be_resent_from: c_long,
    pub application_user_registration_token_wrong_enter_tries_quantity: c_short,
    pub application_user_registration_token_wrong_enter_tries_quantity_limit: c_short,
}

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___RegisterByFirstStep___Precedent {
    pub application_user__email_already_exist: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_first_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___RegisterByFirstStep___Result {
    type Outcoming_ = application_user___authorization::register_by_first_step::Outcoming;

    type Precedent_ = application_user___authorization::register_by_first_step::Precedent;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___RegisterByFirstStep___Outcoming, ApplicationUser__Authorization___RegisterByFirstStep___Precedent>, Box<dyn Error + 'static>> {
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
                    Precedent_::ApplicationUser_EmailAlreadyExist => {}
                };

                let precedent_ = ApplicationUser__Authorization___RegisterByFirstStep___Precedent {
                    application_user__email_already_exist: true,
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_first_step____deallocate(
    result: *mut ApplicationUser__Authorization___RegisterByFirstStep___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}

type ApplicationUser__Authorization___RegisterBySecondStep___Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___RegisterBySecondStep___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___RegisterBySecondStep___Precedent {
    pub application_user_registration_token__not_found: bool,
    pub application_user_registration_token__already_expired: bool,
    pub application_user_registration_token__already_approved: bool,
    pub application_user_registration_token__wrong_value: ApplicationUserRegistrationToken_WrongValue,
}

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUserRegistrationToken_WrongValue {
    pub is_exist: bool,
    pub application_user_registration_token_wrong_enter_tries_quantity: c_short,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_second_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___RegisterBySecondStep___Result {
    type Precedent_ = application_user___authorization::register_by_second_step::Precedent;

    let converter = move |unified_report: UnifiedReport<Void, Precedent_>| -> Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___RegisterBySecondStep___Precedent>, Box<dyn Error + 'static>> {
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
                    Precedent_::ApplicationUserRegistrationToken_NotFound => {
                        ApplicationUser__Authorization___RegisterBySecondStep___Precedent {
                            application_user_registration_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserRegistrationToken_AlreadyExpired => {
                        ApplicationUser__Authorization___RegisterBySecondStep___Precedent {
                            application_user_registration_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserRegistrationToken_AlreadyApproved => {
                        ApplicationUser__Authorization___RegisterBySecondStep___Precedent {
                            application_user_registration_token__already_approved: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserRegistrationToken_WrongValue { application_user_registration_token_wrong_enter_tries_quantity } => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_second_step____deallocate(
    result: *mut ApplicationUser__Authorization___RegisterBySecondStep___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}

type ApplicationUser__Authorization___RegisterByLastStep___Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___RegisterByLastStep___Outcoming, ApplicationUser__Authorization___RegisterByLastStep___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___RegisterByLastStep___Outcoming {
    pub application_user_access_token_encrypted: C_String,
    pub application_user_access_refresh_token_encrypted: C_String,
}

#[repr(C)]
#[derive(Default)]
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
) -> *mut ApplicationUser__Authorization___RegisterByLastStep___Result {
    type Outcoming_ = application_user___authorization::register_by_last_step::Outcoming;

    type Precedent_ = application_user___authorization::register_by_last_step::Precedent;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___RegisterByLastStep___Outcoming, ApplicationUser__Authorization___RegisterByLastStep___Precedent>, Box<dyn Error + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___RegisterByLastStep___Outcoming {
                            application_user_access_token_encrypted: Allocator::<C_String>::allocate(data__.application_user_access_token_encrypted.0),
                            application_user_access_refresh_token_encrypted: Allocator::<C_String>::allocate(data__.application_user_access_refresh_token_encrypted.0),
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    Precedent_::ApplicationUser_NicknameAlreadyExist => {
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent {
                            application_user__nickname_already_exist: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUser_EmailAlreadyExist => {
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent {
                            application_user__email_already_exist: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserRegistrationToken_NotFound => {
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent {
                            application_user_registration_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserRegistrationToken_AlreadyExpired => {
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent {
                            application_user_registration_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserRegistrationToken_IsNotApproved => {
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent {
                            application_user_registration_token__is_not_approved: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserRegistrationToken_WrongValue => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_last_step____deallocate(
    result: *mut ApplicationUser__Authorization___RegisterByLastStep___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(result)
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

type ApplicationUser__Authorization___ResetPasswordByFirstStep___Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming, ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming {
    pub application_user_id: c_long,
    pub verification_message_sent: bool,
    pub application_user_reset_password_token_can_be_resent_from: c_long,
    pub application_user_reset_password_token_wrong_enter_tries_quantity: c_short,
    pub application_user_reset_password_token_wrong_enter_tries_quantity_limit: c_short,
}

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent {
    pub application_user__not_found: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_first_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___Result {
    type Outcoming_ = application_user___authorization::reset_password_by_first_step::Outcoming;

    type Precedent_ = application_user___authorization::reset_password_by_first_step::Precedent;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming, ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent>, Box<dyn Error + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming {
                            application_user_id: data__.application_user_id.0,
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
                    Precedent_::ApplicationUser_NotFound => {}
                };

                let precedent_ = ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent {
                    application_user__not_found: true
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_first_step____deallocate(
    result: *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}

type ApplicationUser__Authorization___ResetPasswordBySecondStep___Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent {
    pub application_user_reset_password_token__not_found: bool,
    pub application_user_reset_password_token__already_expired: bool,
    pub application_user_reset_password_token__already_approved: bool,
    pub application_user_reset_password_token__wrong_value: ApplicationUserResetPasswordToken_WrongValue,
}

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUserResetPasswordToken_WrongValue {
    pub is_exist: bool,
    pub application_user_reset_password_token_wrong_enter_tries_quantity: c_short,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_second_step____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___Result {
    type Precedent_ = application_user___authorization::reset_password_by_second_step::Precedent;

    let converter = move |unified_report: UnifiedReport<Void, Precedent_>| -> Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent>, Box<dyn Error + 'static>> {
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
                    Precedent_::ApplicationUserResetPasswordToken_NotFound => {
                        ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent {
                            application_user_reset_password_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserResetPasswordToken_AlreadyExpired => {
                        ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent {
                            application_user_reset_password_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserResetPasswordToken_AlreadyApproved => {
                        ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent {
                            application_user_reset_password_token__already_approved: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserResetPasswordToken_WrongValue { application_user_reset_password_token_wrong_enter_tries_quantity } => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_second_step____deallocate(
    result: *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}

type ApplicationUser__Authorization___ResetPasswordByLastStep___Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent>>;

#[repr(C)]
#[derive(Default)]
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
) -> *mut ApplicationUser__Authorization___ResetPasswordByLastStep___Result {
    type Precedent_ = application_user___authorization::reset_password_by_last_step::Precedent;

    let converter = move |unified_report: UnifiedReport<Void, Precedent_>| -> Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent>, Box<dyn Error + 'static>> {
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
                    Precedent_::ApplicationUser_NotFound => {
                        ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent {
                            application_user__not_found: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserResetPasswordToken_NotFound => {
                        ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent {
                            application_user_reset_password_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserResetPasswordToken_AlreadyExpired => {
                        ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent {
                            application_user_reset_password_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserResetPasswordToken_IsNotApproved => {
                        ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent {
                            application_user_reset_password_token__is_not_approved: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserResetPasswordToken_WrongValue => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_last_step____deallocate(
    result: *mut ApplicationUser__Authorization___ResetPasswordByLastStep___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}

type ApplicationUser__Authorization___SendEmailForRegister___Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForRegister___Outcoming, ApplicationUser__Authorization___SendEmailForRegister___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___SendEmailForRegister___Outcoming {
    pub application_user_registration_token_can_be_resent_from: c_long,
}

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___SendEmailForRegister___Precedent {
    pub application_user_registration_token__not_found: bool,
    pub application_user_registration_token__already_expired: bool,
    pub application_user_registration_token__already_approved: bool,
    pub application_user_registration_token__time_to_resend_has_not_come: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_register____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___SendEmailForRegister___Result {
    type Outcoming_ = application_user___authorization::send_email_for_register::Outcoming;

    type Precedent_ = application_user___authorization::send_email_for_register::Precedent;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForRegister___Outcoming, ApplicationUser__Authorization___SendEmailForRegister___Precedent>, Box<dyn Error + 'static>> {
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
                    Precedent_::ApplicationUserRegistrationToken_NotFound => {
                        ApplicationUser__Authorization___SendEmailForRegister___Precedent {
                            application_user_registration_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserRegistrationToken_AlreadyExpired => {
                        ApplicationUser__Authorization___SendEmailForRegister___Precedent {
                            application_user_registration_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserRegistrationToken_AlreadyApproved => {
                        ApplicationUser__Authorization___SendEmailForRegister___Precedent {
                            application_user_registration_token__already_approved: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserRegistrationToken_TimeToResendHasNotCome => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_register____deallocate(
    result: *mut ApplicationUser__Authorization___SendEmailForRegister___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}

type ApplicationUser__Authorization___SendEmailForAuthorize___Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming, ApplicationUser__Authorization___SendEmailForAuthorize___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming {
    pub application_user_authorization_token_can_be_resent_from: c_long,
}

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___SendEmailForAuthorize___Precedent {
    pub application_user__not_found: bool,
    pub application_user_authorization_token__not_found: bool,
    pub application_user_authorization_token__already_expired: bool,
    pub application_user_authorization_token__time_to_resend_has_not_come: bool,
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_authorize____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___SendEmailForAuthorize___Result {
    type Outcoming_ = application_user___authorization::send_email_for_authorize::Outcoming;

    type Precedent_ = application_user___authorization::send_email_for_authorize::Precedent;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming, ApplicationUser__Authorization___SendEmailForAuthorize___Precedent>, Box<dyn Error + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let outcoming = ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming {
                            application_user_authorization_token_can_be_resent_from: data__.application_user_authorization_token_can_be_resent_from.0,
                        };

                        C_Data::filled(outcoming)
                    }
                };

                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    Precedent_::ApplicationUser_NotFound => {
                        ApplicationUser__Authorization___SendEmailForAuthorize___Precedent {
                            application_user__not_found: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserAuthorizationToken_NotFound => {
                        ApplicationUser__Authorization___SendEmailForAuthorize___Precedent {
                            application_user_authorization_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserAuthorizationToken_AlreadyExpired => {
                        ApplicationUser__Authorization___SendEmailForAuthorize___Precedent {
                            application_user_authorization_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserAuthorizationToken_TimeToResendHasNotCome => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_authorize____deallocate(
    result: *mut ApplicationUser__Authorization___SendEmailForAuthorize___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}

type ApplicationUser__Authorization___SendEmailForResetPassword___Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming, ApplicationUser__Authorization___SendEmailForResetPassword___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming {
    pub application_user_resep_password_token_can_be_resent_from: c_long,
}

#[repr(C)]
#[derive(Default)]
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
) -> *mut ApplicationUser__Authorization___SendEmailForResetPassword___Result {
    type Outcoming_ = application_user___authorization::send_email_for_reset_password::Outcoming;

    type Precedent_ = application_user___authorization::send_email_for_reset_password::Precedent;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Precedent_>| -> Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming, ApplicationUser__Authorization___SendEmailForResetPassword___Precedent>, Box<dyn Error + 'static>> {
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
                    Precedent_::ApplicationUser_NotFound => {
                        ApplicationUser__Authorization___SendEmailForResetPassword___Precedent {
                            application_user__not_found: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserResetPasswordToken_NotFound => {
                        ApplicationUser__Authorization___SendEmailForResetPassword___Precedent {
                            application_user_reset_password_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserResetPasswordToken_AlreadyExpired => {
                        ApplicationUser__Authorization___SendEmailForResetPassword___Precedent {
                            application_user_reset_password_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserResetPasswordToken_AlreadyApproved => {
                        ApplicationUser__Authorization___SendEmailForResetPassword___Precedent {
                            application_user_reset_password_token__already_approved: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserResetPasswordToken_TimeToResendHasNotCome => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_reset_password____deallocate(
    result: *mut ApplicationUser__Authorization___SendEmailForResetPassword___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}

type Channel__Base___GetManyByNameInSubscriptions___Result = C_Result<C_UnifiedReport<Channel__Base___GetManyByNameInSubscriptions___Outcoming, Channel__Base___GetManyByNameInSubscriptions___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct Channel__Base___GetManyByNameInSubscriptions___Outcoming {
    pub common_registry: C_Vector<Common1>,
}

#[repr(C)]
#[derive(Default)]
pub struct Channel__Base___GetManyByNameInSubscriptions___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_by_name_in_subscriptions____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut Channel__Base___GetManyByNameInSubscriptions___Result {
    type Outcoming_ = channel___base::get_many_by_name_in_subscriptions::Outcoming;

    type Precedent_ = channel___base::get_many_by_name_in_subscriptions::Precedent;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Precedent_>| -> Result<C_UnifiedReport<Channel__Base___GetManyByNameInSubscriptions___Outcoming, Channel__Base___GetManyByNameInSubscriptions___Precedent>, Box<dyn Error + 'static>> {
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
                    Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        Channel__Base___GetManyByNameInSubscriptions___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_by_name_in_subscriptions____deallocate(
    result: *mut Channel__Base___GetManyByNameInSubscriptions___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(result)
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

type Channel__Base___GetManyBySubscription___Result = C_Result<C_UnifiedReport<Channel__Base___GetManyBySubscription___Outcoming, Channel__Base___GetManyBySubscription___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct Channel__Base___GetManyBySubscription___Outcoming {
    pub common_registry: C_Vector<Common1>,
}

#[repr(C)]
#[derive(Default)]
pub struct Channel__Base___GetManyBySubscription___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_by_subscription____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut Channel__Base___GetManyBySubscription___Result {
    type Outcoming_ = channel___base::get_many_by_subscription::Outcoming;

    type Precedent_ = channel___base::get_many_by_subscription::Precedent;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Precedent_>| -> Result<C_UnifiedReport<Channel__Base___GetManyBySubscription___Outcoming, Channel__Base___GetManyBySubscription___Precedent>, Box<dyn Error + 'static>> {
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
                    Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        Channel__Base___GetManyBySubscription___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_by_subscription____deallocate(
    result: *mut Channel__Base___GetManyBySubscription___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(result)
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

type Channel__Base___GetManyPublicByName___Result = C_Result<C_UnifiedReport<Channel__Base___GetManyPublicByName___Outcoming, Channel__Base___GetManyPublicByName___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct Channel__Base___GetManyPublicByName___Outcoming {
    pub common_registry: C_Vector<Common1>,
}

#[repr(C)]
#[derive(Default)]
pub struct Channel__Base___GetManyPublicByName___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_public_by_name____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut Channel__Base___GetManyPublicByName___Result {
    type Outcoming_ = channel___base::get_many_public_by_name::Outcoming;

    type Precedent_ = channel___base::get_many_public_by_name::Precedent;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Precedent_>| -> Result<C_UnifiedReport<Channel__Base___GetManyPublicByName___Outcoming, Channel__Base___GetManyPublicByName___Precedent>, Box<dyn Error + 'static>> {
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
                    Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        Channel__Base___GetManyPublicByName___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn channel___base____get_many_public_by_name____deallocate(
    result: *mut Channel__Base___GetManyPublicByName___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(result)
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

type Channel__Base___GetOneById___Result = C_Result<C_UnifiedReport<Channel__Base___GetOneById___Outcoming, Channel__Base___GetOneById___Precedent>>;

#[repr(C)]
#[derive(Default)]
pub struct Channel__Base___GetOneById___Outcoming {
    pub channel: Channel2,
    pub channel_inner_link_registry: C_Vector<ChannelInnerLink1>,
    pub channel_outer_link_registry: C_Vector<ChannelOuterLink1>,
}

#[repr(C)]
#[derive(Default)]
pub struct Channel__Base___GetOneById___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
    pub channel__not_found: bool,
    pub channel__is_close: bool,
}

#[no_mangle]
pub extern "C" fn channel___base____get_one_by_id____deserialize(
    vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut Channel__Base___GetOneById___Result {
    type Outcoming_ = channel___base::get_one_by_id::Outcoming;

    type Precedent_ = channel___base::get_one_by_id::Precedent;

    let converter = move |unified_report: UnifiedReport<Outcoming_, Precedent_>| -> Result<C_UnifiedReport<Channel__Base___GetOneById___Outcoming, Channel__Base___GetOneById___Precedent>, Box<dyn Error + 'static>> {
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
                            channel_owner: data__.channel.channel_owner.0,
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
                    Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        Channel__Base___GetOneById___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
                        Channel__Base___GetOneById___Precedent {
                            application_user_access_token__in_application_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::Channel_NotFound => {
                        Channel__Base___GetOneById___Precedent {
                            channel__not_found: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::Channel_IsClose => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn channel___base____get_one_by_id____deallocate(
    result: *mut Channel__Base___GetOneById___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let result_ = unsafe {
        Box::from_raw(result)
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

type ChannelSubscription__Base___Create___Result = C_Result<C_UnifiedReport<C_Void, ChannelSubscription__Base___Create___Precedent>>;

#[repr(C)]
#[derive(Default)]
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
) -> *mut ChannelSubscription__Base___Create___Result {
    type Precedent_ = channel_subscription___base::create::Precedent;

    let converter = move |unified_report: UnifiedReport<Void, Precedent_>| -> Result<C_UnifiedReport<C_Void, ChannelSubscription__Base___Create___Precedent>, Box<dyn Error + 'static>> {
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
                    Precedent_::ApplicationUserAccessToken_AlreadyExpired => {
                        ChannelSubscription__Base___Create___Precedent {
                            application_user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList => {
                        ChannelSubscription__Base___Create___Precedent {
                            application_user_access_token__in_application_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::Channel_NotFound => {
                        ChannelSubscription__Base___Create___Precedent {
                            channel__not_found: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::Channel_IsClose => {
                        ChannelSubscription__Base___Create___Precedent {
                            channel__is_close: true,
                            ..Default::default()
                        }
                    }
                    Precedent_::ApplicationUser_IsChannelOwner => {
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

    return Serilizer::deserialize(vector_of_bytes, converter);
}

#[no_mangle]
pub extern "C" fn channel_subscription___base____create____deallocate(
    result: *mut ChannelSubscription__Base___Create___Result
) -> () {
    if result.is_null() {
        return ();
    }

    let _ = unsafe {
        Box::from_raw(result)
    };

    return ();
}