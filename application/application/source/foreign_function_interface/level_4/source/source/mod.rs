use bitcode::{
    Decode,
    Encode,
};
use dedicated_crate::{
    action_processor_incoming_outcoming::action_processor::user_authorization::register_by_first_step::{
        Incoming as UserAuthorization_RegisterByFirstStep_Incoming_,
        Outcoming as UserAuthorization_RegisterByFirstStep_Outcoming_,
        Precedent as UserAuthorization_RegisterByFirstStep_Precedent_,
    },
    bit_code_serializer::Serializer,
    unified_report::{
        Data,
        UnifiedReport,
    },
};
use libc::{
    c_char,
    c_long,
    c_short,
    c_uchar,
    size_t,
};
use std::{
    boxed::Box,
    default::Default,
    error::Error as StdError,
    ffi::{
        CStr,
        CString as CString_,
    },
    marker::PhantomData,
    result::Result,
};
use libc::c_ushort;
// level_1---------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn get_number() -> c_char {
    return 69;
}
#[no_mangle]
pub extern "C" fn is_goyda() -> bool {
    return true;
}
#[no_mangle]
pub extern "C" fn convert_to_bool(number: c_char) -> bool {
    return number > 0;
}
// level_1---------------------------------------------------------------------------------
// level_2---------------------------------------------------------------------------------
#[no_mangle]
pub extern "C" fn check_for_nigger_word(pointer_to_string: *const c_char) -> bool {
    let string = unsafe {
        CStr::from_ptr(pointer_to_string)
    }
    .to_str()
    .unwrap();
    return string == "Nigger";
}
#[no_mangle]
pub extern "C" fn get_nigger_word__allocate(quantity: c_uchar) -> *mut c_char {
    let string = if quantity == 0 {
        "Zero Niggers... it is a joke?!".to_string()
    } else {
        "Nigger".repeat(quantity as usize)
    };
    return CString_::new(string).unwrap().into_raw();
}
#[no_mangle]
pub extern "C" fn get_nigger_word__deallocate(pointer_to_string: *mut c_char) -> () {
    let _ = unsafe {
        CString_::from_raw(pointer_to_string)
    };
    return ();
}
// level_2---------------------------------------------------------------------------------
// level_3---------------------------------------------------------------------------------
#[repr(C)]
pub struct A {
    a: c_uchar,
    b: c_uchar,
}
#[no_mangle]
pub extern "C" fn get_a(a: c_uchar) -> A {
    return A {
        a,
        b: a,
    };
}
#[no_mangle]
pub extern "C" fn calculate_a_sum(a: A) -> c_ushort {
    return (a.a as u16) + (a.b as u16);
}
#[repr(C)]
pub struct B {
    a: c_uchar,
    b: c_uchar,
}
#[no_mangle]
pub extern "C" fn get_b__allocate(a: c_uchar) -> *mut B {
    return Box::into_raw(
        Box::new(
            B {
                a,
                b: a,
            },
        ),
    );
}
#[no_mangle]
pub extern "C" fn get_b__deallocate(b: *mut B) -> () {
    let _ = unsafe {
        Box::from_raw(b)
    };
    return ();
}
#[no_mangle]
pub extern "C" fn calculate_b_sum(b: *mut B) -> c_ushort {
    let b = unsafe {
        &*b
    };
    return (b.a as u16) + (b.b as u16);
}
#[repr(C)]
pub struct C {
    a: c_uchar,
    string_pointer: *mut c_char,
}
#[no_mangle]
pub extern "C" fn get_c__allocate() -> *mut C {
    return Box::into_raw(
        Box::new(
            C {
                a: 1,
                string_pointer: CString_::new("Nigger").unwrap().into_raw(),
            },
        ),
    );
}
#[no_mangle]
pub extern "C" fn get_c__deallocate(c: *mut C) -> () {
    let c = unsafe {
        Box::from_raw(c)
    };
    let _ = unsafe {
        CString_::from_raw(c.string_pointer)
    };
    return ();
}
#[no_mangle]
pub extern "C" fn is_c__1_nigger(c: *mut C) -> bool {
    let c = unsafe {
        &*c
    };
    let string = unsafe {
        CStr::from_ptr(c.string_pointer)
    }
    .to_str()
    .unwrap();
    return c.a == 1 && string == "Nigger";
}
#[no_mangle]
pub extern "C" fn is_c__1_nigger___full_struct(c: C) -> bool {
    let string = unsafe {
        CStr::from_ptr(c.string_pointer)
    }
    .to_str()
    .unwrap();
    return c.a == 1 && string == "Nigger";
}
// level_3---------------------------------------------------------------------------------
// level_4---------------------------------------------------------------------------------
const NULL_POINTER_ERROR_MESAGE: &'static str = "There should not be a null-pointer.";
const ALLOCATION_ERROR: &'static str = "Data is not allocated.";
const DEALLOCATION_ERROR: &'static str = "Data is still allocated.";
#[repr(C)]
pub struct CResult<T> {
    pub data: T,
    // If false, then it means an error occurred.
    pub is_data: bool,
}
impl<T> CResult<T> {
    fn data(data: T) -> Self {
        return Self {
            data,
            is_data: true,
        };
    }
    fn into_raw(self) -> *mut Self {
        return Box::into_raw(Box::new(self));
    }
}
impl<T> CResult<T>
where
    T: Default,
{
    fn error() -> Self {
        return Self {
            data: T::default(),
            is_data: false,
        };
    }
}
#[repr(C)]
pub struct COption<T> {
    pub data: T,
    // If false, then it means it it Option::None.
    pub is_data: bool,
}
impl<T> COption<T> {
    fn data(data: T) -> Self {
        return Self {
            data,
            is_data: true,
        };
    }
}
impl<T> COption<T>
where
    T: Default,
{
    fn none() -> Self {
        return Self {
            data: T::default(),
            is_data: false,
        };
    }
}
impl<T> Default for COption<T>
where
    T: Default,
{
    fn default() -> Self {
        return Self::none();
    }
}
#[repr(C)]
#[derive(Default)]
pub struct CUnifiedReport<D, P> {
    pub target: CData<D>,
    pub precedent: P,
    // If false, then it means we have to work with precedent.
    pub is_target: bool,
}
impl<D, P> CUnifiedReport<D, P>
where
    P: Default,
{
    fn target(target: CData<D>) -> Self {
        return Self {
            target,
            precedent: P::default(),
            is_target: true,
        };
    }
}
impl<D, P> CUnifiedReport<D, P>
where
    D: Default,
{
    fn precedent(precedent: P) -> Self {
        return Self {
            target: CData::<D>::default(),
            precedent,
            is_target: false,
        };
    }
}
#[repr(C)]
#[derive(Default)]
pub struct CData<T> {
    pub filled: T,
    // If false, then it means data is empty.
    pub is_filled: bool,
}
impl<T> CData<T> {
    fn filled(filled: T) -> Self {
        return CData {
            filled,
            is_filled: true,
        };
    }
}
impl<T> CData<T>
where
    T: Default,
{
    fn empty() -> Self {
        return CData {
            filled: T::default(),
            is_filled: false,
        };
    }
}
#[repr(C)]
pub struct CString {
    pub pointer: *mut c_char,
}
impl CString {
    fn clone_as_string<'a>(&'a self) -> Result<String, Box<dyn StdError + 'static>> {
        if self.pointer.is_null() {
            return Result::Err(NULL_POINTER_ERROR_MESAGE.into());
        }
        let c_str = unsafe { CStr::from_ptr(self.pointer as *const _) };
        let c_string = c_str.to_str()?.to_string();
        return Result::Ok(c_string);
    }
}
impl Default for CString {
    fn default() -> Self {
        return Self {
            pointer: std::ptr::null_mut(),
        };
    }
}
#[repr(C)]
pub struct CVector<T> {
    pub pointer: *mut T,
    pub length: size_t,
}
impl<T> CVector<T>
where
    T: Clone,
{
    fn clone_as_vec<'a>(&'a self) -> Result<Vec<T>, Box<dyn StdError + 'static>> {
        return Result::Ok(self.as_slice()?.to_vec());
    }
}
impl<T> CVector<T> {
    fn as_slice<'a>(&'a self) -> Result<&'a [T], Box<dyn StdError + 'static>> {
        if self.pointer.is_null() {
            return Result::Err(NULL_POINTER_ERROR_MESAGE.into());
        }
        return Result::Ok(self.as_slice_unchecked());
    }
    fn as_slice_unchecked<'a>(&'a self) -> &'a [T] {
        return unsafe {
            std::slice::from_raw_parts(
                self.pointer as *const _,
                self.length,
            )
        };
    }
}
impl<T> Default for CVector<T> {
    fn default() -> Self {
        return Self {
            pointer: std::ptr::null_mut(),
            length: 0,
        };
    }
}
// Struct for simulating Void type. That is, we will use this structure
// in those moments when we would like to use the classic Void type.
#[repr(C)]
#[derive(Default)]
pub struct CVoid {
    _inner: bool,
}
impl CVoid {
    fn new() -> Self {
        return Self::default();
    }
}
struct Allocator<S> {
    _subject: PhantomData<S>,
}
impl Allocator<CString> {
    fn allocate(string: String) -> CString {
        return CString {
            pointer: unsafe { CString_::from_vec_unchecked(string.into_bytes()) }.into_raw(),
        };
    }
    fn deallocate<'a>(c_string: &'a CString) -> () {
        if c_string.pointer.is_null() {
            return ();
        }
        {
            let _ = unsafe { CString_::from_raw(c_string.pointer) };
        }
        return ();
    }
}
impl<T> Allocator<CVector<T>> {
    #[allow(clippy::mem_forget)]
    fn allocate(vector: Vec<T>) -> CVector<T> {
        let mut boxed_slice = vector.into_boxed_slice();
        let c_vector = CVector {
            pointer: boxed_slice.as_mut_ptr(),
            length: boxed_slice.len(),
        };
        std::mem::forget(boxed_slice);
        return c_vector;
    }
    fn deallocate<'a>(c_vector: &'a CVector<T>) -> () {
        if c_vector.pointer.is_null() {
            return ();
        }
        let pointer = std::ptr::slice_from_raw_parts_mut(
            c_vector.pointer,
            c_vector.length,
        );
        {
            let _ = unsafe { Box::from_raw(pointer) };
        }
        return ();
    }
}
impl Allocator<CResult<CVector<c_uchar>>> {
    fn deallocate(c_result: *mut CResult<CVector<c_uchar>>) -> () {
        if c_result.is_null() {
            return ();
        }
        {
            let c_result_ = unsafe { Box::from_raw(c_result) };
            if c_result_.is_data {
                Allocator::<CVector<_>>::deallocate(&c_result_.data);
            }
        }
        return ();
    }
}
impl<T, E> Allocator<CResult<CUnifiedReport<T, E>>> {
    fn deallocate(c_result: *mut CResult<CUnifiedReport<T, E>>) -> () {
        if c_result.is_null() {
            return ();
        }
        {
            let _ = unsafe { Box::from_raw(c_result) };
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
    fn transform<F, O1, P1, O2, P2>(c_vector_of_bytes: *mut CVector<c_uchar>, converter: F) -> *mut CResult<CUnifiedReport<O2, P2>>
    where
        F: FnOnce(UnifiedReport<O1, P1>) -> Result<CUnifiedReport<O2, P2>, Box<dyn StdError + 'static>>,
        O1: for<'a> Decode<'a>,
        P1: for<'a> Decode<'a>,
        O2: Default,
        P2: Default,
    {
        if c_vector_of_bytes.is_null() {
            return CResult::error().into_raw();
        }
        let vector_of_bytes_ = unsafe { &*c_vector_of_bytes };
        if vector_of_bytes_.pointer.is_null() || vector_of_bytes_.length == 0 {
            return CResult::error().into_raw();
        }
        let unified_report = match Serializer::deserialize::<'_, UnifiedReport<O1, P1>>(vector_of_bytes_.as_slice_unchecked()) {
            Result::Ok(unified_report_) => unified_report_,
            Result::Err(_) => {
                return CResult::error().into_raw();
            }
        };
        let c_unified_report = match converter(unified_report) {
            Result::Ok(c_unified_report_) => c_unified_report_,
            Result::Err(_) => {
                return CResult::error().into_raw();
            }
        };
        let c_result = CResult::data(c_unified_report);
        return c_result.into_raw();
    }
}
impl Transformer<ServerRequestData> {
    fn transform<I1, F, I2>(incoming: *mut I1, converter: F) -> *mut CResult<CVector<c_uchar>>
    where
        F: for<'a> FnOnce(&'a I1) -> Result<I2, Box<dyn StdError + 'static>>,
        I2: Encode,
    {
        if incoming.is_null() {
            return CResult::error().into_raw();
        }
        let incoming_ = unsafe { &*incoming };
        let incoming__ = match converter(incoming_) {
            Result::Ok(incoming___) => incoming___,
            Result::Err(_) => {
                return CResult::error().into_raw();
            }
        };
        let c_vector = Allocator::<CVector<_>>::allocate(Serializer::serialize(&incoming__));
        let c_result = CResult::data(c_vector);
        return c_result.into_raw();
    }
}
#[repr(C)]
pub struct UserAuthorization_RegisterByFirstStep_Incoming {
    pub user__email: CString,
    pub user_device__id: CString,
}
#[no_mangle]
pub extern "C" fn user_authorization__register_by_first_step__serialize_allocate(incoming: *mut UserAuthorization_RegisterByFirstStep_Incoming) -> *mut CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ UserAuthorization_RegisterByFirstStep_Incoming| -> Result<UserAuthorization_RegisterByFirstStep_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            UserAuthorization_RegisterByFirstStep_Incoming_ {
                user__email: incoming_.user__email.clone_as_string()?,
                user_device__id: incoming_.user_device__id.clone_as_string()?,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn user_authorization__register_by_first_step__serialize_deallocate(c_result: *mut CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_RegisterByFirstStep_CResult = CResult<CUnifiedReport<UserAuthorization_RegisterByFirstStep_Outcoming, UserAuthorization_RegisterByFirstStep_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_RegisterByFirstStep_Outcoming {
    pub verification_message_sent: bool,
    pub user_registration_token__can_be_resent_from: c_long,
    pub user_registration_token__wrong_enter_tries_quantity: c_short,
    pub user_registration_token__wrong_enter_tries_quantity_limit: c_short,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_RegisterByFirstStep_Precedent {
    pub user__email_already_exist: bool,
}
#[no_mangle]
pub extern "C" fn user_authorization__register_by_first_step__deserialize_allocate(c_vector_of_bytes: *mut CVector<c_uchar>) -> *mut UserAuthorization_RegisterByFirstStep_CResult {
    let converter = move |unified_report: UnifiedReport<UserAuthorization_RegisterByFirstStep_Outcoming_, UserAuthorization_RegisterByFirstStep_Precedent_>| -> Result<
        CUnifiedReport<UserAuthorization_RegisterByFirstStep_Outcoming, UserAuthorization_RegisterByFirstStep_Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let data_ = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data__,
                    } => {
                        let outcoming = UserAuthorization_RegisterByFirstStep_Outcoming {
                            verification_message_sent: data__.verification_message_sent,
                            user_registration_token__can_be_resent_from: data__.user_registration_token__can_be_resent_from,
                            user_registration_token__wrong_enter_tries_quantity: data__.user_registration_token__wrong_enter_tries_quantity,
                            user_registration_token__wrong_enter_tries_quantity_limit: data__.user_registration_token__wrong_enter_tries_quantity_limit,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                match precedent {
                    UserAuthorization_RegisterByFirstStep_Precedent_::User_EmailAlreadyExist => {}
                };
                let precedent_ = UserAuthorization_RegisterByFirstStep_Precedent {
                    user__email_already_exist: true,
                };
                CUnifiedReport::precedent(precedent_)
            }
        };
        return Result::Ok(unified_report_);
    };
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn user_authorization__register_by_first_step__deserialize_deallocate(c_result: *mut UserAuthorization_RegisterByFirstStep_CResult) -> () {
    Allocator::<CResult<CUnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
#[repr(C)]
pub struct UserAuthorization_RegisterBySecondStep_Incoming {
    pub user__email: CString,
    pub user_device__id: CString,
    pub user_registration_token__value: CString,
}
