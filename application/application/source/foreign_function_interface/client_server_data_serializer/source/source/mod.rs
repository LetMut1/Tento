use action_processor_incoming_outcoming::action_processor::{
    application_user___authorization::{
        authorize_by_first_step::{
            Incoming as ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming_,
            Outcoming as ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming_,
            Precedent as ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent_,
        },
        authorize_by_last_step::{
            Incoming as ApplicationUser__Authorization___AuthorizeByLastStep___Incoming_,
            Outcoming as ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming_,
            Precedent as ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_,
        },
        check_email_for_existing::{
            Incoming as ApplicationUser__Authorization___CheckEmailForExisting___Incoming_,
            Outcoming as ApplicationUser__Authorization___CheckEmailForExisting___Outcoming_,
        },
        check_nickname_for_existing::{
            Incoming as ApplicationUser__Authorization___CheckNicknameForExisting___Incoming_,
            Outcoming as ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming_,
        },
        deauthorize_from_all_devices::{
            Incoming as ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming_,
            Precedent as ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_,
        },
        deauthorize_from_one_device::{
            Incoming as ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming_,
            Precedent as ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_,
        },
        refresh_access_token::{
            Incoming as ApplicationUser__Authorization___RefreshAccessToken___Incoming_,
            Outcoming as ApplicationUser__Authorization___RefreshAccessToken___Outcoming_,
            Precedent as ApplicationUser__Authorization___RefreshAccessToken___Precedent_,
        },
        register_by_first_step::{
            Incoming as ApplicationUser__Authorization___RegisterByFirstStep___Incoming_,
            Outcoming as ApplicationUser__Authorization___RegisterByFirstStep___Outcoming_,
            Precedent as ApplicationUser__Authorization___RegisterByFirstStep___Precedent_,
        },
        register_by_last_step::{
            Incoming as ApplicationUser__Authorization___RegisterByLastStep___Incoming_,
            Outcoming as ApplicationUser__Authorization___RegisterByLastStep___Outcoming_,
            Precedent as ApplicationUser__Authorization___RegisterByLastStep___Precedent_,
        },
        register_by_second_step::{
            Incoming as ApplicationUser__Authorization___RegisterBySecondStep___Incoming_,
            Precedent as ApplicationUser__Authorization___RegisterBySecondStep___Precedent_,
        },
        reset_password_by_first_step::{
            Incoming as ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming_,
            Outcoming as ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming_,
            Precedent as ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_,
        },
        reset_password_by_last_step::{
            Incoming as ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming_,
            Precedent as ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_,
        },
        reset_password_by_second_step::{
            Incoming as ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming_,
            Precedent as ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_,
        },
        send_email_for_authorize::{
            Incoming as ApplicationUser__Authorization___SendEmailForAuthorize___Incoming_,
            Outcoming as ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming_,
            Precedent as ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_,
        },
        send_email_for_register::{
            Incoming as ApplicationUser__Authorization___SendEmailForRegister___Incoming_,
            Outcoming as ApplicationUser__Authorization___SendEmailForRegister___Outcoming_,
            Precedent as ApplicationUser__Authorization___SendEmailForRegister___Precedent_,
        },
        send_email_for_reset_password::{
            Incoming as ApplicationUser__Authorization___SendEmailForResetPassword___Incoming_,
            Outcoming as ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming_,
            Precedent as ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_,
        },
    },
    channel___base::{
        get_many_by_name_in_subscriptions::{
            Incoming as Channel__Base___GetManyByNameInSubscriptions___Incoming_,
            Outcoming as Channel__Base___GetManyByNameInSubscriptions___Outcoming_,
            Precedent as Channel__Base___GetManyByNameInSubscriptions___Precedent_,
        },
        get_many_by_subscription::{
            Incoming as Channel__Base___GetManyBySubscription___Incoming_,
            Outcoming as Channel__Base___GetManyBySubscription___Outcoming_,
            Precedent as Channel__Base___GetManyBySubscription___Precedent_,
        },
        get_many_public_by_name::{
            Incoming as Channel__Base___GetManyPublicByName___Incoming_,
            Outcoming as Channel__Base___GetManyPublicByName___Outcoming_,
            Precedent as Channel__Base___GetManyPublicByName___Precedent_,
        },
        get_one_by_id::{
            Incoming as Channel__Base___GetOneById___Incoming_,
            Outcoming as Channel__Base___GetOneById___Outcoming_,
            Precedent as Channel__Base___GetOneById___Precedent_,
        },
    },
    channel_subscription___base::create::{
        Incoming as ChannelSubscription__Base___Create___Incoming_,
        Precedent as ChannelSubscription__Base___Create___Precedent_,
    },
};
use application_user_access_refresh_token_encrypted::ApplicationUserAccessRefreshTokenEncrypted as ApplicationUserAccessRefreshTokenEncrypted_;
use application_user_access_token_encrypted::ApplicationUserAccessTokenEncrypted as ApplicationUserAccessTokenEncrypted_;
use libc::{
    c_char,
    c_long,
    c_short,
    c_uchar,
    size_t,
};
use message_pack_serializer::Serializer as Serializer_;
use serde::{
    Deserialize,
    Serialize,
};
use std::{
    boxed::Box,
    default::Default,
    error::Error as StdError,
    ffi::{
        CStr,
        CString,
    },
    marker::PhantomData,
    result::Result,
};
use unified_report::{
    Data,
    UnifiedReport,
};
use void::Void;
// TODO -------------------------------------------------------------------------------------------------------------------------------------------
// TODO-------------------------------------------------------------------------------------------------------------------------------------------
// TODO-------------------------------------------------------------------------------------------------------------------------------------------
// TODO-------------------------------------------------------------------------------------------------------------------------------------------
// TODO-------------------------------------------------------------------------------------------------------------------------------------------
// TODO-------------------------------------------------------------------------------------------------------------------------------------------
// TODO -------------------------------------------------------------------------------------------------------------------------------------------
// TODO rust binary ffi optimize for size.  !!!!!!!!!!!!
// https://arusahni.net/blog/2020/03/optimizing-rust-binary-size.html
// https://github.com/johnthagen/min-sized-rust !!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// TODO access_modifier/visability_modifier посмотреть, как на бэкенде лежат в бд и отдаются. Здесь сделать структуру
// TODO можно ли сериализовать Incoming не со String, а со &str для подготовки converter, чтобы избежать аллокации в стринг. На большой стренге это будет сильно замедлять.
// TODO-------------------------------------------------------------------------------------------------------------------------------------------
// TODO-------------------------------------------------------------------------------------------------------------------------------------------
// TODO-------------------------------------------------------------------------------------------------------------------------------------------
// TODO-------------------------------------------------------------------------------------------------------------------------------------------
// TODO-------------------------------------------------------------------------------------------------------------------------------------------
// TODO-------------------------------------------------------------------------------------------------------------------------------------------
// TODO-------------------------------------------------------------------------------------------------------------------------------------------
// TODO ВОзвращать во вронтенд без Еррора, то, есть сделать здесь на Анвреп
// TODO ВОзвращать во вронтенд без Еррора, то, есть сделать здесь на Анвреп
// TODO ВОзвращать во вронтенд без Еррора, то, есть сделать здесь на Анвреп
const NULL_POINTER_ERROR_MESAGE: &'static str = "There should not be a null-pointer.";
const ALLOCATION_ERROR: &'static str = "Data is not allocated.";
const DEALLOCATION_ERROR: &'static str = "Data is still allocated.";
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
    fn into_raw(self) -> *mut Self {
        return Box::into_raw(Box::new(self));
    }
}
impl<T> C_Result<T>
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
    T: Default,
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
    T: Default,
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
    P: Default,
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
    D: Default,
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
    T: Default,
{
    fn empty() -> Self {
        return C_Data {
            filled: T::default(),
            is_filled: false,
        };
    }
}
#[repr(C)]
pub struct C_String {
    pub pointer: *mut c_char,
}
impl C_String {
    fn clone_as_string<'a>(&'a self) -> Result<String, Box<dyn StdError + 'static>> {
        if self.pointer.is_null() {
            return Err(NULL_POINTER_ERROR_MESAGE.into());
        }
        let c_str = unsafe { CStr::from_ptr(self.pointer as *const _) };
        let c_string = c_str.to_str()?.to_string();
        return Ok(c_string);
    }
}
impl Default for C_String {
    fn default() -> Self {
        return Self {
            pointer: std::ptr::null_mut(),
        };
    }
}
#[repr(C)]
pub struct C_Vector<T> {
    pub pointer: *mut T,
    pub length: size_t,
}
impl<T> C_Vector<T>
where
    T: Clone,
{
    fn clone_as_vec<'a>(&'a self) -> Result<Vec<T>, Box<dyn StdError + 'static>> {
        return Ok(
            self.as_slice()?.to_vec()
        )
    }
}
impl<T> C_Vector<T> {
    fn as_slice<'a>(&'a self) -> Result<&'a [T], Box<dyn StdError + 'static>> {
        if self.pointer.is_null() {
            return Err(NULL_POINTER_ERROR_MESAGE.into());
        }
        return Ok(self.as_slice_unchecked());
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
impl<T> Default for C_Vector<T> {
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
        return C_String {
            pointer: unsafe {
                CString::from_vec_unchecked(string.into_bytes())
            }.into_raw(),
        };
    }
    fn deallocate<'a>(c_string: &'a C_String) -> () {
        if c_string.pointer.is_null() {
            return ();
        }
        {
            let _ = unsafe { CString::from_raw(c_string.pointer) };
        }
        return ();
    }
}
impl<T> Allocator<C_Vector<T>> {
    fn allocate(vector: Vec<T>) -> C_Vector<T> {
        let mut boxed_slice = vector.into_boxed_slice();
        let c_vector = C_Vector {
            pointer: boxed_slice.as_mut_ptr(),
            length: boxed_slice.len(),
        };
        std::mem::forget(boxed_slice);
        return c_vector;
    }
    fn deallocate<'a>(c_vector: &'a C_Vector<T>) -> () {
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
impl Allocator<C_Result<C_Vector<c_uchar>>> {
    fn deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
        if c_result.is_null() {
            return ();
        }
        {
            let c_result_ = unsafe { Box::from_raw(c_result) };
            if c_result_.is_data {
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data);
            }
        }
        return ();
    }
}
impl<T, E> Allocator<C_Result<C_UnifiedReport<T, E>>> {
    fn deallocate(c_result: *mut C_Result<C_UnifiedReport<T, E>>) -> () {
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
    fn transform<F, O1, P1, O2, P2>(c_vector_of_bytes: *mut C_Vector<c_uchar>, converter: F) -> *mut C_Result<C_UnifiedReport<O2, P2>>
    where
        F: FnOnce(UnifiedReport<O1, P1>) -> Result<C_UnifiedReport<O2, P2>, Box<dyn StdError + 'static>>,
        O1: for<'de> Deserialize<'de>,
        P1: for<'de> Deserialize<'de>,
        O2: Default,
        P2: Default,
    {
        if c_vector_of_bytes.is_null() {
            return C_Result::error().into_raw();
        }
        let vector_of_bytes_ = unsafe { &*c_vector_of_bytes };
        if vector_of_bytes_.pointer.is_null() || vector_of_bytes_.length == 0 {
            return C_Result::error().into_raw();
        }
        let unified_report = match Serializer_::deserialize::<'_, UnifiedReport<O1, P1>>(
            vector_of_bytes_.as_slice_unchecked(),
        ) {
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
    fn transform<I1, F, I2>(incoming: *mut I1, converter: F) -> *mut C_Result<C_Vector<c_uchar>>
    where
        F: for<'a> FnOnce(&'a I1) -> Result<I2, Box<dyn StdError + 'static>>,
        I2: Serialize,
    {
        if incoming.is_null() {
            return C_Result::error().into_raw();
        }
        let incoming_ = unsafe { &*incoming };
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
#[derive(Default)]
pub struct ApplicationUserAccessTokenEncrypted {
    pub serialized: C_Vector<c_uchar>,
    pub encoded: C_Vector<c_uchar>,
}
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUserAccessRefreshTokenEncrypted(pub C_Vector<c_uchar>);
#[repr(C)]
#[derive(Default)]
pub struct Common1 {
    pub channel: Channel1,
    pub is_application_user_subscribed: bool,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel1 {
    pub channel__id: c_long,
    pub channel__name: C_String,
    pub channel__linked_name: C_String,
    pub channel__access_modifier: c_short,
    pub channel__visability_modifier: c_short,
    pub channel__cover_image_path: C_Option<C_String>,
    pub channel__background_image_path: C_Option<C_String>,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel2 {
    pub channel__owner: c_long,
    pub channel__name: C_String,
    pub channel__linked_name: C_String,
    pub channel__description: C_Option<C_String>,
    pub channel__access_modifier: c_short,
    pub channel__visability_modifier: c_short,
    pub channel__orientation: C_Vector<c_short>,
    pub channel__cover_image_path: C_Option<C_String>,
    pub channel__background_image_path: C_Option<C_String>,
    pub channel__subscribers_quantity: c_long,
    pub channel__marks_quantity: c_long,
    pub channel__viewing_quantity: c_long,
}
#[repr(C)]
#[derive(Default)]
pub struct ChannelInnerLink1 {
    pub channel_inner_link__to: c_long,
}
#[repr(C)]
#[derive(Default)]
pub struct ChannelOuterLink1 {
    pub channel_outer_link__alias: C_String,
    pub channel_outer_link__address: C_String,
}
#[repr(C)]
pub struct ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming {
    pub application_user_device__id: C_String,
    pub application_user__email___or___application_user__nickname: C_String,
    pub application_user__password: C_String,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_first_step____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming| -> Result<ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming_ {
            application_user_device__id: incoming.application_user_device__id.clone_as_string()?,
            application_user__email___or___application_user__nickname: incoming.application_user__email___or___application_user__nickname.clone_as_string()?,
            application_user__password: incoming.application_user__password.clone_as_string()?,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_first_step____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result =
    C_Result<C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming, ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming {
    pub application_user__id: c_long,
    pub verification_message_sent: bool,
    pub application_user_authorization_token__can_be_resent_from: c_long,
    pub application_user_authorization_token__wrong_enter_tries_quantity: c_short,
    pub application_user_authorization_token__wrong_enter_tries_quantity_limit: c_short,
}
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent {
    pub application_user__wrong_email_or_nickname_or_password: bool,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_first_step____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result {
    let converter = move |unified_report: UnifiedReport<
        ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming_,
        ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent_,
    >|
          -> Result<
        C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming, ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let data_ = match data {
                    Data::Empty => C_Data::empty(),
                    Data::Filled {
                        data: data__,
                    } => {
                        let outcoming = ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming {
                            application_user__id: data__.application_user__id,
                            verification_message_sent: data__.verification_message_sent,
                            application_user_authorization_token__can_be_resent_from: data__.application_user_authorization_token__can_be_resent_from,
                            application_user_authorization_token__wrong_enter_tries_quantity: data__.application_user_authorization_token__wrong_enter_tries_quantity,
                            application_user_authorization_token__wrong_enter_tries_quantity_limit: data__.application_user_authorization_token__wrong_enter_tries_quantity_limit,
                        };
                        C_Data::filled(outcoming)
                    }
                };
                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_first_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result,
) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___AuthorizeByLastStep___Incoming {
    pub application_user__id: c_long,
    pub application_user_device__id: C_String,
    pub application_user_authorization_token__value: C_String,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_last_step____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___AuthorizeByLastStep___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___AuthorizeByLastStep___Incoming| -> Result<ApplicationUser__Authorization___AuthorizeByLastStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___AuthorizeByLastStep___Incoming_ {
            application_user__id: incoming.application_user__id,
            application_user_device__id: incoming.application_user_device__id.clone_as_string()?,
            application_user_authorization_token__value: incoming.application_user_authorization_token__value.clone_as_string()?,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_last_step____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___AuthorizeByLastStep___C_Result =
    C_Result<C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming, ApplicationUser__Authorization___AuthorizeByLastStep___Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted,
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
    pub application_user_authorization_token__wrong_enter_tries_quantity: c_short,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_last_step____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result {
    let converter = move |unified_report: UnifiedReport<
        ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming_,
        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_,
    >|
          -> Result<
        C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming, ApplicationUser__Authorization___AuthorizeByLastStep___Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let data_ = match data {
                    Data::Empty => C_Data::empty(),
                    Data::Filled {
                        data: data__,
                    } => {
                        let outcoming = ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming {
                            application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted {
                                serialized: Allocator::<C_Vector<_>>::allocate(data__.application_user_access_token_encrypted.serialized),
                                encoded: Allocator::<C_Vector<_>>::allocate(data__.application_user_access_token_encrypted.encoded),
                            },
                            application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted(
                                Allocator::<C_Vector<_>>::allocate(data__.application_user_access_refresh_token_encrypted.0),
                            ),
                        };
                        C_Data::filled(outcoming)
                    }
                };
                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
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
                    ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_::ApplicationUserAuthorizationToken_WrongValue {
                        application_user_authorization_token__wrong_enter_tries_quantity,
                    } => {
                        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent {
                            application_user_authorization_token__wrong_value: ApplicationUserAuthorizationToken_WrongValue {
                                is_exist: true,
                                application_user_authorization_token__wrong_enter_tries_quantity,
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____authorize_by_last_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result,
) -> () {
    if c_result.is_null() {
        return ();
    }
    let c_result_ = unsafe { Box::from_raw(c_result) };
    if c_result_.is_data {
        if c_result_.data.is_target {
            if c_result_.data.target.is_filled {
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.application_user_access_token_encrypted.serialized);
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.application_user_access_token_encrypted.encoded);
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.application_user_access_refresh_token_encrypted.0);
            }
        }
    }
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___CheckEmailForExisting___Incoming {
    pub application_user__email: C_String,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____check_email_for_existing____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___CheckEmailForExisting___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___CheckEmailForExisting___Incoming| -> Result<ApplicationUser__Authorization___CheckEmailForExisting___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___CheckEmailForExisting___Incoming_ {
            application_user__email: incoming.application_user__email.clone_as_string()?,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____check_email_for_existing____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___CheckEmailForExisting___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___CheckEmailForExisting___Outcoming, C_Void>>;
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___CheckEmailForExisting___Outcoming {
    pub result: bool,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____check_email_for_existing____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____check_email_for_existing____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___CheckEmailForExisting___C_Result,
) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___CheckNicknameForExisting___Incoming {
    pub application_user__nickname: C_String,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____check_nickname_for_existing____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___CheckNicknameForExisting___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___CheckNicknameForExisting___Incoming| -> Result<ApplicationUser__Authorization___CheckNicknameForExisting___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___CheckNicknameForExisting___Incoming_ {
            application_user__nickname: incoming.application_user__nickname.clone_as_string()?,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____check_nickname_for_existing____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___CheckNicknameForExisting___C_Result =
    C_Result<C_UnifiedReport<ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming, C_Void>>;
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming {
    pub result: bool,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____check_nickname_for_existing____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____check_nickname_for_existing____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___CheckNicknameForExisting___C_Result,
) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_all_devices____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming| -> Result<ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming_ {
            application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted_ {
                serialized: incoming.application_user_access_token_encrypted.serialized.clone_as_vec()?,
                encoded: incoming.application_user_access_token_encrypted.encoded.clone_as_vec()?,
            }
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_all_devices____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result =
    C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_all_devices____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_all_devices____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result,
) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result =
    C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
}
#[repr(C)]
pub struct ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_one_device____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming| -> Result<ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming_ {
            application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted_ {
                serialized: incoming.application_user_access_token_encrypted.serialized.clone_as_vec()?,
                encoded: incoming.application_user_access_token_encrypted.encoded.clone_as_vec()?,
            }
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_one_device____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_one_device____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____deauthorize_from_one_device____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result,
) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___RefreshAccessToken___Incoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____refresh_access_token____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___RefreshAccessToken___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___RefreshAccessToken___Incoming| -> Result<ApplicationUser__Authorization___RefreshAccessToken___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___RefreshAccessToken___Incoming_ {
            application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted_ {
                serialized: incoming.application_user_access_token_encrypted.serialized.clone_as_vec()?,
                encoded: incoming.application_user_access_token_encrypted.encoded.clone_as_vec()?,
            },
            application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted_(
                incoming.application_user_access_refresh_token_encrypted.0.clone_as_vec()?,
            ),
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____refresh_access_token____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___RefreshAccessToken___C_Result =
    C_Result<C_UnifiedReport<ApplicationUser__Authorization___RefreshAccessToken___Outcoming, ApplicationUser__Authorization___RefreshAccessToken___Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___RefreshAccessToken___Outcoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted,
}
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___RefreshAccessToken___Precedent {
    pub application_user_access_refresh_token__not_found: bool,
    pub application_user_access_refresh_token__already_expired: bool,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____refresh_access_token____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result {
    let converter = move |unified_report: UnifiedReport<
        ApplicationUser__Authorization___RefreshAccessToken___Outcoming_,
        ApplicationUser__Authorization___RefreshAccessToken___Precedent_,
    >|
          -> Result<
        C_UnifiedReport<ApplicationUser__Authorization___RefreshAccessToken___Outcoming, ApplicationUser__Authorization___RefreshAccessToken___Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let data_ = match data {
                    Data::Empty => C_Data::empty(),
                    Data::Filled {
                        data: data__,
                    } => {
                        let outcoming = ApplicationUser__Authorization___RefreshAccessToken___Outcoming {
                            application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted {
                                serialized: Allocator::<C_Vector<_>>::allocate(data__.application_user_access_token_encrypted.serialized),
                                encoded: Allocator::<C_Vector<_>>::allocate(data__.application_user_access_token_encrypted.encoded),
                            },
                            application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted(
                                Allocator::<C_Vector<_>>::allocate(data__.application_user_access_refresh_token_encrypted.0),
                            ),
                        };
                        C_Data::filled(outcoming)
                    }
                };
                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____refresh_access_token____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result,
) -> () {
    if c_result.is_null() {
        return ();
    }
    let c_result_ = unsafe { Box::from_raw(c_result) };
    if c_result_.is_data {
        if c_result_.data.is_target {
            if c_result_.data.target.is_filled {
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.application_user_access_token_encrypted.serialized);
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.application_user_access_token_encrypted.encoded);
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.application_user_access_refresh_token_encrypted.0);
            }
        }
    }
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___RegisterByFirstStep___Incoming {
    pub application_user__email: C_String,
    pub application_user_device__id: C_String,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_first_step____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___RegisterByFirstStep___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___RegisterByFirstStep___Incoming| -> Result<ApplicationUser__Authorization___RegisterByFirstStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___RegisterByFirstStep___Incoming_ {
            application_user__email: incoming.application_user__email.clone_as_string()?,
            application_user_device__id: incoming.application_user_device__id.clone_as_string()?,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_first_step____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___RegisterByFirstStep___C_Result =
    C_Result<C_UnifiedReport<ApplicationUser__Authorization___RegisterByFirstStep___Outcoming, ApplicationUser__Authorization___RegisterByFirstStep___Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___RegisterByFirstStep___Outcoming {
    pub verification_message_sent: bool,
    pub application_user_registration_token__can_be_resent_from: c_long,
    pub application_user_registration_token__wrong_enter_tries_quantity: c_short,
    pub application_user_registration_token__wrong_enter_tries_quantity_limit: c_short,
}
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___RegisterByFirstStep___Precedent {
    pub application_user__email_already_exist: bool,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_first_step____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result {
    let converter = move |unified_report: UnifiedReport<
        ApplicationUser__Authorization___RegisterByFirstStep___Outcoming_,
        ApplicationUser__Authorization___RegisterByFirstStep___Precedent_,
    >|
          -> Result<
        C_UnifiedReport<ApplicationUser__Authorization___RegisterByFirstStep___Outcoming, ApplicationUser__Authorization___RegisterByFirstStep___Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let data_ = match data {
                    Data::Empty => C_Data::empty(),
                    Data::Filled {
                        data: data__,
                    } => {
                        let outcoming = ApplicationUser__Authorization___RegisterByFirstStep___Outcoming {
                            verification_message_sent: data__.verification_message_sent,
                            application_user_registration_token__can_be_resent_from: data__.application_user_registration_token__can_be_resent_from,
                            application_user_registration_token__wrong_enter_tries_quantity: data__.application_user_registration_token__wrong_enter_tries_quantity,
                            application_user_registration_token__wrong_enter_tries_quantity_limit: data__.application_user_registration_token__wrong_enter_tries_quantity_limit,
                        };
                        C_Data::filled(outcoming)
                    }
                };
                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_first_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result,
) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___RegisterBySecondStep___Incoming {
    pub application_user__email: C_String,
    pub application_user_device__id: C_String,
    pub application_user_registration_token__value: C_String,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_second_step____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___RegisterBySecondStep___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___RegisterBySecondStep___Incoming| -> Result<ApplicationUser__Authorization___RegisterBySecondStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___RegisterBySecondStep___Incoming_ {
            application_user__email: incoming.application_user__email.clone_as_string()?,
            application_user_device__id: incoming.application_user_device__id.clone_as_string()?,
            application_user_registration_token__value: incoming.application_user_registration_token__value.clone_as_string()?,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_second_step____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___RegisterBySecondStep___C_Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___RegisterBySecondStep___Precedent>>;
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
    pub application_user_registration_token__wrong_enter_tries_quantity: c_short,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_second_step____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
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
                    ApplicationUser__Authorization___RegisterBySecondStep___Precedent_::ApplicationUserRegistrationToken_WrongValue { application_user_registration_token__wrong_enter_tries_quantity } => {
                        ApplicationUser__Authorization___RegisterBySecondStep___Precedent {
                            application_user_registration_token__wrong_value: ApplicationUserRegistrationToken_WrongValue {
                                is_exist: true,
                                application_user_registration_token__wrong_enter_tries_quantity: application_user_registration_token__wrong_enter_tries_quantity,
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_second_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___RegisterBySecondStep___C_Result,
) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___RegisterByLastStep___Incoming {
    pub application_user_device__id: C_String,
    pub application_user__nickname: C_String,
    pub application_user__password: C_String,
    pub application_user__email: C_String,
    pub application_user_registration_token__value: C_String,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_last_step____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___RegisterByLastStep___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___RegisterByLastStep___Incoming| -> Result<ApplicationUser__Authorization___RegisterByLastStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___RegisterByLastStep___Incoming_ {
            application_user_device__id: incoming.application_user_device__id.clone_as_string()?,
            application_user__email: incoming.application_user__email.clone_as_string()?,
            application_user__nickname: incoming.application_user__nickname.clone_as_string()?,
            application_user__password: incoming.application_user__password.clone_as_string()?,
            application_user_registration_token__value: incoming.application_user_registration_token__value.clone_as_string()?,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_last_step____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___RegisterByLastStep___C_Result =
    C_Result<C_UnifiedReport<ApplicationUser__Authorization___RegisterByLastStep___Outcoming, ApplicationUser__Authorization___RegisterByLastStep___Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___RegisterByLastStep___Outcoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted,
}
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___RegisterByLastStep___Precedent {
    pub application_user__nickname_already_exist: bool,
    pub application_user__email_already_exist: bool,
    pub application_user_registration_token__not_found: bool,
    pub application_user_registration_token__already_expired: bool,
    pub application_user_registration_token__is_not_approved: bool,
    pub application_user_registration_token__wrong_value: bool,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_last_step____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result {
    let converter = move |unified_report: UnifiedReport<
        ApplicationUser__Authorization___RegisterByLastStep___Outcoming_,
        ApplicationUser__Authorization___RegisterByLastStep___Precedent_,
    >|
          -> Result<
        C_UnifiedReport<ApplicationUser__Authorization___RegisterByLastStep___Outcoming, ApplicationUser__Authorization___RegisterByLastStep___Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let data_ = match data {
                    Data::Empty => C_Data::empty(),
                    Data::Filled {
                        data: data__,
                    } => {
                        let outcoming = ApplicationUser__Authorization___RegisterByLastStep___Outcoming {
                            application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted {
                                serialized: Allocator::<C_Vector<_>>::allocate(data__.application_user_access_token_encrypted.serialized),
                                encoded: Allocator::<C_Vector<_>>::allocate(data__.application_user_access_token_encrypted.encoded),
                            },
                            application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted(
                                Allocator::<C_Vector<_>>::allocate(data__.application_user_access_refresh_token_encrypted.0),
                            ),
                        };
                        C_Data::filled(outcoming)
                    }
                };
                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____register_by_last_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result,
) -> () {
    if c_result.is_null() {
        return ();
    }
    let c_result_ = unsafe { Box::from_raw(c_result) };
    if c_result_.is_data {
        if c_result_.data.is_target {
            if c_result_.data.target.is_filled {
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.application_user_access_token_encrypted.serialized);
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.application_user_access_token_encrypted.encoded);
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.application_user_access_refresh_token_encrypted.0);
            }
        }
    }
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming {
    pub application_user__email: C_String,
    pub application_user_device__id: C_String,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_first_step____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming| -> Result<ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming_ {
            application_user__email: incoming.application_user__email.clone_as_string()?,
            application_user_device__id: incoming.application_user_device__id.clone_as_string()?,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_first_step____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result =
    C_Result<C_UnifiedReport<ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming, ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming {
    pub application_user__id: c_long,
    pub verification_message_sent: bool,
    pub application_user_reset_password_token__can_be_resent_from: c_long,
    pub application_user_reset_password_token__wrong_enter_tries_quantity: c_short,
    pub application_user_reset_password_token__wrong_enter_tries_quantity_limit: c_short,
}
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent {
    pub application_user__not_found: bool,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_first_step____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result {
    let converter = move |unified_report: UnifiedReport<
        ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming_,
        ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_,
    >|
          -> Result<
        C_UnifiedReport<ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming, ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let data_ = match data {
                    Data::Empty => C_Data::empty(),
                    Data::Filled {
                        data: data__,
                    } => {
                        let outcoming = ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming {
                            application_user__id: data__.application_user__id,
                            verification_message_sent: data__.verification_message_sent,
                            application_user_reset_password_token__can_be_resent_from: data__.application_user_reset_password_token__can_be_resent_from,
                            application_user_reset_password_token__wrong_enter_tries_quantity: data__.application_user_reset_password_token__wrong_enter_tries_quantity,
                            application_user_reset_password_token__wrong_enter_tries_quantity_limit: data__.application_user_reset_password_token__wrong_enter_tries_quantity_limit,
                        };
                        C_Data::filled(outcoming)
                    }
                };
                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                match precedent {
                    ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_::ApplicationUser_NotFound => {}
                };
                let precedent_ = ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent {
                    application_user__not_found: true,
                };
                C_UnifiedReport::precedent(precedent_)
            }
        };
        return Ok(unified_report_);
    };
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_first_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result,
) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming {
    pub application_user__id: c_long,
    pub application_user_device__id: C_String,
    pub application_user_reset_password_token__value: C_String,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_second_step____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming| -> Result<ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming_ {
            application_user__id: incoming.application_user__id,
            application_user_device__id: incoming.application_user_device__id.clone_as_string()?,
            application_user_reset_password_token__value: incoming.application_user_reset_password_token__value.clone_as_string()?,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_second_step____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result =
    C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent>>;
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
    pub application_user_reset_password_token__wrong_enter_tries_quantity: c_short,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_second_step____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
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
                    ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_::ApplicationUserResetPasswordToken_WrongValue { application_user_reset_password_token__wrong_enter_tries_quantity } => {
                        ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent {
                            application_user_reset_password_token__wrong_value: ApplicationUserResetPasswordToken_WrongValue {
                                is_exist: true,
                                application_user_reset_password_token__wrong_enter_tries_quantity: application_user_reset_password_token__wrong_enter_tries_quantity,
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_second_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result,
) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming {
    pub application_user__id: c_long,
    pub application_user_device__id: C_String,
    pub application_user__password: C_String,
    pub application_user_reset_password_token__value: C_String,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_last_step____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming| -> Result<ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming_ {
            application_user__id: incoming.application_user__id,
            application_user_device__id: incoming.application_user_device__id.clone_as_string()?,
            application_user__password: incoming.application_user__password.clone_as_string()?,
            application_user_reset_password_token__value: incoming.application_user_reset_password_token__value.clone_as_string()?,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_last_step____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent {
    pub application_user__not_found: bool,
    pub application_user_reset_password_token__not_found: bool,
    pub application_user_reset_password_token__already_expired: bool,
    pub application_user_reset_password_token__is_not_approved: bool,
    pub application_user_reset_password_token__wrong_value: bool,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_last_step____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____reset_password_by_last_step____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result,
) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___SendEmailForRegister___Incoming {
    pub application_user__email: C_String,
    pub application_user_device__id: C_String,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_register____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___SendEmailForRegister___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___SendEmailForRegister___Incoming| -> Result<ApplicationUser__Authorization___SendEmailForRegister___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___SendEmailForRegister___Incoming_ {
            application_user__email: incoming.application_user__email.clone_as_string()?,
            application_user_device__id: incoming.application_user_device__id.clone_as_string()?,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_register____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___SendEmailForRegister___C_Result =
    C_Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForRegister___Outcoming, ApplicationUser__Authorization___SendEmailForRegister___Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___SendEmailForRegister___Outcoming {
    pub application_user_registration_token__can_be_resent_from: c_long,
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
pub extern "C" fn application_user___authorization____send_email_for_register____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result {
    let converter = move |unified_report: UnifiedReport<
        ApplicationUser__Authorization___SendEmailForRegister___Outcoming_,
        ApplicationUser__Authorization___SendEmailForRegister___Precedent_,
    >|
          -> Result<
        C_UnifiedReport<ApplicationUser__Authorization___SendEmailForRegister___Outcoming, ApplicationUser__Authorization___SendEmailForRegister___Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let data_ = match data {
                    Data::Empty => C_Data::empty(),
                    Data::Filled {
                        data: data__,
                    } => {
                        let outcoming = ApplicationUser__Authorization___SendEmailForRegister___Outcoming {
                            application_user_registration_token__can_be_resent_from: data__.application_user_registration_token__can_be_resent_from,
                        };
                        C_Data::filled(outcoming)
                    }
                };
                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_register____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result,
) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___SendEmailForAuthorize___Incoming {
    pub application_user_device__id: C_String,
    pub application_user__id: c_long,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_authorize____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___SendEmailForAuthorize___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___SendEmailForAuthorize___Incoming| -> Result<ApplicationUser__Authorization___SendEmailForAuthorize___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___SendEmailForAuthorize___Incoming_ {
            application_user_device__id: incoming.application_user_device__id.clone_as_string()?,
            application_user__id: incoming.application_user__id,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_authorize____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___SendEmailForAuthorize___C_Result =
    C_Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming, ApplicationUser__Authorization___SendEmailForAuthorize___Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming {
    pub application_user_authorization_token__can_be_resent_from: c_long,
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
pub extern "C" fn application_user___authorization____send_email_for_authorize____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result {
    let converter = move |unified_report: UnifiedReport<
        ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming_,
        ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_,
    >|
          -> Result<
        C_UnifiedReport<ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming, ApplicationUser__Authorization___SendEmailForAuthorize___Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let data_ = match data {
                    Data::Empty => C_Data::empty(),
                    Data::Filled {
                        data: data__,
                    } => {
                        let outcoming = ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming {
                            application_user_authorization_token__can_be_resent_from: data__.application_user_authorization_token__can_be_resent_from,
                        };
                        C_Data::filled(outcoming)
                    }
                };
                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_authorize____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result,
) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
#[repr(C)]
pub struct ApplicationUser__Authorization___SendEmailForResetPassword___Incoming {
    pub application_user__id: c_long,
    pub application_user_device__id: C_String,
}
#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_reset_password____serialize____allocate(
    incoming: *mut ApplicationUser__Authorization___SendEmailForResetPassword___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ApplicationUser__Authorization___SendEmailForResetPassword___Incoming| -> Result<ApplicationUser__Authorization___SendEmailForResetPassword___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ApplicationUser__Authorization___SendEmailForResetPassword___Incoming_ {
            application_user__id: incoming.application_user__id,
            application_user_device__id: incoming.application_user_device__id.clone_as_string()?,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_reset_password____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ApplicationUser__Authorization___SendEmailForResetPassword___C_Result =
    C_Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming, ApplicationUser__Authorization___SendEmailForResetPassword___Precedent>>;
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
pub extern "C" fn application_user___authorization____send_email_for_reset_password____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result {
    let converter = move |unified_report: UnifiedReport<
        ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming_,
        ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_,
    >|
          -> Result<
        C_UnifiedReport<ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming, ApplicationUser__Authorization___SendEmailForResetPassword___Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let data_ = match data {
                    Data::Empty => C_Data::empty(),
                    Data::Filled {
                        data: data__,
                    } => {
                        let outcoming = ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming {
                            application_user_resep_password_token_can_be_resent_from: data__.application_user_reset_password_token__can_be_resent_from,
                        };
                        C_Data::filled(outcoming)
                    }
                };
                C_UnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn application_user___authorization____send_email_for_reset_password____deserialize____deallocate(
    c_result: *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result,
) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
#[repr(C)]
pub struct Channel__Base___GetManyByNameInSubscriptions___Incoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub channel__name: C_String,
    pub requery___channel__name: C_Option<C_String>,
    pub limit: c_short,
}
#[no_mangle]
pub extern "C" fn channel___base____get_many_by_name_in_subscriptions____serialize____allocate(
    incoming: *mut Channel__Base___GetManyByNameInSubscriptions___Incoming,
) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter =
        move |incoming: &'_ Channel__Base___GetManyByNameInSubscriptions___Incoming| -> Result<Channel__Base___GetManyByNameInSubscriptions___Incoming_, Box<dyn StdError + 'static>> {
            let requery___channel__name = if incoming.requery___channel__name.is_data {
                Some(incoming.requery___channel__name.data.clone_as_string()?)
            } else {
                None
            };
            let incoming_ = Channel__Base___GetManyByNameInSubscriptions___Incoming_ {
                application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted_ {
                    serialized: incoming.application_user_access_token_encrypted.serialized.clone_as_vec()?,
                    encoded: incoming.application_user_access_token_encrypted.encoded.clone_as_vec()?,
                },
                channel__name: incoming.channel__name.clone_as_string()?,
                requery___channel__name,
                limit: incoming.limit,
            };
            return Ok(incoming_);
        };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn channel___base____get_many_by_name_in_subscriptions____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type Channel__Base___GetManyByNameInSubscriptions___C_Result =
    C_Result<C_UnifiedReport<Channel__Base___GetManyByNameInSubscriptions___Outcoming, Channel__Base___GetManyByNameInSubscriptions___Precedent>>;
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
pub extern "C" fn channel___base____get_many_by_name_in_subscriptions____deserialize____allocate(
    c_vector_of_bytes: *mut C_Vector<c_uchar>,
) -> *mut Channel__Base___GetManyByNameInSubscriptions___C_Result {
    let converter = move |unified_report: UnifiedReport<Channel__Base___GetManyByNameInSubscriptions___Outcoming_, Channel__Base___GetManyByNameInSubscriptions___Precedent_>| -> Result<
        C_UnifiedReport<Channel__Base___GetManyByNameInSubscriptions___Outcoming, Channel__Base___GetManyByNameInSubscriptions___Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let data_ = match data {
                    Data::Empty => C_Data::empty(),
                    Data::Filled {
                        data: data__,
                    } => {
                        let mut common_registry: Vec<Common1> = vec![];
                        '_a: for common_1 in data__.common_registry {
                            let channel__cover_image_path = match common_1.channel.channel__cover_image_path {
                                Some(channel__cover_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel__cover_image_path_)),
                                None => C_Option::none(),
                            };
                            let channel__background_image_path = match common_1.channel.channel__background_image_path {
                                Some(channel__background_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel__background_image_path_)),
                                None => C_Option::none(),
                            };
                            let common_1_ = Common1 {
                                channel: Channel1 {
                                    channel__id: common_1.channel.channel__id,
                                    channel__name: Allocator::<C_String>::allocate(common_1.channel.channel__name),
                                    channel__linked_name: Allocator::<C_String>::allocate(common_1.channel.channel__linked_name),
                                    channel__access_modifier: common_1.channel.channel__access_modifier,
                                    channel__visability_modifier: common_1.channel.channel__visability_modifier,
                                    channel__cover_image_path,
                                    channel__background_image_path,
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
            UnifiedReport::Precedent {
                precedent,
            } => {
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn channel___base____get_many_by_name_in_subscriptions____deserialize____deallocate(c_result: *mut Channel__Base___GetManyByNameInSubscriptions___C_Result) -> () {
    if c_result.is_null() {
        return ();
    }
    let c_result_ = unsafe { Box::from_raw(c_result) };
    if c_result_.is_data {
        if c_result_.data.is_target {
            if c_result_.data.target.is_filled {
                let common_registry = c_result_.data.target.filled.common_registry.as_slice_unchecked();
                for common in common_registry {
                    Allocator::<C_String>::deallocate(&common.channel.channel__name);
                    Allocator::<C_String>::deallocate(&common.channel.channel__linked_name);
                    if common.channel.channel__background_image_path.is_data {
                        Allocator::<C_String>::deallocate(&common.channel.channel__background_image_path.data);
                    }
                    if common.channel.channel__cover_image_path.is_data {
                        Allocator::<C_String>::deallocate(&common.channel.channel__cover_image_path.data);
                    }
                }
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.common_registry);
            }
        }
    }
    return ();
}
#[repr(C)]
pub struct Channel__Base___GetManyBySubscription___Incoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub requery___channel__id: C_Option<c_long>,
    pub limit: c_short,
}
#[no_mangle]
pub extern "C" fn channel___base____get_many_by_subscription____serialize____allocate(incoming: *mut Channel__Base___GetManyBySubscription___Incoming) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ Channel__Base___GetManyBySubscription___Incoming| -> Result<Channel__Base___GetManyBySubscription___Incoming_, Box<dyn StdError + 'static>> {
        let requery___channel__id = if incoming.requery___channel__id.is_data {
            Some(incoming.requery___channel__id.data)
        } else {
            None
        };
        let incoming_ = Channel__Base___GetManyBySubscription___Incoming_ {
            application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted_ {
                serialized: incoming.application_user_access_token_encrypted.serialized.clone_as_vec()?,
                encoded: incoming.application_user_access_token_encrypted.encoded.clone_as_vec()?,
            },
            requery___channel__id,
            limit: incoming.limit,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn channel___base____get_many_by_subscription____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type Channel__Base___GetManyBySubscription___C_Result =
    C_Result<C_UnifiedReport<Channel__Base___GetManyBySubscription___Outcoming, Channel__Base___GetManyBySubscription___Precedent>>;
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
pub extern "C" fn channel___base____get_many_by_subscription____deserialize____allocate(c_vector_of_bytes: *mut C_Vector<c_uchar>) -> *mut Channel__Base___GetManyBySubscription___C_Result {
    let converter = move |unified_report: UnifiedReport<Channel__Base___GetManyBySubscription___Outcoming_, Channel__Base___GetManyBySubscription___Precedent_>| -> Result<
        C_UnifiedReport<Channel__Base___GetManyBySubscription___Outcoming, Channel__Base___GetManyBySubscription___Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let data_ = match data {
                    Data::Empty => C_Data::empty(),
                    Data::Filled {
                        data: data__,
                    } => {
                        let mut common_registry: Vec<Common1> = vec![];
                        '_a: for common_1 in data__.common_registry {
                            let channel__cover_image_path = match common_1.channel.channel__cover_image_path {
                                Some(channel__cover_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel__cover_image_path_)),
                                None => C_Option::none(),
                            };
                            let channel__background_image_path = match common_1.channel.channel__background_image_path {
                                Some(channel__background_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel__background_image_path_)),
                                None => C_Option::none(),
                            };
                            let common_1_ = Common1 {
                                channel: Channel1 {
                                    channel__id: common_1.channel.channel__id,
                                    channel__name: Allocator::<C_String>::allocate(common_1.channel.channel__name),
                                    channel__linked_name: Allocator::<C_String>::allocate(common_1.channel.channel__linked_name),
                                    channel__access_modifier: common_1.channel.channel__access_modifier,
                                    channel__visability_modifier: common_1.channel.channel__visability_modifier,
                                    channel__cover_image_path,
                                    channel__background_image_path,
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
            UnifiedReport::Precedent {
                precedent,
            } => {
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn channel___base____get_many_by_subscription____deserialize____deallocate(c_result: *mut Channel__Base___GetManyBySubscription___C_Result) -> () {
    if c_result.is_null() {
        return ();
    }
    let c_result_ = unsafe { Box::from_raw(c_result) };
    if c_result_.is_data {
        if c_result_.data.is_target {
            if c_result_.data.target.is_filled {
                let common_registry = c_result_.data.target.filled.common_registry.as_slice_unchecked();
                for common in common_registry {
                    Allocator::<C_String>::deallocate(&common.channel.channel__name);
                    Allocator::<C_String>::deallocate(&common.channel.channel__linked_name);
                    if common.channel.channel__background_image_path.is_data {
                        Allocator::<C_String>::deallocate(&common.channel.channel__background_image_path.data);
                    }
                    if common.channel.channel__cover_image_path.is_data {
                        Allocator::<C_String>::deallocate(&common.channel.channel__cover_image_path.data);
                    }
                }
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.common_registry);
            }
        }
    }
    return ();
}
#[repr(C)]
pub struct Channel__Base___GetManyPublicByName___Incoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub channel__name: C_String,
    pub requery___channel__name: C_Option<C_String>,
    pub limit: c_short,
}
#[no_mangle]
pub extern "C" fn channel___base____get_many_public_by_name____serialize____allocate(incoming: *mut Channel__Base___GetManyPublicByName___Incoming) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ Channel__Base___GetManyPublicByName___Incoming| -> Result<Channel__Base___GetManyPublicByName___Incoming_, Box<dyn StdError + 'static>> {
        let requery___channel__name = if incoming.requery___channel__name.is_data {
            Some(incoming.requery___channel__name.data.clone_as_string()?)
        } else {
            None
        };
        let incoming_ = Channel__Base___GetManyPublicByName___Incoming_ {
            application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted_ {
                serialized: incoming.application_user_access_token_encrypted.serialized.clone_as_vec()?,
                encoded: incoming.application_user_access_token_encrypted.encoded.clone_as_vec()?,
            },
            channel__name: incoming.channel__name.clone_as_string()?,
            requery___channel__name,
            limit: incoming.limit,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn channel___base____get_many_public_by_name____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type Channel__Base___GetManyPublicByName___C_Result = C_Result<C_UnifiedReport<Channel__Base___GetManyPublicByName___Outcoming, Channel__Base___GetManyPublicByName___Precedent>>;
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
pub extern "C" fn channel___base____get_many_public_by_name____deserialize____allocate(c_vector_of_bytes: *mut C_Vector<c_uchar>) -> *mut Channel__Base___GetManyPublicByName___C_Result {
    let converter = move |unified_report: UnifiedReport<Channel__Base___GetManyPublicByName___Outcoming_, Channel__Base___GetManyPublicByName___Precedent_>| -> Result<
        C_UnifiedReport<Channel__Base___GetManyPublicByName___Outcoming, Channel__Base___GetManyPublicByName___Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let data_ = match data {
                    Data::Empty => C_Data::empty(),
                    Data::Filled {
                        data: data__,
                    } => {
                        let mut common_registry: Vec<Common1> = vec![];
                        '_a: for common_1 in data__.common_registry {
                            let channel__cover_image_path = match common_1.channel.channel__cover_image_path {
                                Some(channel__cover_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel__cover_image_path_)),
                                None => C_Option::none(),
                            };
                            let channel__background_image_path = match common_1.channel.channel__background_image_path {
                                Some(channel__background_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel__background_image_path_)),
                                None => C_Option::none(),
                            };
                            let common_1_ = Common1 {
                                channel: Channel1 {
                                    channel__id: common_1.channel.channel__id,
                                    channel__name: Allocator::<C_String>::allocate(common_1.channel.channel__name),
                                    channel__linked_name: Allocator::<C_String>::allocate(common_1.channel.channel__linked_name),
                                    channel__access_modifier: common_1.channel.channel__access_modifier,
                                    channel__visability_modifier: common_1.channel.channel__visability_modifier,
                                    channel__cover_image_path,
                                    channel__background_image_path,
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
            UnifiedReport::Precedent {
                precedent,
            } => {
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn channel___base____get_many_public_by_name____deserialize____deallocate(c_result: *mut Channel__Base___GetManyPublicByName___C_Result) -> () {
    if c_result.is_null() {
        return ();
    }
    let c_result_ = unsafe { Box::from_raw(c_result) };
    if c_result_.is_data {
        if c_result_.data.is_target {
            if c_result_.data.target.is_filled {
                let common_registry = c_result_.data.target.filled.common_registry.as_slice_unchecked();
                for common_1 in common_registry {
                    Allocator::<C_String>::deallocate(&common_1.channel.channel__name);
                    Allocator::<C_String>::deallocate(&common_1.channel.channel__linked_name);
                    if common_1.channel.channel__background_image_path.is_data {
                        Allocator::<C_String>::deallocate(&common_1.channel.channel__background_image_path.data);
                    }
                    if common_1.channel.channel__cover_image_path.is_data {
                        Allocator::<C_String>::deallocate(&common_1.channel.channel__cover_image_path.data);
                    }
                }
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.common_registry);
            }
        }
    }
    return ();
}
#[repr(C)]
pub struct Channel__Base___GetOneById___Incoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub channel__id: c_long,
}
#[no_mangle]
pub extern "C" fn channel___base____get_one_by_id____serialize____allocate(incoming: *mut Channel__Base___GetOneById___Incoming) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ Channel__Base___GetOneById___Incoming| -> Result<Channel__Base___GetOneById___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = Channel__Base___GetOneById___Incoming_ {
            application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted_ {
                serialized: incoming.application_user_access_token_encrypted.serialized.clone_as_vec()?,
                encoded: incoming.application_user_access_token_encrypted.encoded.clone_as_vec()?,
            },
            channel__id: incoming.channel__id,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn channel___base____get_one_by_id____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type Channel__Base___GetOneById___C_Result = C_Result<C_UnifiedReport<Channel__Base___GetOneById___Outcoming, Channel__Base___GetOneById___Precedent>>;
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
pub extern "C" fn channel___base____get_one_by_id____deserialize____allocate(c_vector_of_bytes: *mut C_Vector<c_uchar>) -> *mut Channel__Base___GetOneById___C_Result {
    let converter = move |unified_report: UnifiedReport<Channel__Base___GetOneById___Outcoming_, Channel__Base___GetOneById___Precedent_>| -> Result<C_UnifiedReport<Channel__Base___GetOneById___Outcoming, Channel__Base___GetOneById___Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        C_Data::empty()
                    }
                    Data::Filled { data: data__ } => {
                        let channel__description = match data__.channel.channel__description {
                            Some(channel__description_) => C_Option::data(Allocator::<C_String>::allocate(channel__description_)),
                            None => C_Option::none()
                        };

                        let channel__cover_image_path = match data__.channel.channel__cover_image_path {
                            Some(channel__cover_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel__cover_image_path_)),
                            None => C_Option::none()
                        };

                        let channel__background_image_path = match data__.channel.channel__background_image_path {
                            Some(channel__background_image_path_) => C_Option::data(Allocator::<C_String>::allocate(channel__background_image_path_)),
                            None => C_Option::none()
                        };

                        let channel_2 = Channel2 {
                            channel__owner: data__.channel.channel__owner,
                            channel__name: Allocator::<C_String>::allocate(data__.channel.channel__name),
                            channel__linked_name: Allocator::<C_String>::allocate(data__.channel.channel__linked_name),
                            channel__description,
                            channel__access_modifier: data__.channel.channel__access_modifier,
                            channel__visability_modifier: data__.channel.channel__visability_modifier,
                            channel__orientation: Allocator::<C_Vector<_>>::allocate(data__.channel.channel__orientation),
                            channel__cover_image_path,
                            channel__background_image_path,
                            channel__subscribers_quantity: data__.channel.channel__subscribers_quantity,
                            channel__marks_quantity: data__.channel.channel__marks_quantity,
                            channel__viewing_quantity: data__.channel. channel__viewing_quantity,
                        };

                        let mut channel_inner_link_registry: Vec<ChannelInnerLink1> = vec![];

                        '_a: for channel_inner_link_1 in data__.channel_inner_link_registry {
                            let channel_inner_link_1_ = ChannelInnerLink1 {
                                channel_inner_link__to: channel_inner_link_1.channel_inner_link__to
                            };

                            channel_inner_link_registry.push(channel_inner_link_1_);
                        }

                        let mut channel_outer_link_registry: Vec<ChannelOuterLink1> = vec![];

                        '_a: for channel_outer_link_1 in data__.channel_outer_link_registry {
                            let channel_outer_link_1_ = ChannelOuterLink1 {
                                channel_outer_link__alias: Allocator::<C_String>::allocate(channel_outer_link_1.channel_outer_link__alias),
                                channel_outer_link__address: Allocator::<C_String>::allocate(channel_outer_link_1.channel_outer_link__address)
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
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn channel___base____get_one_by_id____deserialize____deallocate(c_result: *mut Channel__Base___GetOneById___C_Result) -> () {
    if c_result.is_null() {
        return ();
    }
    let c_result_ = unsafe { Box::from_raw(c_result) };
    if c_result_.is_data {
        if c_result_.data.is_target {
            if c_result_.data.target.is_filled {
                Allocator::<C_String>::deallocate(&c_result_.data.target.filled.channel.channel__name);
                Allocator::<C_String>::deallocate(&c_result_.data.target.filled.channel.channel__linked_name);
                if c_result_.data.target.filled.channel.channel__description.is_data {
                    Allocator::<C_String>::deallocate(&c_result_.data.target.filled.channel.channel__description.data);
                }
                if c_result_.data.target.filled.channel.channel__background_image_path.is_data {
                    Allocator::<C_String>::deallocate(&c_result_.data.target.filled.channel.channel__background_image_path.data);
                }
                if c_result_.data.target.filled.channel.channel__cover_image_path.is_data {
                    Allocator::<C_String>::deallocate(&c_result_.data.target.filled.channel.channel__cover_image_path.data);
                }
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.channel.channel__orientation);
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.channel_inner_link_registry);
                let channel_outer_link_registry = c_result_.data.target.filled.channel_outer_link_registry.as_slice_unchecked();
                for channel_outer_link_1 in channel_outer_link_registry {
                    Allocator::<C_String>::deallocate(&channel_outer_link_1.channel_outer_link__alias);
                    Allocator::<C_String>::deallocate(&channel_outer_link_1.channel_outer_link__address);
                }
                Allocator::<C_Vector<_>>::deallocate(&c_result_.data.target.filled.channel_outer_link_registry);
            }
        }
    }
    return ();
}
#[repr(C)]
pub struct ChannelSubscription__Base___Create___Incoming {
    pub application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted,
    pub channel__id: c_long,
}
#[no_mangle]
pub extern "C" fn channel_subscription___base____create____serialize____allocate(incoming: *mut ChannelSubscription__Base___Create___Incoming) -> *mut C_Result<C_Vector<c_uchar>> {
    let converter = move |incoming: &'_ ChannelSubscription__Base___Create___Incoming| -> Result<ChannelSubscription__Base___Create___Incoming_, Box<dyn StdError + 'static>> {
        let incoming_ = ChannelSubscription__Base___Create___Incoming_ {
            application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted_ {
                serialized: incoming.application_user_access_token_encrypted.serialized.clone_as_vec()?,
                encoded: incoming.application_user_access_token_encrypted.encoded.clone_as_vec()?,
            },
            channel__id: incoming.channel__id,
        };
        return Ok(incoming_);
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn channel_subscription___base____create____serialize____deallocate(c_result: *mut C_Result<C_Vector<c_uchar>>) -> () {
    Allocator::<C_Result<C_Vector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ChannelSubscription__Base___Create___C_Result = C_Result<C_UnifiedReport<C_Void, ChannelSubscription__Base___Create___Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ChannelSubscription__Base___Create___Precedent {
    pub application_user_access_token__already_expired: bool,
    pub application_user_access_token__in_application_user_access_token_black_list: bool,
    pub channel__not_found: bool,
    pub channel__is_close: bool,
    pub application_user__is_channel__owner: bool,
}
#[no_mangle]
pub extern "C" fn channel_subscription___base____create____deserialize____allocate(c_vector_of_bytes: *mut C_Vector<c_uchar>) -> *mut ChannelSubscription__Base___Create___C_Result {
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
                            application_user__is_channel__owner: true,
                            ..Default::default()
                        }
                    }
                };

                C_UnifiedReport::precedent(precedent_)
            }
        };

        return Ok(unified_report_);
    };
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[no_mangle]
pub extern "C" fn channel_subscription___base____create____deserialize____deallocate(c_result: *mut ChannelSubscription__Base___Create___C_Result) -> () {
    Allocator::<C_Result<C_UnifiedReport<_, _>>>::deallocate(c_result);
    return ();
}
// All tests should be executed using the `Valgrind` utility as `cargo valgrind test ...' command.
#[cfg(test)]
mod test {
    use super::*;
    mod deallocation {
        use super::*;
        const NOT_EMPTY_STRING_LITERAL: &'static str = "qwerty";
        const NOT_EMPTY_ARRAY_LITERAL: [u8; 3] = [0, 1, 2];
        #[test]
        fn c_vector_clone() -> Result<(), Box<dyn StdError + 'static>> {
            let c_vector = Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec());
            {
                let _ = c_vector.clone_as_vec()?;
            }
            if c_vector.pointer.is_null() {
                return Err(ALLOCATION_ERROR.into());
            }
            Allocator::<C_Vector<_>>::deallocate(&c_vector);
            return Ok(());
        }
        #[test]
        fn c_string_clone() -> Result<(), Box<dyn StdError + 'static>> {
            let c_string = Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string());
            {
                let _ = c_string.clone_as_string()?;
            }
            if c_string.pointer.is_null() {
                return Err(ALLOCATION_ERROR.into());
            }
            Allocator::<C_String>::deallocate(&c_string);
            return Ok(());
        }
        mod server_response_data_deserialization {
            use super::*;
            use aggregate_error::AggregateError;
            use formatter::Formatter;
            fn run_by_template<'a, T, E, A, D>(data: &'a T, allocator: A, deallocator: D) -> Result<(), Box<dyn StdError + 'static>>
            where
                T: Serialize,
                A: FnOnce(*mut C_Vector<c_uchar>) -> *mut C_Result<E>,
                D: FnOnce(*mut C_Result<E>) -> (),
            {
                let registry = Serializer_::serialize(data)
                    .map_err(
                        |aggregate_error: _| -> Box<dyn StdError + 'static> {
                            return Formatter::<AggregateError>::format(&aggregate_error).into()
                        }
                    )?;
                let c_vector = Allocator::<C_Vector<_>>::allocate(registry);
                let c_vector_ = ((&c_vector) as *const _) as *mut _;
                let c_result = allocator(c_vector_);
                let c_result_ = unsafe { &*c_result };
                if !c_result_.is_data {
                    return Err(ALLOCATION_ERROR.into());
                }
                deallocator(c_result);
                if c_vector_.is_null() {
                    return Err(DEALLOCATION_ERROR.into());
                }
                Allocator::<C_Vector<_>>::deallocate(&c_vector);
                return Ok(());
            }
            // Needed to test all `unified_report::UnifiedReport` variants.
            mod unified_report {
                use super::*;
                use action_processor_incoming_outcoming::{
                    Channel1 as Channel1_,
                    Channel2 as Channel2_,
                    ChannelInnerLink1 as ChannelInnerLink1_,
                    ChannelOuterLink1 as ChannelOuterLink1_,
                    Common1 as Common1_,
                };
                #[test]
                fn target_empty____application_user___authorization____authorize_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming_,
                        ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent_,
                    >::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result {
                        return application_user___authorization____authorize_by_first_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result| -> () {
                        application_user___authorization____authorize_by_first_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____authorize_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming_ {
                        application_user__id: 0,
                        verification_message_sent: false,
                        application_user_authorization_token__can_be_resent_from: 0,
                        application_user_authorization_token__wrong_enter_tries_quantity: 0,
                        application_user_authorization_token__wrong_enter_tries_quantity_limit: 0,
                    };
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming_,
                        ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent_,
                    >::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result {
                        return application_user___authorization____authorize_by_first_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result| -> () {
                        application_user___authorization____authorize_by_first_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn precedent____application_user___authorization____authorize_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let precedent = ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent_::ApplicationUser_WrongEmailOrNicknameOrPassword;
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming_,
                        ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent_,
                    >::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result {
                        return application_user___authorization____authorize_by_first_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result| -> () {
                        application_user___authorization____authorize_by_first_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_empty____application_user___authorization____authorize_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming_,
                        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_,
                    >::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result {
                        return application_user___authorization____authorize_by_last_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result| -> () {
                        application_user___authorization____authorize_by_last_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____authorize_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming_ {
                        application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted_ {
                            serialized: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                            encoded: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                        },
                        application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted_(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    };
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming_,
                        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_,
                    >::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result {
                        return application_user___authorization____authorize_by_last_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result| -> () {
                        application_user___authorization____authorize_by_last_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                fn _precedent____application_user___authorization____authorize_by_last_step(
                    precedent: ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_,
                ) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming_,
                        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_,
                    >::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result {
                        return application_user___authorization____authorize_by_last_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___AuthorizeByLastStep___C_Result| -> () {
                        application_user___authorization____authorize_by_last_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn precedent____application_user___authorization____authorize_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_> = vec![];
                    precedent_registry.push(ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_::ApplicationUserAuthorizationToken_NotFound);
                    precedent_registry.push(ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_::ApplicationUserAuthorizationToken_AlreadyExpired);
                    precedent_registry.push(
                        ApplicationUser__Authorization___AuthorizeByLastStep___Precedent_::ApplicationUserAuthorizationToken_WrongValue {
                            application_user_authorization_token__wrong_enter_tries_quantity: 0,
                        },
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
                        return application_user___authorization____check_email_for_existing____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___CheckEmailForExisting___C_Result| -> () {
                        application_user___authorization____check_email_for_existing____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____check_email_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___CheckEmailForExisting___Outcoming_ {
                        result: false,
                    };
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___CheckEmailForExisting___Outcoming_, Void>::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___CheckEmailForExisting___C_Result {
                        return application_user___authorization____check_email_for_existing____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___CheckEmailForExisting___C_Result| -> () {
                        application_user___authorization____check_email_for_existing____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn precedent____application_user___authorization____check_email_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }
                #[test]
                fn target_empty____application_user___authorization____check_nickname_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming_, Void>::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___CheckNicknameForExisting___C_Result {
                        return application_user___authorization____check_nickname_for_existing____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___CheckNicknameForExisting___C_Result| -> () {
                        application_user___authorization____check_nickname_for_existing____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____check_nickname_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming_ {
                        result: false,
                    };
                    let unified_report = UnifiedReport::<ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming_, Void>::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___CheckNicknameForExisting___C_Result {
                        return application_user___authorization____check_nickname_for_existing____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___CheckNicknameForExisting___C_Result| -> () {
                        application_user___authorization____check_nickname_for_existing____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn precedent____application_user___authorization____check_nickname_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }
                #[test]
                fn target_empty____application_user___authorization____deauthorize_from_all_devices() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_>::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result {
                        return application_user___authorization____deauthorize_from_all_devices____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result| -> () {
                        application_user___authorization____deauthorize_from_all_devices____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____deauthorize_from_all_devices() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }
                fn _precedent____application_user___authorization____deauthorize_from_all_devices(
                    precedent: ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_,
                ) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_>::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result {
                        return application_user___authorization____deauthorize_from_all_devices____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result| -> () {
                        application_user___authorization____deauthorize_from_all_devices____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn precedent____application_user___authorization____deauthorize_from_all_devices() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_> = vec![];
                    precedent_registry.push(ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_::ApplicationUserAccessToken_AlreadyExpired);
                    precedent_registry
                        .push(ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList);
                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____deauthorize_from_all_devices(precedent)?;
                    }
                    return Ok(());
                }
                #[test]
                fn target_empty____application_user___authorization____deauthorize_from_one_device() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_>::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result {
                        return application_user___authorization____deauthorize_from_one_device____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result| -> () {
                        application_user___authorization____deauthorize_from_one_device____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____deauthorize_from_one_device() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }
                fn _precedent____application_user___authorization____deauthorize_from_one_device(
                    precedent: ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_,
                ) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_>::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result {
                        return application_user___authorization____deauthorize_from_one_device____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result| -> () {
                        application_user___authorization____deauthorize_from_one_device____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn precedent____application_user___authorization____deauthorize_from_one_device() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_> = vec![];
                    precedent_registry.push(ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_::ApplicationUserAccessToken_AlreadyExpired);
                    precedent_registry
                        .push(ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent_::ApplicationUserAccessToken_InApplicationUserAccessTokenBlackList);
                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____deauthorize_from_one_device(precedent)?;
                    }
                    return Ok(());
                }
                #[test]
                fn target_empty____application_user___authorization____refresh_access_token() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___RefreshAccessToken___Outcoming_,
                        ApplicationUser__Authorization___RefreshAccessToken___Precedent_,
                    >::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result {
                        return application_user___authorization____refresh_access_token____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result| -> () {
                        application_user___authorization____refresh_access_token____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____refresh_access_token() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___RefreshAccessToken___Outcoming_ {
                        application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted_ {
                            serialized: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                            encoded: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                        },
                        application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted_(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    };
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___RefreshAccessToken___Outcoming_,
                        ApplicationUser__Authorization___RefreshAccessToken___Precedent_,
                    >::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result {
                        return application_user___authorization____refresh_access_token____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result| -> () {
                        application_user___authorization____refresh_access_token____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                fn _precedent____application_user___authorization____refresh_access_token(
                    precedent: ApplicationUser__Authorization___RefreshAccessToken___Precedent_,
                ) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___RefreshAccessToken___Outcoming_,
                        ApplicationUser__Authorization___RefreshAccessToken___Precedent_,
                    >::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result {
                        return application_user___authorization____refresh_access_token____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RefreshAccessToken___C_Result| -> () {
                        application_user___authorization____refresh_access_token____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
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
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___RegisterByFirstStep___Outcoming_,
                        ApplicationUser__Authorization___RegisterByFirstStep___Precedent_,
                    >::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result {
                        return application_user___authorization____register_by_first_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result| -> () {
                        application_user___authorization____register_by_first_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____register_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___RegisterByFirstStep___Outcoming_ {
                        verification_message_sent: false,
                        application_user_registration_token__can_be_resent_from: 0,
                        application_user_registration_token__wrong_enter_tries_quantity: 0,
                        application_user_registration_token__wrong_enter_tries_quantity_limit: 0,
                    };
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___RegisterByFirstStep___Outcoming_,
                        ApplicationUser__Authorization___RegisterByFirstStep___Precedent_,
                    >::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result {
                        return application_user___authorization____register_by_first_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result| -> () {
                        application_user___authorization____register_by_first_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn precedent____application_user___authorization____register_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let precedent = ApplicationUser__Authorization___RegisterByFirstStep___Precedent_::ApplicationUser_EmailAlreadyExist;
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___RegisterByFirstStep___Outcoming_,
                        ApplicationUser__Authorization___RegisterByFirstStep___Precedent_,
                    >::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result {
                        return application_user___authorization____register_by_first_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterByFirstStep___C_Result| -> () {
                        application_user___authorization____register_by_first_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_empty____application_user___authorization____register_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___RegisterBySecondStep___Precedent_>::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterBySecondStep___C_Result {
                        return application_user___authorization____register_by_second_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterBySecondStep___C_Result| -> () {
                        application_user___authorization____register_by_second_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____register_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }
                fn _precedent____application_user___authorization____register_by_second_step(
                    precedent: ApplicationUser__Authorization___RegisterBySecondStep___Precedent_,
                ) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___RegisterBySecondStep___Precedent_>::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterBySecondStep___C_Result {
                        return application_user___authorization____register_by_second_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterBySecondStep___C_Result| -> () {
                        application_user___authorization____register_by_second_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn precedent____application_user___authorization____register_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___RegisterBySecondStep___Precedent_> = vec![];
                    precedent_registry.push(ApplicationUser__Authorization___RegisterBySecondStep___Precedent_::ApplicationUserRegistrationToken_NotFound);
                    precedent_registry.push(ApplicationUser__Authorization___RegisterBySecondStep___Precedent_::ApplicationUserRegistrationToken_AlreadyExpired);
                    precedent_registry.push(ApplicationUser__Authorization___RegisterBySecondStep___Precedent_::ApplicationUserRegistrationToken_AlreadyApproved);
                    precedent_registry.push(
                        ApplicationUser__Authorization___RegisterBySecondStep___Precedent_::ApplicationUserRegistrationToken_WrongValue {
                            application_user_registration_token__wrong_enter_tries_quantity: 0,
                        },
                    );
                    '_a: for precedent in precedent_registry {
                        _precedent____application_user___authorization____register_by_second_step(precedent)?;
                    }
                    return Ok(());
                }
                #[test]
                fn target_empty____application_user___authorization____register_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___RegisterByLastStep___Outcoming_,
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent_,
                    >::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result {
                        return application_user___authorization____register_by_last_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result| -> () {
                        application_user___authorization____register_by_last_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____register_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___RegisterByLastStep___Outcoming_ {
                        application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted_ {
                            serialized: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                            encoded: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                        },
                        application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted_(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    };
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___RegisterByLastStep___Outcoming_,
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent_,
                    >::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result {
                        return application_user___authorization____register_by_last_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result| -> () {
                        application_user___authorization____register_by_last_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                fn _precedent____application_user___authorization____register_by_last_step(
                    precedent: ApplicationUser__Authorization___RegisterByLastStep___Precedent_,
                ) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___RegisterByLastStep___Outcoming_,
                        ApplicationUser__Authorization___RegisterByLastStep___Precedent_,
                    >::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result {
                        return application_user___authorization____register_by_last_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___RegisterByLastStep___C_Result| -> () {
                        application_user___authorization____register_by_last_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
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
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming_,
                        ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_,
                    >::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result {
                        return application_user___authorization____reset_password_by_first_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_first_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____reset_password_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming_ {
                        application_user__id: 0,
                        verification_message_sent: false,
                        application_user_reset_password_token__can_be_resent_from: 0,
                        application_user_reset_password_token__wrong_enter_tries_quantity: 0,
                        application_user_reset_password_token__wrong_enter_tries_quantity_limit: 0,
                    };
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming_,
                        ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_,
                    >::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result {
                        return application_user___authorization____reset_password_by_first_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_first_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn precedent____application_user___authorization____reset_password_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let precedent = ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_::ApplicationUser_NotFound;
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming_,
                        ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent_,
                    >::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result {
                        return application_user___authorization____reset_password_by_first_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_first_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_empty____application_user___authorization____reset_password_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_>::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result {
                        return application_user___authorization____reset_password_by_second_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_second_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____reset_password_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }
                fn _precedent____application_user___authorization____reset_password_by_second_step(
                    precedent: ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_,
                ) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_>::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result {
                        return application_user___authorization____reset_password_by_second_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_second_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn precedent____application_user___authorization____reset_password_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut precedent_registry: Vec<ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_> = vec![];
                    precedent_registry.push(ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_::ApplicationUserResetPasswordToken_NotFound);
                    precedent_registry.push(ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_::ApplicationUserResetPasswordToken_AlreadyExpired);
                    precedent_registry.push(ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_::ApplicationUserResetPasswordToken_AlreadyApproved);
                    precedent_registry.push(
                        ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent_::ApplicationUserResetPasswordToken_WrongValue {
                            application_user_reset_password_token__wrong_enter_tries_quantity: 0,
                        },
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
                        return application_user___authorization____reset_password_by_last_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_last_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____reset_password_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }
                fn _precedent____application_user___authorization____reset_password_by_last_step(
                    precedent: ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_,
                ) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent_>::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result {
                        return application_user___authorization____reset_password_by_last_step____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result| -> () {
                        application_user___authorization____reset_password_by_last_step____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
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
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___SendEmailForRegister___Outcoming_,
                        ApplicationUser__Authorization___SendEmailForRegister___Precedent_,
                    >::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result {
                        return application_user___authorization____send_email_for_register____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result| -> () {
                        application_user___authorization____send_email_for_register____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____send_email_for_register() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___SendEmailForRegister___Outcoming_ {
                        application_user_registration_token__can_be_resent_from: 0,
                    };
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___SendEmailForRegister___Outcoming_,
                        ApplicationUser__Authorization___SendEmailForRegister___Precedent_,
                    >::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result {
                        return application_user___authorization____send_email_for_register____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result| -> () {
                        application_user___authorization____send_email_for_register____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                fn _precedent____application_user___authorization____send_email_for_register(
                    precedent: ApplicationUser__Authorization___SendEmailForRegister___Precedent_,
                ) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___SendEmailForRegister___Outcoming_,
                        ApplicationUser__Authorization___SendEmailForRegister___Precedent_,
                    >::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result {
                        return application_user___authorization____send_email_for_register____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForRegister___C_Result| -> () {
                        application_user___authorization____send_email_for_register____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
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
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming_,
                        ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_,
                    >::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result {
                        return application_user___authorization____send_email_for_authorize____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result| -> () {
                        application_user___authorization____send_email_for_authorize____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____send_email_for_authorize() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming_ {
                        application_user_authorization_token__can_be_resent_from: 0,
                    };
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming_,
                        ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_,
                    >::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result {
                        return application_user___authorization____send_email_for_authorize____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result| -> () {
                        application_user___authorization____send_email_for_authorize____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                fn _precedent____application_user___authorization____send_email_for_authorize(
                    precedent: ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_,
                ) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming_,
                        ApplicationUser__Authorization___SendEmailForAuthorize___Precedent_,
                    >::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result {
                        return application_user___authorization____send_email_for_authorize____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForAuthorize___C_Result| -> () {
                        application_user___authorization____send_email_for_authorize____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
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
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming_,
                        ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_,
                    >::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result {
                        return application_user___authorization____send_email_for_reset_password____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result| -> () {
                        application_user___authorization____send_email_for_reset_password____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____application_user___authorization____send_email_for_reset_password() -> Result<(), Box<dyn StdError + 'static>> {
                    let outcoming = ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming_ {
                        application_user_reset_password_token__can_be_resent_from: 0,
                    };
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming_,
                        ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_,
                    >::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result {
                        return application_user___authorization____send_email_for_reset_password____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result| -> () {
                        application_user___authorization____send_email_for_reset_password____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                fn _precedent____application_user___authorization____send_email_for_reset_password(
                    precedent: ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_,
                ) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<
                        ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming_,
                        ApplicationUser__Authorization___SendEmailForResetPassword___Precedent_,
                    >::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result {
                        return application_user___authorization____send_email_for_reset_password____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ApplicationUser__Authorization___SendEmailForResetPassword___C_Result| -> () {
                        application_user___authorization____send_email_for_reset_password____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
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
                    let unified_report =
                        UnifiedReport::<Channel__Base___GetManyByNameInSubscriptions___Outcoming_, Channel__Base___GetManyByNameInSubscriptions___Precedent_>::target_empty();
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyByNameInSubscriptions___C_Result {
                        return channel___base____get_many_by_name_in_subscriptions____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut Channel__Base___GetManyByNameInSubscriptions___C_Result| -> () {
                        channel___base____get_many_by_name_in_subscriptions____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____channel___base____get_many_by_name_in_subscriptions() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut common_registry: Vec<Common1_> = vec![];
                    '_a: for _ in 1..=5 {
                        let common_1 = Common1_ {
                            channel: Channel1_ {
                                channel__id: 0,
                                channel__name: NOT_EMPTY_STRING_LITERAL.to_string(),
                                channel__linked_name: NOT_EMPTY_STRING_LITERAL.to_string(),
                                channel__access_modifier: 0,
                                channel__visability_modifier: 0,
                                channel__background_image_path: Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                                channel__cover_image_path: Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                            },
                            is_application_user_subscribed: false,
                        };
                        common_registry.push(common_1);
                    }
                    let outcoming = Channel__Base___GetManyByNameInSubscriptions___Outcoming_ {
                        common_registry,
                    };
                    let unified_report =
                        UnifiedReport::<Channel__Base___GetManyByNameInSubscriptions___Outcoming_, Channel__Base___GetManyByNameInSubscriptions___Precedent_>::target_filled(
                            outcoming,
                        );
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyByNameInSubscriptions___C_Result {
                        return channel___base____get_many_by_name_in_subscriptions____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut Channel__Base___GetManyByNameInSubscriptions___C_Result| -> () {
                        channel___base____get_many_by_name_in_subscriptions____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                fn _precedent____channel___base____get_many_by_name_in_subscriptions(
                    precedent: Channel__Base___GetManyByNameInSubscriptions___Precedent_,
                ) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report =
                        UnifiedReport::<Channel__Base___GetManyByNameInSubscriptions___Outcoming_, Channel__Base___GetManyByNameInSubscriptions___Precedent_>::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyByNameInSubscriptions___C_Result {
                        return channel___base____get_many_by_name_in_subscriptions____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut Channel__Base___GetManyByNameInSubscriptions___C_Result| -> () {
                        channel___base____get_many_by_name_in_subscriptions____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
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
                        return channel___base____get_many_by_subscription____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut Channel__Base___GetManyBySubscription___C_Result| -> () {
                        channel___base____get_many_by_subscription____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____channel___base____get_many_by_subscription() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut common_registry: Vec<Common1_> = vec![];
                    '_a: for _ in 1..=2 {
                        let common_1 = Common1_ {
                            channel: Channel1_ {
                                channel__id: 0,
                                channel__name: NOT_EMPTY_STRING_LITERAL.to_string(),
                                channel__linked_name: NOT_EMPTY_STRING_LITERAL.to_string(),
                                channel__access_modifier: 0,
                                channel__visability_modifier: 0,
                                channel__background_image_path: Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                                channel__cover_image_path: Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                            },
                            is_application_user_subscribed: false,
                        };
                        common_registry.push(common_1);
                    }
                    let outcoming = Channel__Base___GetManyBySubscription___Outcoming_ {
                        common_registry,
                    };
                    let unified_report =
                        UnifiedReport::<Channel__Base___GetManyBySubscription___Outcoming_, Channel__Base___GetManyBySubscription___Precedent_>::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyBySubscription___C_Result {
                        return channel___base____get_many_by_subscription____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut Channel__Base___GetManyBySubscription___C_Result| -> () {
                        channel___base____get_many_by_subscription____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                fn _precedent____channel___base____get_many_by_subscription(
                    precedent: Channel__Base___GetManyBySubscription___Precedent_,
                ) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report =
                        UnifiedReport::<Channel__Base___GetManyBySubscription___Outcoming_, Channel__Base___GetManyBySubscription___Precedent_>::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyBySubscription___C_Result {
                        return channel___base____get_many_by_subscription____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut Channel__Base___GetManyBySubscription___C_Result| -> () {
                        channel___base____get_many_by_subscription____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
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
                        return channel___base____get_many_public_by_name____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut Channel__Base___GetManyPublicByName___C_Result| -> () {
                        channel___base____get_many_public_by_name____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____channel___base____get_many_public_by_name() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut common_registry: Vec<Common1_> = vec![];
                    '_a: for _ in 1..=5 {
                        let common_1 = Common1_ {
                            channel: Channel1_ {
                                channel__id: 0,
                                channel__name: NOT_EMPTY_STRING_LITERAL.to_string(),
                                channel__linked_name: NOT_EMPTY_STRING_LITERAL.to_string(),
                                channel__access_modifier: 0,
                                channel__visability_modifier: 0,
                                channel__background_image_path: Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                                channel__cover_image_path: Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                            },
                            is_application_user_subscribed: false,
                        };
                        common_registry.push(common_1);
                    }
                    let outcoming = Channel__Base___GetManyPublicByName___Outcoming_ {
                        common_registry,
                    };
                    let unified_report =
                        UnifiedReport::<Channel__Base___GetManyPublicByName___Outcoming_, Channel__Base___GetManyPublicByName___Precedent_>::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyPublicByName___C_Result {
                        return channel___base____get_many_public_by_name____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut Channel__Base___GetManyPublicByName___C_Result| -> () {
                        channel___base____get_many_public_by_name____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                fn _precedent____channel___base____get_many_public_by_name(precedent: Channel__Base___GetManyPublicByName___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Channel__Base___GetManyPublicByName___Outcoming_, Channel__Base___GetManyPublicByName___Precedent_>::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetManyPublicByName___C_Result {
                        return channel___base____get_many_public_by_name____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut Channel__Base___GetManyPublicByName___C_Result| -> () {
                        channel___base____get_many_public_by_name____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
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
                        return channel___base____get_one_by_id____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut Channel__Base___GetOneById___C_Result| -> () {
                        channel___base____get_one_by_id____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____channel___base____get_one_by_id() -> Result<(), Box<dyn StdError + 'static>> {
                    let mut channel_inner_link_registry: Vec<ChannelInnerLink1_> = vec![];
                    '_a: for _ in 1..=5 {
                        let channel_inner_link_1 = ChannelInnerLink1_ {
                            channel_inner_link__to: 0,
                        };
                        channel_inner_link_registry.push(channel_inner_link_1);
                    }
                    let mut channel_outer_link_registry: Vec<ChannelOuterLink1_> = vec![];
                    '_a: for _ in 1..=5 {
                        let channel_outer_link_1 = ChannelOuterLink1_ {
                            channel_outer_link__alias: NOT_EMPTY_STRING_LITERAL.to_string(),
                            channel_outer_link__address: NOT_EMPTY_STRING_LITERAL.to_string(),
                        };
                        channel_outer_link_registry.push(channel_outer_link_1);
                    }
                    let channel_2 = Channel2_ {
                        channel__owner: 0,
                        channel__name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__linked_name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__description: Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel__access_modifier: 0,
                        channel__visability_modifier: 0,
                        channel__orientation: vec![0, 0, 0],
                        channel__background_image_path: Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel__cover_image_path: Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel__subscribers_quantity: 0,
                        channel__marks_quantity: 0,
                        channel__viewing_quantity: 0,
                    };
                    let outcoming = Channel__Base___GetOneById___Outcoming_ {
                        channel: channel_2,
                        channel_inner_link_registry,
                        channel_outer_link_registry,
                    };
                    let unified_report = UnifiedReport::<Channel__Base___GetOneById___Outcoming_, Channel__Base___GetOneById___Precedent_>::target_filled(outcoming);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetOneById___C_Result {
                        return channel___base____get_one_by_id____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut Channel__Base___GetOneById___C_Result| -> () {
                        channel___base____get_one_by_id____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                fn _precedent____channel___base____get_one_by_id(precedent: Channel__Base___GetOneById___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Channel__Base___GetOneById___Outcoming_, Channel__Base___GetOneById___Precedent_>::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut Channel__Base___GetOneById___C_Result {
                        return channel___base____get_one_by_id____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut Channel__Base___GetOneById___C_Result| -> () {
                        channel___base____get_one_by_id____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
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
                        return channel_subscription___base____create____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ChannelSubscription__Base___Create___C_Result| -> () {
                        channel_subscription___base____create____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
                }
                #[test]
                fn target_filled____channel_subscription___base____create() -> Result<(), Box<dyn StdError + 'static>> {
                    return Ok(());
                }
                fn _precedent____channel_subscription___base____create(precedent: ChannelSubscription__Base___Create___Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                    let unified_report = UnifiedReport::<Void, ChannelSubscription__Base___Create___Precedent_>::precedent(precedent);
                    let allocator = move |vector_of_bytes: *mut C_Vector<c_uchar>| -> *mut ChannelSubscription__Base___Create___C_Result {
                        return channel_subscription___base____create____deserialize____allocate(vector_of_bytes);
                    };
                    let deallocator = move |c_result: *mut ChannelSubscription__Base___Create___C_Result| -> () {
                        channel_subscription___base____create____deserialize____deallocate(c_result);
                        return ();
                    };
                    run_by_template(
                        &unified_report,
                        allocator,
                        deallocator,
                    )?;
                    return Ok(());
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
            fn run_by_template<'a, I, A, D>(incoming: &'a I, allocator: A, deallocator: D) -> Result<(), Box<dyn StdError + 'static>>
            where
                A: FnOnce(*mut I) -> *mut C_Result<C_Vector<c_uchar>>,
                D: FnOnce(*mut C_Result<C_Vector<c_uchar>>) -> (),
            {
                let incoming_ = (incoming as *const _) as *mut _;
                let c_result = allocator(incoming_);
                let c_result_ = unsafe { &*c_result };
                if !c_result_.is_data {
                    return Err(ALLOCATION_ERROR.into());
                }
                deallocator(c_result);
                if incoming_.is_null() {
                    return Err(DEALLOCATION_ERROR.into());
                }
                return Ok(());
            }
            #[test]
            fn application_user___authorization____authorize_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming {
                    application_user_device__id: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user__email___or___application_user__nickname: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user__password: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____authorize_by_first_step____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____authorize_by_first_step____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_String>::deallocate(&incoming.application_user_device__id);
                Allocator::<C_String>::deallocate(&incoming.application_user__email___or___application_user__nickname);
                Allocator::<C_String>::deallocate(&incoming.application_user__password);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____authorize_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___AuthorizeByLastStep___Incoming {
                    application_user__id: 0,
                    application_user_device__id: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user_authorization_token__value: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___AuthorizeByLastStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____authorize_by_last_step____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____authorize_by_last_step____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_String>::deallocate(&incoming.application_user_device__id);
                Allocator::<C_String>::deallocate(&incoming.application_user_authorization_token__value);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____check_email_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___CheckEmailForExisting___Incoming {
                    application_user__email: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___CheckEmailForExisting___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____check_email_for_existing____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____check_email_for_existing____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_String>::deallocate(&incoming.application_user__email);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____check_nickname_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___CheckNicknameForExisting___Incoming {
                    application_user__nickname: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___CheckNicknameForExisting___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____check_nickname_for_existing____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____check_nickname_for_existing____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_String>::deallocate(&incoming.application_user__nickname);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____deauthorize_from_all_devices() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming {
                    application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted {
                        serialized: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                        encoded: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___DeauthorizeFromAllDevices___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____deauthorize_from_all_devices____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____deauthorize_from_all_devices____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.serialized);
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.encoded);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____deauthorize_from_one_device() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming {
                    application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted {
                        serialized: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                        encoded: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___DeauthorizeFromOneDevice___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____deauthorize_from_one_device____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____deauthorize_from_one_device____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.serialized);
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.encoded);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____refresh_access_token() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___RefreshAccessToken___Incoming {
                    application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted {
                        serialized: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                        encoded: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    application_user_access_refresh_token_encrypted: ApplicationUserAccessRefreshTokenEncrypted(
                        Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec())
                    ),
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___RefreshAccessToken___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____refresh_access_token____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____refresh_access_token____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.serialized);
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.encoded);
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_refresh_token_encrypted.0);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____register_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___RegisterByFirstStep___Incoming {
                    application_user__email: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user_device__id: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___RegisterByFirstStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____register_by_first_step____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____register_by_first_step____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_String>::deallocate(&incoming.application_user__email);
                Allocator::<C_String>::deallocate(&incoming.application_user_device__id);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____register_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___RegisterBySecondStep___Incoming {
                    application_user__email: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user_device__id: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user_registration_token__value: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___RegisterBySecondStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____register_by_second_step____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____register_by_second_step____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_String>::deallocate(&incoming.application_user__email);
                Allocator::<C_String>::deallocate(&incoming.application_user_device__id);
                Allocator::<C_String>::deallocate(&incoming.application_user_registration_token__value);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____register_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___RegisterByLastStep___Incoming {
                    application_user_device__id: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user__nickname: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user__password: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user__email: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user_registration_token__value: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___RegisterByLastStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____register_by_last_step____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____register_by_last_step____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_String>::deallocate(&incoming.application_user_device__id);
                Allocator::<C_String>::deallocate(&incoming.application_user__nickname);
                Allocator::<C_String>::deallocate(&incoming.application_user__password);
                Allocator::<C_String>::deallocate(&incoming.application_user__email);
                Allocator::<C_String>::deallocate(&incoming.application_user_registration_token__value);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____reset_password_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming {
                    application_user__email: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user_device__id: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___ResetPasswordByFirstStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____reset_password_by_first_step____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____reset_password_by_first_step____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_String>::deallocate(&incoming.application_user__email);
                Allocator::<C_String>::deallocate(&incoming.application_user_device__id);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____reset_password_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming {
                    application_user__id: 0,
                    application_user_device__id: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user_reset_password_token__value: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___ResetPasswordBySecondStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____reset_password_by_second_step____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____reset_password_by_second_step____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_String>::deallocate(&incoming.application_user_device__id);
                Allocator::<C_String>::deallocate(&incoming.application_user_reset_password_token__value);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____reset_password_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming {
                    application_user__id: 0,
                    application_user_device__id: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user__password: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user_reset_password_token__value: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___ResetPasswordByLastStep___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____reset_password_by_last_step____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____reset_password_by_last_step____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_String>::deallocate(&incoming.application_user_device__id);
                Allocator::<C_String>::deallocate(&incoming.application_user__password);
                Allocator::<C_String>::deallocate(&incoming.application_user_reset_password_token__value);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____send_email_for_register() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___SendEmailForRegister___Incoming {
                    application_user__email: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user_device__id: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___SendEmailForRegister___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____send_email_for_register____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____send_email_for_register____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_String>::deallocate(&incoming.application_user__email);
                Allocator::<C_String>::deallocate(&incoming.application_user_device__id);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____send_email_for_authorize() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___SendEmailForAuthorize___Incoming {
                    application_user_device__id: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    application_user__id: 0,
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___SendEmailForAuthorize___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____send_email_for_authorize____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____send_email_for_authorize____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_String>::deallocate(&incoming.application_user_device__id);
                return Ok(());
            }
            #[test]
            fn application_user___authorization____send_email_for_reset_password() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ApplicationUser__Authorization___SendEmailForResetPassword___Incoming {
                    application_user__id: 0,
                    application_user_device__id: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                let allocator = move |incoming: *mut ApplicationUser__Authorization___SendEmailForResetPassword___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return application_user___authorization____send_email_for_reset_password____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    application_user___authorization____send_email_for_reset_password____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_String>::deallocate(&incoming.application_user_device__id);
                return Ok(());
            }
            #[test]
            fn channel___base____get_many_by_name_in_subscriptions() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel__Base___GetManyByNameInSubscriptions___Incoming {
                    application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted {
                        serialized: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                        encoded: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel__name: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    requery___channel__name: C_Option::data(Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string())),
                    limit: 0,
                };
                let allocator = move |incoming: *mut Channel__Base___GetManyByNameInSubscriptions___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return channel___base____get_many_by_name_in_subscriptions____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    channel___base____get_many_by_name_in_subscriptions____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.serialized);
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.encoded);
                Allocator::<C_String>::deallocate(&incoming.channel__name);
                Allocator::<C_String>::deallocate(&incoming.requery___channel__name.data);
                return Ok(());
            }
            #[test]
            fn channel___base____get_many_by_subscription() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel__Base___GetManyBySubscription___Incoming {
                    application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted {
                        serialized: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                        encoded: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    requery___channel__id: C_Option::data(0),
                    limit: 0,
                };
                let allocator = move |incoming: *mut Channel__Base___GetManyBySubscription___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return channel___base____get_many_by_subscription____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    channel___base____get_many_by_subscription____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.serialized);
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.encoded);
                return Ok(());
            }
            #[test]
            fn channel___base____get_many_public_by_name() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel__Base___GetManyPublicByName___Incoming {
                    application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted {
                        serialized: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                        encoded: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel__name: Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    requery___channel__name: C_Option::data(Allocator::<C_String>::allocate(NOT_EMPTY_STRING_LITERAL.to_string())),
                    limit: 0,
                };
                let allocator = move |incoming: *mut Channel__Base___GetManyPublicByName___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return channel___base____get_many_public_by_name____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    channel___base____get_many_public_by_name____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.serialized);
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.encoded);
                Allocator::<C_String>::deallocate(&incoming.channel__name);
                Allocator::<C_String>::deallocate(&incoming.requery___channel__name.data);
                return Ok(());
            }
            #[test]
            fn channel___base____get_one_by_id() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel__Base___GetOneById___Incoming {
                    application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted {
                        serialized: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                        encoded: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel__id: 0,
                };
                let allocator = move |incoming: *mut Channel__Base___GetOneById___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return channel___base____get_one_by_id____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    channel___base____get_one_by_id____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.serialized);
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.encoded);
                return Ok(());
            }
            #[test]
            fn channel_subscription___base____create() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ChannelSubscription__Base___Create___Incoming {
                    application_user_access_token_encrypted: ApplicationUserAccessTokenEncrypted {
                        serialized: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                        encoded: Allocator::<C_Vector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel__id: 0,
                };
                let allocator = move |incoming: *mut ChannelSubscription__Base___Create___Incoming| -> *mut C_Result<C_Vector<c_uchar>> {
                    return channel_subscription___base____create____serialize____allocate(incoming);
                };
                let deallocator = move |c_result: *mut C_Result<C_Vector<c_uchar>>| -> () {
                    channel_subscription___base____create____serialize____deallocate(c_result);
                    return ();
                };
                run_by_template(
                    &incoming,
                    allocator,
                    deallocator,
                )?;
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.serialized);
                Allocator::<C_Vector<_>>::deallocate(&incoming.application_user_access_token_encrypted.encoded);
                return Ok(());
            }
        }
    }
}
