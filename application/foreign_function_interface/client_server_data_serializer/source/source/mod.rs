// It is necessary to specify permanent target endian in purpose to obtain a permanent hash-function result.
#![cfg(target_endian = "little")]
use {
    bitcode::{
        Decode,
        Encode,
    },
    dedicated::{
        action_processor_incoming_outcoming::action_processor::{
            channel::{
                get_many_by_name_in_subscriptions::{
                    Incoming as Channel_GetManyByNameInSubscriptions_Incoming_,
                    Outcoming as Channel_GetManyByNameInSubscriptions_Outcoming_,
                    Precedent as Channel_GetManyByNameInSubscriptions_Precedent_,
                },
                get_many_by_subscription::{
                    Incoming as Channel_GetManyBySubscription_Incoming_,
                    Outcoming as Channel_GetManyBySubscription_Outcoming_,
                    Precedent as Channel_GetManyBySubscription_Precedent_,
                },
                get_many_public_by_name::{
                    Incoming as Channel_GetManyPublicByName_Incoming_,
                    Outcoming as Channel_GetManyPublicByName_Outcoming_,
                    Precedent as Channel_GetManyPublicByName_Precedent_,
                },
                get_one_by_id::{
                    Incoming as Channel_GetOneById_Incoming_,
                    Outcoming as Channel_GetOneById_Outcoming_,
                    Precedent as Channel_GetOneById_Precedent_,
                },
            },
            channel_subscription::create::{
                Incoming as ChannelSubscription_Create_Incoming_,
                Precedent as ChannelSubscription_Create_Precedent_,
            },
            user_authorization::{
                authorize_by_first_step::{
                    Incoming as UserAuthorization_AuthorizeByFirstStep_Incoming_,
                    Outcoming as UserAuthorization_AuthorizeByFirstStep_Outcoming_,
                    Precedent as UserAuthorization_AuthorizeByFirstStep_Precedent_,
                },
                authorize_by_last_step::{
                    Incoming as UserAuthorization_AuthorizeByLastStep_Incoming_,
                    Outcoming as UserAuthorization_AuthorizeByLastStep_Outcoming_,
                    Precedent as UserAuthorization_AuthorizeByLastStep_Precedent_,
                },
                check_email_for_existing::{
                    Incoming as UserAuthorization_CheckEmailForExisting_Incoming_,
                    Outcoming as UserAuthorization_CheckEmailForExisting_Outcoming_,
                },
                check_nickname_for_existing::{
                    Incoming as UserAuthorization_CheckNicknameForExisting_Incoming_,
                    Outcoming as UserAuthorization_CheckNicknameForExisting_Outcoming_,
                },
                deauthorize_from_all_devices::{
                    Incoming as UserAuthorization_DeauthorizeFromAllDevices_Incoming_,
                    Precedent as UserAuthorization_DeauthorizeFromAllDevices_Precedent_,
                },
                deauthorize_from_one_device::{
                    Incoming as UserAuthorization_DeauthorizeFromOneDevice_Incoming_,
                    Precedent as UserAuthorization_DeauthorizeFromOneDevice_Precedent_,
                },
                refresh_access_token::{
                    Incoming as UserAuthorization_RefreshAccessToken_Incoming_,
                    Outcoming as UserAuthorization_RefreshAccessToken_Outcoming_,
                    Precedent as UserAuthorization_RefreshAccessToken_Precedent_,
                },
                register_by_first_step::{
                    Incoming as UserAuthorization_RegisterByFirstStep_Incoming_,
                    Outcoming as UserAuthorization_RegisterByFirstStep_Outcoming_,
                    Precedent as UserAuthorization_RegisterByFirstStep_Precedent_,
                },
                register_by_last_step::{
                    Incoming as UserAuthorization_RegisterByLastStep_Incoming_,
                    Outcoming as UserAuthorization_RegisterByLastStep_Outcoming_,
                    Precedent as UserAuthorization_RegisterByLastStep_Precedent_,
                },
                register_by_second_step::{
                    Incoming as UserAuthorization_RegisterBySecondStep_Incoming_,
                    Precedent as UserAuthorization_RegisterBySecondStep_Precedent_,
                },
                reset_password_by_first_step::{
                    Incoming as UserAuthorization_ResetPasswordByFirstStep_Incoming_,
                    Outcoming as UserAuthorization_ResetPasswordByFirstStep_Outcoming_,
                    Precedent as UserAuthorization_ResetPasswordByFirstStep_Precedent_,
                },
                reset_password_by_last_step::{
                    Incoming as UserAuthorization_ResetPasswordByLastStep_Incoming_,
                    Precedent as UserAuthorization_ResetPasswordByLastStep_Precedent_,
                },
                reset_password_by_second_step::{
                    Incoming as UserAuthorization_ResetPasswordBySecondStep_Incoming_,
                    Precedent as UserAuthorization_ResetPasswordBySecondStep_Precedent_,
                },
                send_email_for_authorize::{
                    Incoming as UserAuthorization_SendEmailForAuthorize_Incoming_,
                    Outcoming as UserAuthorization_SendEmailForAuthorize_Outcoming_,
                    Precedent as UserAuthorization_SendEmailForAuthorize_Precedent_,
                },
                send_email_for_register::{
                    Incoming as UserAuthorization_SendEmailForRegister_Incoming_,
                    Outcoming as UserAuthorization_SendEmailForRegister_Outcoming_,
                    Precedent as UserAuthorization_SendEmailForRegister_Precedent_,
                },
                send_email_for_reset_password::{
                    Incoming as UserAuthorization_SendEmailForResetPassword_Incoming_,
                    Outcoming as UserAuthorization_SendEmailForResetPassword_Outcoming_,
                    Precedent as UserAuthorization_SendEmailForResetPassword_Precedent_,
                },
            },
        },
        bit_code_serializer::Serializer,
        unified_report::{
            Data,
            UnifiedReport,
        },
        user_access_refresh_token_signed::UserAccessRefreshTokenSigned as UserAccessRefreshTokenSigned_,
        user_access_token_signed::UserAccessTokenSigned as UserAccessTokenSigned_,
        void::Void,
        channel_subscription_token_hashed::ChannelSubscriptionTokenHashed as ChannelSubscriptionTokenHashed_,
        channel_token_hashed::ChannelTokenHashed as ChannelTokenHashed_,
    },
    libc::{
        c_char,
        c_long,
        c_ulong,
        c_short,
        c_uchar,
        size_t,
    },
    std::{
        error::Error as StdError,
        ffi::{
            CStr,
            CString as CString_,
        },
        marker::PhantomData,
    },
};
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
#[derive(Clone, Copy)]
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
#[derive(Clone, Copy)]
pub struct CString {
    pub pointer: *mut c_char,
}
impl CString {
    fn clone_as_string<'a>(&'a self) -> Result<String, Box<dyn StdError + 'static>> {
        if self.pointer.is_null() {
            return Result::Err(NULL_POINTER_ERROR_MESAGE.into());
        }
        let c_str = unsafe { CStr::from_ptr(self.pointer as *const _) };
        let string = c_str.to_str()?.to_string();
        return Result::Ok(string);
    }
    fn get_as_str<'a, 'b>(&'a self) -> Result<&'b str, Box<dyn StdError + 'static>> {
        if self.pointer.is_null() {
            return Result::Err(NULL_POINTER_ERROR_MESAGE.into());
        }
        let c_str = unsafe { CStr::from_ptr(self.pointer as *const _) };
        return Result::Ok(c_str.to_str()?);
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
#[derive(Clone, Copy)]
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
struct Allocator<S> {
    _subject: PhantomData<S>,
}
impl Allocator<CString> {
    fn allocate(string: String) -> CString {
        return CString {
            pointer: unsafe { CString_::from_vec_unchecked(string.into_bytes()) }.into_raw(),
        };
    }
    fn deallocate(c_string: CString) -> () {
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
    fn deallocate(c_vector: CVector<T>) -> () {
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
    fn deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
        if c_result.is_data {
            Allocator::<CVector<_>>::deallocate(c_result.data);
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
    fn transform<O1, P1, O2, P2>(
        c_vector_of_bytes: CVector<c_uchar>,
        converter: impl FnOnce(UnifiedReport<O1, P1>) -> Result<CUnifiedReport<O2, P2>, Box<dyn StdError + 'static>>,
    ) -> CResult<CUnifiedReport<O2, P2>>
    where
        O1: for<'a> Decode<'a>,
        P1: for<'a> Decode<'a>,
        O2: Default,
        P2: Default,
    {
        if c_vector_of_bytes.pointer.is_null() || c_vector_of_bytes.length == 0 {
            return CResult::error();
        }
        let unified_report = match Serializer::deserialize::<'_, UnifiedReport<O1, P1>>(c_vector_of_bytes.as_slice_unchecked()) {
            Result::Ok(unified_report_) => unified_report_,
            Result::Err(_) => {
                return CResult::error();
            }
        };
        let c_unified_report = match converter(unified_report) {
            Result::Ok(c_unified_report_) => c_unified_report_,
            Result::Err(_) => {
                return CResult::error();
            }
        };
        return CResult::data(c_unified_report);
    }
}
impl Transformer<ServerRequestData> {
    fn transform<I1, I2>(incoming: I1, converter: impl for<'a> FnOnce(&'a I1) -> Result<I2, Box<dyn StdError + 'static>>) -> CResult<CVector<c_uchar>>
    where
        I2: Encode,
    {
        let incoming_ = match converter(&incoming) {
            Result::Ok(incoming__) => incoming__,
            Result::Err(_) => {
                return CResult::error();
            }
        };
        return CResult::data(Allocator::<CVector<_>>::allocate(Serializer::serialize(&incoming_)));
    }
}
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct UserAccessTokenSigned {
    pub user_access_token__id: CString,
    pub user__id: c_long,
    pub user_device__id: CString,
    pub user_access_token__expires_at: c_long,
    pub signature: CVector<c_uchar>,
}
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct UserAccessRefreshTokenSigned {
    pub user_access_refresh_token__expires_at: c_long,
    pub signature: CVector<c_uchar>,
}
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ChannelSubscriptionTokenHashed {
    pub channel_subscription_token__expires_at: c_long,
    pub hash: c_ulong,
}
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ChannelTokenHashed {
    pub channel_token__expires_at: c_long,
    pub hash: c_ulong,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_AuthorizeByFirstStep_Incoming {
    pub user_device__id: CString,
    pub user__email___or___user__nickname: CString,
    pub user__password: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__authorize_by_first_step__serialize_allocate(incoming: UserAuthorization_AuthorizeByFirstStep_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ UserAuthorization_AuthorizeByFirstStep_Incoming| -> Result<UserAuthorization_AuthorizeByFirstStep_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            UserAuthorization_AuthorizeByFirstStep_Incoming_ {
                user_device__id: incoming_.user_device__id.get_as_str()?,
                user__email___or___user__nickname: incoming_.user__email___or___user__nickname.get_as_str()?,
                user__password: incoming_.user__password.get_as_str()?,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__authorize_by_first_step__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_AuthorizeByFirstStep_CResult = CResult<CUnifiedReport<UserAuthorization_AuthorizeByFirstStep_Outcoming, UserAuthorization_AuthorizeByFirstStep_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_AuthorizeByFirstStep_Outcoming {
    pub user__id: c_long,
    pub verification_message_sent: bool,
    pub user_authorization_token__can_be_resent_from: c_long,
    pub user_authorization_token__wrong_enter_tries_quantity: c_short,
    pub user_authorization_token__wrong_enter_tries_quantity_limit: c_short,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_AuthorizeByFirstStep_Precedent {
    pub user__wrong_email_or_nickname_or_password: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__authorize_by_first_step__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> UserAuthorization_AuthorizeByFirstStep_CResult {
    let converter = move |unified_report: UnifiedReport<UserAuthorization_AuthorizeByFirstStep_Outcoming_, UserAuthorization_AuthorizeByFirstStep_Precedent_>| -> Result<
        CUnifiedReport<UserAuthorization_AuthorizeByFirstStep_Outcoming, UserAuthorization_AuthorizeByFirstStep_Precedent>,
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
                        let outcoming = UserAuthorization_AuthorizeByFirstStep_Outcoming {
                            user__id: data__.user__id,
                            verification_message_sent: data__.verification_message_sent,
                            user_authorization_token__can_be_resent_from: data__.user_authorization_token__can_be_resent_from,
                            user_authorization_token__wrong_enter_tries_quantity: data__.user_authorization_token__wrong_enter_tries_quantity,
                            user_authorization_token__wrong_enter_tries_quantity_limit: data__.user_authorization_token__wrong_enter_tries_quantity_limit,
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
                    UserAuthorization_AuthorizeByFirstStep_Precedent_::User_WrongEmailOrNicknameOrPassword => {}
                };
                let precedent_ = UserAuthorization_AuthorizeByFirstStep_Precedent {
                    user__wrong_email_or_nickname_or_password: true,
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__authorize_by_first_step__deserialize_deallocate(_c_result: UserAuthorization_AuthorizeByFirstStep_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_AuthorizeByLastStep_Incoming {
    pub user__id: c_long,
    pub user_device__id: CString,
    pub user_authorization_token__value: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__authorize_by_last_step__serialize_allocate(incoming: UserAuthorization_AuthorizeByLastStep_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ UserAuthorization_AuthorizeByLastStep_Incoming| -> Result<UserAuthorization_AuthorizeByLastStep_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            UserAuthorization_AuthorizeByLastStep_Incoming_ {
                user__id: incoming_.user__id,
                user_device__id: incoming_.user_device__id.get_as_str()?,
                user_authorization_token__value: incoming_.user_authorization_token__value.get_as_str()?,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__authorize_by_last_step__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_AuthorizeByLastStep_CResult = CResult<CUnifiedReport<UserAuthorization_AuthorizeByLastStep_Outcoming, UserAuthorization_AuthorizeByLastStep_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_AuthorizeByLastStep_Outcoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub user_access_refresh_token_signed: UserAccessRefreshTokenSigned,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_AuthorizeByLastStep_Precedent {
    pub user_authorization_token__not_found: bool,
    pub user_authorization_token__already_expired: bool,
    pub user_authorization_token__wrong_value: UserAuthorizationToken_WrongValue,
    pub user__not_found: bool,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorizationToken_WrongValue {
    pub is_exist: bool,
    pub user_authorization_token__wrong_enter_tries_quantity: c_short,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__authorize_by_last_step__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> UserAuthorization_AuthorizeByLastStep_CResult {
    let converter = move |unified_report: UnifiedReport<UserAuthorization_AuthorizeByLastStep_Outcoming_, UserAuthorization_AuthorizeByLastStep_Precedent_>| -> Result<
        CUnifiedReport<UserAuthorization_AuthorizeByLastStep_Outcoming, UserAuthorization_AuthorizeByLastStep_Precedent>,
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
                        let outcoming = UserAuthorization_AuthorizeByLastStep_Outcoming {
                            user_access_token_signed: UserAccessTokenSigned {
                                user_access_token__id: Allocator::<CString>::allocate(data__.user_access_token_signed.user_access_token__id),
                                user__id: data__.user_access_token_signed.user__id,
                                user_device__id: Allocator::<CString>::allocate(data__.user_access_token_signed.user_device__id),
                                user_access_token__expires_at: data__.user_access_token_signed.user_access_token__expires_at,
                                signature: Allocator::<CVector<_>>::allocate(data__.user_access_token_signed.signature),
                            },
                            user_access_refresh_token_signed: UserAccessRefreshTokenSigned {
                                user_access_refresh_token__expires_at: data__.user_access_refresh_token_signed.user_access_refresh_token__expires_at,
                                signature: Allocator::<CVector<_>>::allocate(data__.user_access_refresh_token_signed.signature),
                            },
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken_AlreadyExpired => {
                        UserAuthorization_AuthorizeByLastStep_Precedent {
                            user_authorization_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken_NotFound => {
                        UserAuthorization_AuthorizeByLastStep_Precedent {
                            user_authorization_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken_WrongValue {
                        user_authorization_token__wrong_enter_tries_quantity,
                    } => {
                        UserAuthorization_AuthorizeByLastStep_Precedent {
                            user_authorization_token__wrong_value: UserAuthorizationToken_WrongValue {
                                is_exist: true,
                                user_authorization_token__wrong_enter_tries_quantity,
                            },
                            ..Default::default()
                        }
                    }
                    UserAuthorization_AuthorizeByLastStep_Precedent_::User_NotFound => {
                        UserAuthorization_AuthorizeByLastStep_Precedent {
                            user__not_found: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__authorize_by_last_step__deserialize_deallocate(c_result: UserAuthorization_AuthorizeByLastStep_CResult) -> () {
    if c_result.is_data && c_result.data.is_target && c_result.data.target.is_filled {
        Allocator::<CString>::deallocate(c_result.data.target.filled.user_access_token_signed.user_access_token__id);
        Allocator::<CString>::deallocate(c_result.data.target.filled.user_access_token_signed.user_device__id);
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.user_access_token_signed.signature);
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.user_access_refresh_token_signed.signature);
    }
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_CheckEmailForExisting_Incoming {
    pub user__email: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__check_email_for_existing__serialize_allocate(incoming: UserAuthorization_CheckEmailForExisting_Incoming) -> CResult<CVector<c_uchar>> {
    let converter =
        move |incoming_: &'_ UserAuthorization_CheckEmailForExisting_Incoming| -> Result<UserAuthorization_CheckEmailForExisting_Incoming_, Box<dyn StdError + 'static>> {
            return Result::Ok(
                UserAuthorization_CheckEmailForExisting_Incoming_ {
                    user__email: incoming_.user__email.get_as_str()?,
                },
            );
        };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__check_email_for_existing__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_CheckEmailForExisting_CResult = CResult<CUnifiedReport<UserAuthorization_CheckEmailForExisting_Outcoming, CVoid>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_CheckEmailForExisting_Outcoming {
    pub result: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__check_email_for_existing__deserialize_allocate(
    c_vector_of_bytes: CVector<c_uchar>,
) -> UserAuthorization_CheckEmailForExisting_CResult {
    let converter = move |unified_report: UnifiedReport<UserAuthorization_CheckEmailForExisting_Outcoming_, Void>| -> Result<CUnifiedReport<UserAuthorization_CheckEmailForExisting_Outcoming, CVoid>, Box<dyn StdError + 'static>> {
        let UnifiedReport::Target { data } = unified_report;
        let data_ = match data {
            Data::Empty => {
                CData::empty()
            }
            Data::Filled { data: data__ } => {
                let outcoming = UserAuthorization_CheckEmailForExisting_Outcoming {
                    result: data__.result,
                };
                CData::filled(outcoming)
            }
        };
        return Result::Ok(CUnifiedReport::target(data_));
    };
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__check_email_for_existing__deserialize_deallocate(_c_result: UserAuthorization_CheckEmailForExisting_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_CheckNicknameForExisting_Incoming {
    pub user__nickname: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__check_nickname_for_existing__serialize_allocate(
    incoming: UserAuthorization_CheckNicknameForExisting_Incoming,
) -> CResult<CVector<c_uchar>> {
    let converter =
        move |incoming_: &'_ UserAuthorization_CheckNicknameForExisting_Incoming| -> Result<UserAuthorization_CheckNicknameForExisting_Incoming_, Box<dyn StdError + 'static>> {
            return Result::Ok(
                UserAuthorization_CheckNicknameForExisting_Incoming_ {
                    user__nickname: incoming_.user__nickname.get_as_str()?,
                },
            );
        };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__check_nickname_for_existing__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_CheckNicknameForExisting_CResult = CResult<CUnifiedReport<UserAuthorization_CheckNicknameForExisting_Outcoming, CVoid>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_CheckNicknameForExisting_Outcoming {
    pub result: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__check_nickname_for_existing__deserialize_allocate(
    c_vector_of_bytes: CVector<c_uchar>,
) -> UserAuthorization_CheckNicknameForExisting_CResult {
    let converter = move |unified_report: UnifiedReport<UserAuthorization_CheckNicknameForExisting_Outcoming_, Void>| -> Result<CUnifiedReport<UserAuthorization_CheckNicknameForExisting_Outcoming, CVoid>, Box<dyn StdError + 'static>> {
        let UnifiedReport::Target { data } = unified_report;
        let data_ = match data {
            Data::Empty => {
                CData::empty()
            }
            Data::Filled { data: data__ } => {
                let outcoming = UserAuthorization_CheckNicknameForExisting_Outcoming {
                    result: data__.result,
                };
                CData::filled(outcoming)
            }
        };
        return Result::Ok(CUnifiedReport::target(data_));
    };
    return Transformer::<ServerResponseData>::transform(
        c_vector_of_bytes,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__check_nickname_for_existing__deserialize_deallocate(_c_result: UserAuthorization_CheckNicknameForExisting_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_DeauthorizeFromAllDevices_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__deauthorize_from_all_devices__serialize_allocate(
    incoming: UserAuthorization_DeauthorizeFromAllDevices_Incoming,
) -> CResult<CVector<c_uchar>> {
    let converter =
        move |incoming_: &'_ UserAuthorization_DeauthorizeFromAllDevices_Incoming| -> Result<UserAuthorization_DeauthorizeFromAllDevices_Incoming_, Box<dyn StdError + 'static>> {
            return Result::Ok(
                UserAuthorization_DeauthorizeFromAllDevices_Incoming_ {
                    user_access_token_signed: UserAccessTokenSigned_ {
                        user_access_token__id: incoming_.user_access_token_signed.user_access_token__id.get_as_str()?,
                        user__id: incoming_.user_access_token_signed.user__id,
                        user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                        user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                        singature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                    },
                },
            );
        };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__deauthorize_from_all_devices__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_DeauthorizeFromAllDevices_CResult = CResult<CUnifiedReport<CVoid, UserAuthorization_DeauthorizeFromAllDevices_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_DeauthorizeFromAllDevices_Precedent {
    pub user_access_token__already_expired: bool,
    pub user_access_token__in_user_access_token_black_list: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__deauthorize_from_all_devices__deserialize_allocate(
    c_vector_of_bytes: CVector<c_uchar>,
) -> UserAuthorization_DeauthorizeFromAllDevices_CResult {
    let converter = move |unified_report: UnifiedReport<Void, UserAuthorization_DeauthorizeFromAllDevices_Precedent_>| -> Result<CUnifiedReport<CVoid, UserAuthorization_DeauthorizeFromAllDevices_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    UserAuthorization_DeauthorizeFromAllDevices_Precedent_::UserAccessToken_AlreadyExpired => {
                        UserAuthorization_DeauthorizeFromAllDevices_Precedent {
                            user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_DeauthorizeFromAllDevices_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {
                        UserAuthorization_DeauthorizeFromAllDevices_Precedent {
                            user_access_token__in_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__deauthorize_from_all_devices__deserialize_deallocate(_c_result: UserAuthorization_DeauthorizeFromAllDevices_CResult) -> () {
    return ();
}
type UserAuthorization_DeauthorizeFromOneDevice_CResult = CResult<CUnifiedReport<CVoid, UserAuthorization_DeauthorizeFromOneDevice_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_DeauthorizeFromOneDevice_Precedent {
    pub user_access_token__already_expired: bool,
    pub user_access_token__in_user_access_token_black_list: bool,
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_DeauthorizeFromOneDevice_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__deauthorize_from_one_device__serialize_allocate(
    incoming: UserAuthorization_DeauthorizeFromOneDevice_Incoming,
) -> CResult<CVector<c_uchar>> {
    let converter =
        move |incoming_: &'_ UserAuthorization_DeauthorizeFromOneDevice_Incoming| -> Result<UserAuthorization_DeauthorizeFromOneDevice_Incoming_, Box<dyn StdError + 'static>> {
            return Result::Ok(
                UserAuthorization_DeauthorizeFromOneDevice_Incoming_ {
                    user_access_token_signed: UserAccessTokenSigned_ {
                        user_access_token__id: incoming_.user_access_token_signed.user_access_token__id.get_as_str()?,
                        user__id: incoming_.user_access_token_signed.user__id,
                        user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                        user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                        singature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                    },
                },
            );
        };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__deauthorize_from_one_device__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__deauthorize_from_one_device__deserialize_allocate(
    c_vector_of_bytes: CVector<c_uchar>,
) -> UserAuthorization_DeauthorizeFromOneDevice_CResult {
    let converter = move |unified_report: UnifiedReport<Void, UserAuthorization_DeauthorizeFromOneDevice_Precedent_>| -> Result<CUnifiedReport<CVoid, UserAuthorization_DeauthorizeFromOneDevice_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    UserAuthorization_DeauthorizeFromOneDevice_Precedent_::UserAccessToken_AlreadyExpired => {
                        UserAuthorization_DeauthorizeFromOneDevice_Precedent {
                            user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_DeauthorizeFromOneDevice_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {
                        UserAuthorization_DeauthorizeFromOneDevice_Precedent {
                            user_access_token__in_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__deauthorize_from_one_device__deserialize_deallocate(_c_result: UserAuthorization_DeauthorizeFromOneDevice_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_RefreshAccessToken_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub user_access_refresh_token_signed: UserAccessRefreshTokenSigned,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__refresh_access_token__serialize_allocate(incoming: UserAuthorization_RefreshAccessToken_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ UserAuthorization_RefreshAccessToken_Incoming| -> Result<UserAuthorization_RefreshAccessToken_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            UserAuthorization_RefreshAccessToken_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user_access_token__id: incoming_.user_access_token_signed.user_access_token__id.get_as_str()?,
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    singature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                user_access_refresh_token_signed: UserAccessRefreshTokenSigned_ {
                    user_access_refresh_token__expires_at: incoming_.user_access_refresh_token_signed.user_access_refresh_token__expires_at,
                    signature: incoming_.user_access_refresh_token_signed.signature.clone_as_vec()?,
                },
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__refresh_access_token__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_RefreshAccessToken_CResult = CResult<CUnifiedReport<UserAuthorization_RefreshAccessToken_Outcoming, UserAuthorization_RefreshAccessToken_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_RefreshAccessToken_Outcoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub user_access_refresh_token_signed: UserAccessRefreshTokenSigned,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_RefreshAccessToken_Precedent {
    pub user_access_refresh_token__not_found: bool,
    pub user_access_refresh_token__already_expired: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__refresh_access_token__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> UserAuthorization_RefreshAccessToken_CResult {
    let converter = move |unified_report: UnifiedReport<UserAuthorization_RefreshAccessToken_Outcoming_, UserAuthorization_RefreshAccessToken_Precedent_>| -> Result<
        CUnifiedReport<UserAuthorization_RefreshAccessToken_Outcoming, UserAuthorization_RefreshAccessToken_Precedent>,
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
                        let outcoming = UserAuthorization_RefreshAccessToken_Outcoming {
                            user_access_token_signed: UserAccessTokenSigned {
                                user_access_token__id: Allocator::<CString>::allocate(data__.user_access_token_signed.user_access_token__id),
                                user__id: data__.user_access_token_signed.user__id,
                                user_device__id: Allocator::<CString>::allocate(data__.user_access_token_signed.user_device__id),
                                user_access_token__expires_at: data__.user_access_token_signed.user_access_token__expires_at,
                                signature: Allocator::<CVector<_>>::allocate(data__.user_access_token_signed.signature),
                            },
                            user_access_refresh_token_signed: UserAccessRefreshTokenSigned {
                                user_access_refresh_token__expires_at: data__.user_access_refresh_token_signed.user_access_refresh_token__expires_at,
                                signature: Allocator::<CVector<_>>::allocate(data__.user_access_refresh_token_signed.signature),
                            },
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken_AlreadyExpired => {
                        UserAuthorization_RefreshAccessToken_Precedent {
                            user_access_refresh_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken_NotFound => {
                        UserAuthorization_RefreshAccessToken_Precedent {
                            user_access_refresh_token__not_found: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__refresh_access_token__deserialize_deallocate(c_result: UserAuthorization_RefreshAccessToken_CResult) -> () {
    if c_result.is_data && c_result.data.is_target && c_result.data.target.is_filled {
        Allocator::<CString>::deallocate(c_result.data.target.filled.user_access_token_signed.user_access_token__id);
        Allocator::<CString>::deallocate(c_result.data.target.filled.user_access_token_signed.user_device__id);
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.user_access_token_signed.signature);
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.user_access_refresh_token_signed.signature);
    }
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_RegisterByFirstStep_Incoming {
    pub user__email: CString,
    pub user_device__id: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__register_by_first_step__serialize_allocate(incoming: UserAuthorization_RegisterByFirstStep_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ UserAuthorization_RegisterByFirstStep_Incoming| -> Result<UserAuthorization_RegisterByFirstStep_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            UserAuthorization_RegisterByFirstStep_Incoming_ {
                user__email: incoming_.user__email.get_as_str()?,
                user_device__id: incoming_.user_device__id.get_as_str()?,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__register_by_first_step__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__register_by_first_step__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> UserAuthorization_RegisterByFirstStep_CResult {
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__register_by_first_step__deserialize_deallocate(_c_result: UserAuthorization_RegisterByFirstStep_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_RegisterBySecondStep_Incoming {
    pub user__email: CString,
    pub user_device__id: CString,
    pub user_registration_token__value: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__register_by_second_step__serialize_allocate(incoming: UserAuthorization_RegisterBySecondStep_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ UserAuthorization_RegisterBySecondStep_Incoming| -> Result<UserAuthorization_RegisterBySecondStep_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            UserAuthorization_RegisterBySecondStep_Incoming_ {
                user__email: incoming_.user__email.get_as_str()?,
                user_device__id: incoming_.user_device__id.get_as_str()?,
                user_registration_token__value: incoming_.user_registration_token__value.get_as_str()?,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__register_by_second_step__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_RegisterBySecondStep_CResult = CResult<CUnifiedReport<CVoid, UserAuthorization_RegisterBySecondStep_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_RegisterBySecondStep_Precedent {
    pub user_registration_token__not_found: bool,
    pub user_registration_token__already_expired: bool,
    pub user_registration_token__already_approved: bool,
    pub user_registration_token__wrong_value: UserRegistrationToken_WrongValue,
}
#[repr(C)]
#[derive(Default)]
pub struct UserRegistrationToken_WrongValue {
    pub is_exist: bool,
    pub user_registration_token__wrong_enter_tries_quantity: c_short,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__register_by_second_step__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> UserAuthorization_RegisterBySecondStep_CResult {
    let converter = move |unified_report: UnifiedReport<Void, UserAuthorization_RegisterBySecondStep_Precedent_>| -> Result<CUnifiedReport<CVoid, UserAuthorization_RegisterBySecondStep_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken_NotFound => {
                        UserAuthorization_RegisterBySecondStep_Precedent {
                            user_registration_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken_AlreadyExpired => {
                        UserAuthorization_RegisterBySecondStep_Precedent {
                            user_registration_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken_AlreadyApproved => {
                        UserAuthorization_RegisterBySecondStep_Precedent {
                            user_registration_token__already_approved: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken_WrongValue { user_registration_token__wrong_enter_tries_quantity } => {
                        UserAuthorization_RegisterBySecondStep_Precedent {
                            user_registration_token__wrong_value: UserRegistrationToken_WrongValue {
                                is_exist: true,
                                user_registration_token__wrong_enter_tries_quantity,
                            },
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__register_by_second_step__deserialize_deallocate(_c_result: UserAuthorization_RegisterBySecondStep_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_RegisterByLastStep_Incoming {
    pub user_device__id: CString,
    pub user__nickname: CString,
    pub user__password: CString,
    pub user__email: CString,
    pub user_registration_token__value: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__register_by_last_step__serialize_allocate(incoming: UserAuthorization_RegisterByLastStep_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ UserAuthorization_RegisterByLastStep_Incoming| -> Result<UserAuthorization_RegisterByLastStep_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            UserAuthorization_RegisterByLastStep_Incoming_ {
                user_device__id: incoming_.user_device__id.get_as_str()?,
                user__email: incoming_.user__email.get_as_str()?,
                user__nickname: incoming_.user__nickname.get_as_str()?,
                user__password: incoming_.user__password.get_as_str()?,
                user_registration_token__value: incoming_.user_registration_token__value.get_as_str()?,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__register_by_last_step__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_RegisterByLastStep_CResult = CResult<CUnifiedReport<UserAuthorization_RegisterByLastStep_Outcoming, UserAuthorization_RegisterByLastStep_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_RegisterByLastStep_Outcoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub user_access_refresh_token_signed: UserAccessRefreshTokenSigned,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_RegisterByLastStep_Precedent {
    pub user__nickname_already_exist: bool,
    pub user__email_already_exist: bool,
    pub user_registration_token__not_found: bool,
    pub user_registration_token__already_expired: bool,
    pub user_registration_token__is_not_approved: bool,
    pub user_registration_token__wrong_value: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__register_by_last_step__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> UserAuthorization_RegisterByLastStep_CResult {
    let converter = move |unified_report: UnifiedReport<UserAuthorization_RegisterByLastStep_Outcoming_, UserAuthorization_RegisterByLastStep_Precedent_>| -> Result<
        CUnifiedReport<UserAuthorization_RegisterByLastStep_Outcoming, UserAuthorization_RegisterByLastStep_Precedent>,
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
                        let outcoming = UserAuthorization_RegisterByLastStep_Outcoming {
                            user_access_token_signed: UserAccessTokenSigned {
                            user_access_token__id: Allocator::<CString>::allocate(data__.user_access_token_signed.user_access_token__id),
                            user__id: data__.user_access_token_signed.user__id,
                            user_device__id: Allocator::<CString>::allocate(data__.user_access_token_signed.user_device__id),
                            user_access_token__expires_at: data__.user_access_token_signed.user_access_token__expires_at,
                            signature: Allocator::<CVector<_>>::allocate(data__.user_access_token_signed.signature),
                            },
                            user_access_refresh_token_signed: UserAccessRefreshTokenSigned {
                                user_access_refresh_token__expires_at: data__.user_access_refresh_token_signed.user_access_refresh_token__expires_at,
                                signature: Allocator::<CVector<_>>::allocate(data__.user_access_refresh_token_signed.signature),
                            },
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_RegisterByLastStep_Precedent_::User_NicknameAlreadyExist => {
                        UserAuthorization_RegisterByLastStep_Precedent {
                            user__nickname_already_exist: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterByLastStep_Precedent_::User_EmailAlreadyExist => {
                        UserAuthorization_RegisterByLastStep_Precedent {
                            user__email_already_exist: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken_NotFound => {
                        UserAuthorization_RegisterByLastStep_Precedent {
                            user_registration_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken_AlreadyExpired => {
                        UserAuthorization_RegisterByLastStep_Precedent {
                            user_registration_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken_IsNotApproved => {
                        UserAuthorization_RegisterByLastStep_Precedent {
                            user_registration_token__is_not_approved: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken_WrongValue => {
                        UserAuthorization_RegisterByLastStep_Precedent {
                            user_registration_token__wrong_value: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__register_by_last_step__deserialize_deallocate(c_result: UserAuthorization_RegisterByLastStep_CResult) -> () {
    if c_result.is_data && c_result.data.is_target && c_result.data.target.is_filled {
        Allocator::<CString>::deallocate(c_result.data.target.filled.user_access_token_signed.user_access_token__id);
        Allocator::<CString>::deallocate(c_result.data.target.filled.user_access_token_signed.user_device__id);
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.user_access_token_signed.signature);
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.user_access_refresh_token_signed.signature);
    }
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_ResetPasswordByFirstStep_Incoming {
    pub user__email: CString,
    pub user_device__id: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__reset_password_by_first_step__serialize_allocate(
    incoming: UserAuthorization_ResetPasswordByFirstStep_Incoming,
) -> CResult<CVector<c_uchar>> {
    let converter =
        move |incoming_: &'_ UserAuthorization_ResetPasswordByFirstStep_Incoming| -> Result<UserAuthorization_ResetPasswordByFirstStep_Incoming_, Box<dyn StdError + 'static>> {
            return Result::Ok(
                UserAuthorization_ResetPasswordByFirstStep_Incoming_ {
                    user__email: incoming_.user__email.get_as_str()?,
                    user_device__id: incoming_.user_device__id.get_as_str()?,
                },
            );
        };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__reset_password_by_first_step__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_ResetPasswordByFirstStep_CResult =
    CResult<CUnifiedReport<UserAuthorization_ResetPasswordByFirstStep_Outcoming, UserAuthorization_ResetPasswordByFirstStep_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_ResetPasswordByFirstStep_Outcoming {
    pub user__id: c_long,
    pub verification_message_sent: bool,
    pub user_reset_password_token__can_be_resent_from: c_long,
    pub user_reset_password_token__wrong_enter_tries_quantity: c_short,
    pub user_reset_password_token__wrong_enter_tries_quantity_limit: c_short,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_ResetPasswordByFirstStep_Precedent {
    pub user__not_found: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__reset_password_by_first_step__deserialize_allocate(
    c_vector_of_bytes: CVector<c_uchar>,
) -> UserAuthorization_ResetPasswordByFirstStep_CResult {
    let converter = move |unified_report: UnifiedReport<UserAuthorization_ResetPasswordByFirstStep_Outcoming_, UserAuthorization_ResetPasswordByFirstStep_Precedent_>| -> Result<
        CUnifiedReport<UserAuthorization_ResetPasswordByFirstStep_Outcoming, UserAuthorization_ResetPasswordByFirstStep_Precedent>,
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
                        let outcoming = UserAuthorization_ResetPasswordByFirstStep_Outcoming {
                            user__id: data__.user__id,
                            verification_message_sent: data__.verification_message_sent,
                            user_reset_password_token__can_be_resent_from: data__.user_reset_password_token__can_be_resent_from,
                            user_reset_password_token__wrong_enter_tries_quantity: data__.user_reset_password_token__wrong_enter_tries_quantity,
                            user_reset_password_token__wrong_enter_tries_quantity_limit: data__.user_reset_password_token__wrong_enter_tries_quantity_limit,
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
                    UserAuthorization_ResetPasswordByFirstStep_Precedent_::User_NotFound => {}
                };
                let precedent_ = UserAuthorization_ResetPasswordByFirstStep_Precedent {
                    user__not_found: true,
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__reset_password_by_first_step__deserialize_deallocate(_c_result: UserAuthorization_ResetPasswordByFirstStep_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_ResetPasswordBySecondStep_Incoming {
    pub user__id: c_long,
    pub user_device__id: CString,
    pub user_reset_password_token__value: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__reset_password_by_second_step__serialize_allocate(
    incoming: UserAuthorization_ResetPasswordBySecondStep_Incoming,
) -> CResult<CVector<c_uchar>> {
    let converter =
        move |incoming_: &'_ UserAuthorization_ResetPasswordBySecondStep_Incoming| -> Result<UserAuthorization_ResetPasswordBySecondStep_Incoming_, Box<dyn StdError + 'static>> {
            return Result::Ok(
                UserAuthorization_ResetPasswordBySecondStep_Incoming_ {
                    user__id: incoming_.user__id,
                    user_device__id: incoming_.user_device__id.get_as_str()?,
                    user_reset_password_token__value: incoming_.user_reset_password_token__value.get_as_str()?,
                },
            );
        };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__reset_password_by_second_step__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_ResetPasswordBySecondStep_CResult = CResult<CUnifiedReport<CVoid, UserAuthorization_ResetPasswordBySecondStep_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_ResetPasswordBySecondStep_Precedent {
    pub user_reset_password_token__not_found: bool,
    pub user_reset_password_token__already_expired: bool,
    pub user_reset_password_token__already_approved: bool,
    pub user_reset_password_token__wrong_value: UserResetPasswordToken_WrongValue,
}
#[repr(C)]
#[derive(Default)]
pub struct UserResetPasswordToken_WrongValue {
    pub is_exist: bool,
    pub user_reset_password_token__wrong_enter_tries_quantity: c_short,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__reset_password_by_second_step__deserialize_allocate(
    c_vector_of_bytes: CVector<c_uchar>,
) -> UserAuthorization_ResetPasswordBySecondStep_CResult {
    let converter = move |unified_report: UnifiedReport<Void, UserAuthorization_ResetPasswordBySecondStep_Precedent_>| -> Result<CUnifiedReport<CVoid, UserAuthorization_ResetPasswordBySecondStep_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken_NotFound => {
                        UserAuthorization_ResetPasswordBySecondStep_Precedent {
                            user_reset_password_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken_AlreadyExpired => {
                        UserAuthorization_ResetPasswordBySecondStep_Precedent {
                            user_reset_password_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken_AlreadyApproved => {
                        UserAuthorization_ResetPasswordBySecondStep_Precedent {
                            user_reset_password_token__already_approved: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken_WrongValue { user_reset_password_token__wrong_enter_tries_quantity } => {
                        UserAuthorization_ResetPasswordBySecondStep_Precedent {
                            user_reset_password_token__wrong_value: UserResetPasswordToken_WrongValue {
                                is_exist: true,
                                user_reset_password_token__wrong_enter_tries_quantity,
                            },
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__reset_password_by_second_step__deserialize_deallocate(_c_result: UserAuthorization_ResetPasswordBySecondStep_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_ResetPasswordByLastStep_Incoming {
    pub user__id: c_long,
    pub user_device__id: CString,
    pub user__password: CString,
    pub user_reset_password_token__value: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__reset_password_by_last_step__serialize_allocate(
    incoming: UserAuthorization_ResetPasswordByLastStep_Incoming,
) -> CResult<CVector<c_uchar>> {
    let converter =
        move |incoming_: &'_ UserAuthorization_ResetPasswordByLastStep_Incoming| -> Result<UserAuthorization_ResetPasswordByLastStep_Incoming_, Box<dyn StdError + 'static>> {
            return Result::Ok(
                UserAuthorization_ResetPasswordByLastStep_Incoming_ {
                    user__id: incoming_.user__id,
                    user_device__id: incoming_.user_device__id.get_as_str()?,
                    user__password: incoming_.user__password.get_as_str()?,
                    user_reset_password_token__value: incoming_.user_reset_password_token__value.get_as_str()?,
                },
            );
        };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__reset_password_by_last_step__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_ResetPasswordByLastStep_CResult = CResult<CUnifiedReport<CVoid, UserAuthorization_ResetPasswordByLastStep_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_ResetPasswordByLastStep_Precedent {
    pub user__not_found: bool,
    pub user_reset_password_token__not_found: bool,
    pub user_reset_password_token__already_expired: bool,
    pub user_reset_password_token__is_not_approved: bool,
    pub user_reset_password_token__wrong_value: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__reset_password_by_last_step__deserialize_allocate(
    c_vector_of_bytes: CVector<c_uchar>,
) -> UserAuthorization_ResetPasswordByLastStep_CResult {
    let converter = move |unified_report: UnifiedReport<Void, UserAuthorization_ResetPasswordByLastStep_Precedent_>| -> Result<CUnifiedReport<CVoid, UserAuthorization_ResetPasswordByLastStep_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::User_NotFound => {
                        UserAuthorization_ResetPasswordByLastStep_Precedent {
                            user__not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken_NotFound => {
                        UserAuthorization_ResetPasswordByLastStep_Precedent {
                            user_reset_password_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken_AlreadyExpired => {
                        UserAuthorization_ResetPasswordByLastStep_Precedent {
                            user_reset_password_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken_IsNotApproved => {
                        UserAuthorization_ResetPasswordByLastStep_Precedent {
                            user_reset_password_token__is_not_approved: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken_WrongValue => {
                        UserAuthorization_ResetPasswordByLastStep_Precedent {
                            user_reset_password_token__wrong_value: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__reset_password_by_last_step__deserialize_deallocate(_c_result: UserAuthorization_ResetPasswordByLastStep_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_SendEmailForRegister_Incoming {
    pub user__email: CString,
    pub user_device__id: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__send_email_for_register__serialize_allocate(incoming: UserAuthorization_SendEmailForRegister_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ UserAuthorization_SendEmailForRegister_Incoming| -> Result<UserAuthorization_SendEmailForRegister_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            UserAuthorization_SendEmailForRegister_Incoming_ {
                user__email: incoming_.user__email.get_as_str()?,
                user_device__id: incoming_.user_device__id.get_as_str()?,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__send_email_for_register__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_SendEmailForRegister_CResult = CResult<CUnifiedReport<UserAuthorization_SendEmailForRegister_Outcoming, UserAuthorization_SendEmailForRegister_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_SendEmailForRegister_Outcoming {
    pub user_registration_token__can_be_resent_from: c_long,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_SendEmailForRegister_Precedent {
    pub user_registration_token__not_found: bool,
    pub user_registration_token__already_expired: bool,
    pub user_registration_token__already_approved: bool,
    pub user_registration_token__time_to_resend_has_not_come: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__send_email_for_register__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> UserAuthorization_SendEmailForRegister_CResult {
    let converter = move |unified_report: UnifiedReport<UserAuthorization_SendEmailForRegister_Outcoming_, UserAuthorization_SendEmailForRegister_Precedent_>| -> Result<
        CUnifiedReport<UserAuthorization_SendEmailForRegister_Outcoming, UserAuthorization_SendEmailForRegister_Precedent>,
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
                        let outcoming = UserAuthorization_SendEmailForRegister_Outcoming {
                            user_registration_token__can_be_resent_from: data__.user_registration_token__can_be_resent_from,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken_NotFound => {
                        UserAuthorization_SendEmailForRegister_Precedent {
                            user_registration_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken_AlreadyExpired => {
                        UserAuthorization_SendEmailForRegister_Precedent {
                            user_registration_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken_AlreadyApproved => {
                        UserAuthorization_SendEmailForRegister_Precedent {
                            user_registration_token__already_approved: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken_TimeToResendHasNotCome => {
                        UserAuthorization_SendEmailForRegister_Precedent {
                            user_registration_token__time_to_resend_has_not_come: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__send_email_for_register__deserialize_deallocate(_c_result: UserAuthorization_SendEmailForRegister_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_SendEmailForAuthorize_Incoming {
    pub user_device__id: CString,
    pub user__id: c_long,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__send_email_for_authorize__serialize_allocate(incoming: UserAuthorization_SendEmailForAuthorize_Incoming) -> CResult<CVector<c_uchar>> {
    let converter =
        move |incoming_: &'_ UserAuthorization_SendEmailForAuthorize_Incoming| -> Result<UserAuthorization_SendEmailForAuthorize_Incoming_, Box<dyn StdError + 'static>> {
            return Result::Ok(
                UserAuthorization_SendEmailForAuthorize_Incoming_ {
                    user_device__id: incoming_.user_device__id.get_as_str()?,
                    user__id: incoming_.user__id,
                },
            );
        };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__send_email_for_authorize__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_SendEmailForAuthorize_CResult =
    CResult<CUnifiedReport<UserAuthorization_SendEmailForAuthorize_Outcoming, UserAuthorization_SendEmailForAuthorize_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_SendEmailForAuthorize_Outcoming {
    pub user_authorization_token__can_be_resent_from: c_long,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_SendEmailForAuthorize_Precedent {
    pub user__not_found: bool,
    pub user_authorization_token__not_found: bool,
    pub user_authorization_token__already_expired: bool,
    pub user_authorization_token__time_to_resend_has_not_come: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__send_email_for_authorize__deserialize_allocate(
    c_vector_of_bytes: CVector<c_uchar>,
) -> UserAuthorization_SendEmailForAuthorize_CResult {
    let converter = move |unified_report: UnifiedReport<UserAuthorization_SendEmailForAuthorize_Outcoming_, UserAuthorization_SendEmailForAuthorize_Precedent_>| -> Result<
        CUnifiedReport<UserAuthorization_SendEmailForAuthorize_Outcoming, UserAuthorization_SendEmailForAuthorize_Precedent>,
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
                        let outcoming = UserAuthorization_SendEmailForAuthorize_Outcoming {
                            user_authorization_token__can_be_resent_from: data__.user_authorization_token__can_be_resent_from,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_SendEmailForAuthorize_Precedent_::User_NotFound => {
                        UserAuthorization_SendEmailForAuthorize_Precedent {
                            user__not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken_NotFound => {
                        UserAuthorization_SendEmailForAuthorize_Precedent {
                            user_authorization_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken_AlreadyExpired => {
                        UserAuthorization_SendEmailForAuthorize_Precedent {
                            user_authorization_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken_TimeToResendHasNotCome => {
                        UserAuthorization_SendEmailForAuthorize_Precedent {
                            user_authorization_token__time_to_resend_has_not_come: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__send_email_for_authorize__deserialize_deallocate(_c_result: UserAuthorization_SendEmailForAuthorize_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UserAuthorization_SendEmailForResetPassword_Incoming {
    pub user__id: c_long,
    pub user_device__id: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__send_email_for_reset_password__serialize_allocate(
    incoming: UserAuthorization_SendEmailForResetPassword_Incoming,
) -> CResult<CVector<c_uchar>> {
    let converter =
        move |incoming_: &'_ UserAuthorization_SendEmailForResetPassword_Incoming| -> Result<UserAuthorization_SendEmailForResetPassword_Incoming_, Box<dyn StdError + 'static>> {
            return Result::Ok(
                UserAuthorization_SendEmailForResetPassword_Incoming_ {
                    user__id: incoming_.user__id,
                    user_device__id: incoming_.user_device__id.get_as_str()?,
                },
            );
        };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__send_email_for_reset_password__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type UserAuthorization_SendEmailForResetPassword_CResult =
    CResult<CUnifiedReport<UserAuthorization_SendEmailForResetPassword_Outcoming, UserAuthorization_SendEmailForResetPassword_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_SendEmailForResetPassword_Outcoming {
    pub user_resep_password_token_can_be_resent_from: c_long,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_SendEmailForResetPassword_Precedent {
    pub user__not_found: bool,
    pub user_reset_password_token__not_found: bool,
    pub user_reset_password_token__already_expired: bool,
    pub user_reset_password_token__already_approved: bool,
    pub user_reset_password_token__time_to_resend_has_not_come: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__send_email_for_reset_password__deserialize_allocate(
    c_vector_of_bytes: CVector<c_uchar>,
) -> UserAuthorization_SendEmailForResetPassword_CResult {
    let converter = move |unified_report: UnifiedReport<UserAuthorization_SendEmailForResetPassword_Outcoming_, UserAuthorization_SendEmailForResetPassword_Precedent_>| -> Result<
        CUnifiedReport<UserAuthorization_SendEmailForResetPassword_Outcoming, UserAuthorization_SendEmailForResetPassword_Precedent>,
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
                        let outcoming = UserAuthorization_SendEmailForResetPassword_Outcoming {
                            user_resep_password_token_can_be_resent_from: data__.user_reset_password_token__can_be_resent_from,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_SendEmailForResetPassword_Precedent_::User_NotFound => {
                        UserAuthorization_SendEmailForResetPassword_Precedent {
                            user__not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken_NotFound => {
                        UserAuthorization_SendEmailForResetPassword_Precedent {
                            user_reset_password_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken_AlreadyExpired => {
                        UserAuthorization_SendEmailForResetPassword_Precedent {
                            user_reset_password_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken_AlreadyApproved => {
                        UserAuthorization_SendEmailForResetPassword_Precedent {
                            user_reset_password_token__already_approved: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken_TimeToResendHasNotCome => {
                        UserAuthorization_SendEmailForResetPassword_Precedent {
                            user_reset_password_token__time_to_resend_has_not_come: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__send_email_for_reset_password__deserialize_deallocate(_c_result: UserAuthorization_SendEmailForResetPassword_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Channel_GetManyByNameInSubscriptions_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel__name: CString,
    pub requery___channel__name: COption<CString>,
    pub limit: c_short,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_many_by_name_in_subscriptions__serialize_allocate(incoming: Channel_GetManyByNameInSubscriptions_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ Channel_GetManyByNameInSubscriptions_Incoming| -> Result<Channel_GetManyByNameInSubscriptions_Incoming_, Box<dyn StdError + 'static>> {
        let requery___channel__name = if incoming_.requery___channel__name.is_data {
            Option::Some(incoming_.requery___channel__name.data.get_as_str()?)
        } else {
            Option::None
        };
        return Result::Ok(
            Channel_GetManyByNameInSubscriptions_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user_access_token__id: incoming_.user_access_token_signed.user_access_token__id.get_as_str()?,
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    singature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel__name: incoming_.channel__name.get_as_str()?,
                requery___channel__name,
                limit: incoming_.limit,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_many_by_name_in_subscriptions__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type Channel_GetManyByNameInSubscriptions_CResult = CResult<CUnifiedReport<Channel_GetManyByNameInSubscriptions_Outcoming, Channel_GetManyByNameInSubscriptions_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyByNameInSubscriptions_Data {
    pub channel__id: c_long,
    pub channel__name: CString,
    pub channel__linked_name: CString,
    pub channel__access_modifier: c_short,
    pub channel__visability_modifier: c_short,
    pub channel__cover_image_path: COption<CString>,
    pub channel__background_image_path: COption<CString>,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyByNameInSubscriptions_Outcoming {
    pub data_registry: CVector<Channel_GetManyByNameInSubscriptions_Data>,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyByNameInSubscriptions_Precedent {
    pub user_access_token__already_expired: bool,
    pub user_access_token__in_user_access_token_black_list: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_many_by_name_in_subscriptions__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> Channel_GetManyByNameInSubscriptions_CResult {
    let converter = move |unified_report: UnifiedReport<Channel_GetManyByNameInSubscriptions_Outcoming_, Channel_GetManyByNameInSubscriptions_Precedent_>| -> Result<
        CUnifiedReport<Channel_GetManyByNameInSubscriptions_Outcoming, Channel_GetManyByNameInSubscriptions_Precedent>,
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
                        let mut data_registry: Vec<Channel_GetManyByNameInSubscriptions_Data> = vec![];
                        '_a: for data___ in data__.data_registry {
                            let channel__cover_image_path = match data___.channel__cover_image_path {
                                Option::Some(channel__cover_image_path_) => COption::data(Allocator::<CString>::allocate(channel__cover_image_path_)),
                                Option::None => COption::none(),
                            };
                            let channel__background_image_path = match data___.channel__background_image_path {
                                Option::Some(channel__background_image_path_) => COption::data(Allocator::<CString>::allocate(channel__background_image_path_)),
                                Option::None => COption::none(),
                            };
                            let data_ = Channel_GetManyByNameInSubscriptions_Data {
                                channel__id: data___.channel__id,
                                channel__name: Allocator::<CString>::allocate(data___.channel__name),
                                channel__linked_name: Allocator::<CString>::allocate(data___.channel__linked_name),
                                channel__access_modifier: data___.channel__access_modifier,
                                channel__visability_modifier: data___.channel__visability_modifier,
                                channel__cover_image_path,
                                channel__background_image_path,
                            };
                            data_registry.push(data_);
                        }
                        let outcoming = Channel_GetManyByNameInSubscriptions_Outcoming {
                            data_registry: Allocator::<CVector<_>>::allocate(data_registry),
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    Channel_GetManyByNameInSubscriptions_Precedent_::UserAccessToken_AlreadyExpired => {
                        Channel_GetManyByNameInSubscriptions_Precedent {
                            user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Channel_GetManyByNameInSubscriptions_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {
                        Channel_GetManyByNameInSubscriptions_Precedent {
                            user_access_token__in_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_many_by_name_in_subscriptions__deserialize_deallocate(c_result: Channel_GetManyByNameInSubscriptions_CResult) -> () {
    if c_result.is_data && c_result.data.is_target && c_result.data.target.is_filled {
        let data_registry = c_result.data.target.filled.data_registry.as_slice_unchecked();
        for data in data_registry {
            Allocator::<CString>::deallocate(data.channel__name);
            Allocator::<CString>::deallocate(data.channel__linked_name);
            if data.channel__background_image_path.is_data {
                Allocator::<CString>::deallocate(data.channel__background_image_path.data);
            }
            if data.channel__cover_image_path.is_data {
                Allocator::<CString>::deallocate(data.channel__cover_image_path.data);
            }
        }
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.data_registry);
    }
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Channel_GetManyBySubscription_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub requery___channel__id: COption<c_long>,
    pub limit: c_short,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_many_by_subscription__serialize_allocate(incoming: Channel_GetManyBySubscription_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ Channel_GetManyBySubscription_Incoming| -> Result<Channel_GetManyBySubscription_Incoming_, Box<dyn StdError + 'static>> {
        let requery___channel__id = if incoming_.requery___channel__id.is_data {
            Option::Some(incoming_.requery___channel__id.data)
        } else {
            Option::None
        };
        return Result::Ok(
            Channel_GetManyBySubscription_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user_access_token__id: incoming_.user_access_token_signed.user_access_token__id.get_as_str()?,
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    singature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                requery___channel__id,
                limit: incoming_.limit,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_many_by_subscription__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type Channel_GetManyBySubscription_CResult = CResult<CUnifiedReport<Channel_GetManyBySubscription_Outcoming, Channel_GetManyBySubscription_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyBySubscription_Data {
    pub channel__id: c_long,
    pub channel__name: CString,
    pub channel__linked_name: CString,
    pub channel__access_modifier: c_short,
    pub channel__visability_modifier: c_short,
    pub channel__cover_image_path: COption<CString>,
    pub channel__background_image_path: COption<CString>,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyBySubscription_Outcoming {
    pub data_registry: CVector<Channel_GetManyBySubscription_Data>,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyBySubscription_Precedent {
    pub user_access_token__already_expired: bool,
    pub user_access_token__in_user_access_token_black_list: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_many_by_subscription__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> Channel_GetManyBySubscription_CResult {
    let converter = move |unified_report: UnifiedReport<Channel_GetManyBySubscription_Outcoming_, Channel_GetManyBySubscription_Precedent_>| -> Result<
        CUnifiedReport<Channel_GetManyBySubscription_Outcoming, Channel_GetManyBySubscription_Precedent>,
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
                        let mut data_registry: Vec<Channel_GetManyBySubscription_Data> = vec![];
                        '_a: for data___ in data__.data_registry {
                            let channel__cover_image_path = match data___.channel__cover_image_path {
                                Option::Some(channel__cover_image_path_) => COption::data(Allocator::<CString>::allocate(channel__cover_image_path_)),
                                Option::None => COption::none(),
                            };
                            let channel__background_image_path = match data___.channel__background_image_path {
                                Option::Some(channel__background_image_path_) => COption::data(Allocator::<CString>::allocate(channel__background_image_path_)),
                                Option::None => COption::none(),
                            };
                            let data_ = Channel_GetManyBySubscription_Data {
                                channel__id: data___.channel__id,
                                channel__name: Allocator::<CString>::allocate(data___.channel__name),
                                channel__linked_name: Allocator::<CString>::allocate(data___.channel__linked_name),
                                channel__access_modifier: data___.channel__access_modifier,
                                channel__visability_modifier: data___.channel__visability_modifier,
                                channel__cover_image_path,
                                channel__background_image_path,
                            };
                            data_registry.push(data_);
                        }
                        let outcoming = Channel_GetManyBySubscription_Outcoming {
                            data_registry: Allocator::<CVector<_>>::allocate(data_registry),
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    Channel_GetManyBySubscription_Precedent_::UserAccessToken_AlreadyExpired => {
                        Channel_GetManyBySubscription_Precedent {
                            user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Channel_GetManyBySubscription_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {
                        Channel_GetManyBySubscription_Precedent {
                            user_access_token__in_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_many_by_subscription__deserialize_deallocate(c_result: Channel_GetManyBySubscription_CResult) -> () {
    if c_result.is_data && c_result.data.is_target && c_result.data.target.is_filled {
        let data_registry = c_result.data.target.filled.data_registry.as_slice_unchecked();
        for data in data_registry {
            Allocator::<CString>::deallocate(data.channel__name);
            Allocator::<CString>::deallocate(data.channel__linked_name);
            if data.channel__background_image_path.is_data {
                Allocator::<CString>::deallocate(data.channel__background_image_path.data);
            }
            if data.channel__cover_image_path.is_data {
                Allocator::<CString>::deallocate(data.channel__cover_image_path.data);
            }
        }
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.data_registry);
    }
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Channel_GetManyPublicByName_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel__name: CString,
    pub requery___channel__name: COption<CString>,
    pub limit: c_short,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_many_public_by_name__serialize_allocate(incoming: Channel_GetManyPublicByName_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ Channel_GetManyPublicByName_Incoming| -> Result<Channel_GetManyPublicByName_Incoming_, Box<dyn StdError + 'static>> {
        let requery___channel__name = if incoming_.requery___channel__name.is_data {
            Option::Some(incoming_.requery___channel__name.data.get_as_str()?)
        } else {
            Option::None
        };
        return Result::Ok(
            Channel_GetManyPublicByName_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user_access_token__id: incoming_.user_access_token_signed.user_access_token__id.get_as_str()?,
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    singature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel__name: incoming_.channel__name.get_as_str()?,
                requery___channel__name,
                limit: incoming_.limit,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_many_public_by_name__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type Channel_GetManyPublicByName_CResult = CResult<CUnifiedReport<Channel_GetManyPublicByName_Outcoming, Channel_GetManyPublicByName_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyPublicByName_Data {
    pub channel__id: c_long,
    pub channel__name: CString,
    pub channel__linked_name: CString,
    pub channel__access_modifier: c_short,
    pub channel__cover_image_path: COption<CString>,
    pub channel__background_image_path: COption<CString>,
    pub channel_token_hashed_for_unsubscribed_users: COption<ChannelTokenHashed>,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyPublicByName_Outcoming {
    pub data_registry: CVector<Channel_GetManyPublicByName_Data>,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyPublicByName_Precedent {
    pub user_access_token__already_expired: bool,
    pub user_access_token__in_user_access_token_black_list: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_many_public_by_name__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> Channel_GetManyPublicByName_CResult {
    let converter = move |unified_report: UnifiedReport<Channel_GetManyPublicByName_Outcoming_, Channel_GetManyPublicByName_Precedent_>| -> Result<
        CUnifiedReport<Channel_GetManyPublicByName_Outcoming, Channel_GetManyPublicByName_Precedent>,
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
                        let mut data_registry: Vec<Channel_GetManyPublicByName_Data> = vec![];
                        '_a: for data___ in data__.data_registry {
                            let channel__cover_image_path = match data___.channel__cover_image_path {
                                Option::Some(channel__cover_image_path_) => COption::data(Allocator::<CString>::allocate(channel__cover_image_path_)),
                                Option::None => COption::none(),
                            };
                            let channel__background_image_path = match data___.channel__background_image_path {
                                Option::Some(channel__background_image_path_) => COption::data(Allocator::<CString>::allocate(channel__background_image_path_)),
                                Option::None => COption::none(),
                            };
                            let channel_token_hashed_for_unsubscribed_users = match data___.channel_token_hashed_for_unsubscribed_users {
                                Option::Some(channel_token_hashed_for_unsubscribed_users_) => {
                                    COption::data(
                                        ChannelTokenHashed {
                                            channel_token__expires_at: channel_token_hashed_for_unsubscribed_users_.channel_token__expires_at,
                                            hash: channel_token_hashed_for_unsubscribed_users_.hash,
                                        }
                                    )
                                }
                                Option::None => COption::none(),
                            };
                            let data_ = Channel_GetManyPublicByName_Data {
                                channel__id: data___.channel__id,
                                channel__name: Allocator::<CString>::allocate(data___.channel__name),
                                channel__linked_name: Allocator::<CString>::allocate(data___.channel__linked_name),
                                channel__access_modifier: data___.channel__access_modifier,
                                channel__cover_image_path,
                                channel__background_image_path,
                                channel_token_hashed_for_unsubscribed_users,
                            };
                            data_registry.push(data_);
                        }
                        let outcoming = Channel_GetManyPublicByName_Outcoming {
                            data_registry: Allocator::<CVector<_>>::allocate(data_registry),
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(data_)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    Channel_GetManyPublicByName_Precedent_::UserAccessToken_AlreadyExpired => {
                        Channel_GetManyPublicByName_Precedent {
                            user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Channel_GetManyPublicByName_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {
                        Channel_GetManyPublicByName_Precedent {
                            user_access_token__in_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_many_public_by_name__deserialize_deallocate(c_result: Channel_GetManyPublicByName_CResult) -> () {
    if c_result.is_data && c_result.data.is_target && c_result.data.target.is_filled {
        let data_registry = c_result.data.target.filled.data_registry.as_slice_unchecked();
        for data in data_registry {
            Allocator::<CString>::deallocate(data.channel__name);
            Allocator::<CString>::deallocate(data.channel__linked_name);
            if data.channel__background_image_path.is_data {
                Allocator::<CString>::deallocate(data.channel__background_image_path.data);
            }
            if data.channel__cover_image_path.is_data {
                Allocator::<CString>::deallocate(data.channel__cover_image_path.data);
            }
        }
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.data_registry);
    }
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Channel_GetOneById_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel__id: c_long,
    pub channel_token_hashed: COption<ChannelTokenHashed>,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_one_by_id__serialize_allocate(incoming: Channel_GetOneById_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ Channel_GetOneById_Incoming| -> Result<Channel_GetOneById_Incoming_, Box<dyn StdError + 'static>> {
        let channel_token_hashed = if incoming_.channel_token_hashed.is_data {
            Option::Some(
                ChannelTokenHashed_ {
                    channel_token__expires_at: incoming_.channel_token_hashed.data.channel_token__expires_at,
                    hash: incoming_.channel_token_hashed.data.hash,
                },
            )
        } else {
            Option::None
        };
        return Result::Ok(
            Channel_GetOneById_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user_access_token__id: incoming_.user_access_token_signed.user_access_token__id.get_as_str()?,
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    singature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel__id: incoming_.channel__id,
                channel_token_hashed,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_one_by_id__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type Channel_GetOneById_CResult = CResult<CUnifiedReport<Channel_GetOneById_Outcoming, Channel_GetOneById_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetOneById_Outcoming {
    pub channel__name: CString,
    pub channel__linked_name: CString,
    pub channel__description: COption<CString>,
    pub channel__access_modifier: c_short,
    pub channel__visability_modifier: c_short,
    pub channel__orientation: CVector<c_short>,
    pub channel__cover_image_path: COption<CString>,
    pub channel__background_image_path: COption<CString>,
    pub channel__subscribers_quantity: c_long,
    pub channel__marks_quantity: c_long,
    pub channel__viewing_quantity: c_long,
    pub user_is_channel_owner: bool,
    pub channel_subscription_token_hashed: ChannelSubscriptionTokenHashed,

}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetOneById_Precedent {
    pub user_access_token__already_expired: bool,
    pub user_access_token__in_user_access_token_black_list: bool,
    pub channel__not_found: bool,
    pub channel__is_close: bool,
    pub channel_token__not_found: bool,
    pub channel_token__already_expired: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_one_by_id__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> Channel_GetOneById_CResult {
    let converter = move |unified_report: UnifiedReport<Channel_GetOneById_Outcoming_, Channel_GetOneById_Precedent_>| -> Result<CUnifiedReport<Channel_GetOneById_Outcoming, Channel_GetOneById_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let data_ = match data {
                    Data::Empty => {
                        CData::empty()
                    }
                    Data::Filled {
                        data: data__
                    } => {
                        let channel__description = match data__.channel__description {
                            Option::Some(channel__description_) => COption::data(Allocator::<CString>::allocate(channel__description_)),
                            Option::None => COption::none()
                        };
                        let channel__cover_image_path = match data__.channel__cover_image_path {
                            Option::Some(channel__cover_image_path_) => COption::data(Allocator::<CString>::allocate(channel__cover_image_path_)),
                            Option::None => COption::none()
                        };
                        let channel__background_image_path = match data__.channel__background_image_path {
                            Option::Some(channel__background_image_path_) => COption::data(Allocator::<CString>::allocate(channel__background_image_path_)),
                            Option::None => COption::none()
                        };
                        let outcoming = Channel_GetOneById_Outcoming {
                            channel__name: Allocator::<CString>::allocate(data__.channel__name),
                            channel__linked_name: Allocator::<CString>::allocate(data__.channel__linked_name),
                            channel__description,
                            channel__access_modifier: data__.channel__access_modifier,
                            channel__visability_modifier: data__.channel__visability_modifier,
                            channel__orientation: Allocator::<CVector<_>>::allocate(data__.channel__orientation),
                            channel__cover_image_path,
                            channel__background_image_path,
                            channel__subscribers_quantity: data__.channel__subscribers_quantity,
                            channel__marks_quantity: data__.channel__marks_quantity,
                            channel__viewing_quantity: data__.channel__viewing_quantity,
                            user_is_channel_owner: data__.user_is_channel_owner,
                            channel_subscription_token_hashed: ChannelSubscriptionTokenHashed {
                                channel_subscription_token__expires_at: data__.channel_subscription_token_hashed.channel_subscription_token__expires_at,
                                hash: data__.channel_subscription_token_hashed.hash,
                            },
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(data_)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    Channel_GetOneById_Precedent_::UserAccessToken_AlreadyExpired => {
                        Channel_GetOneById_Precedent {
                            user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    Channel_GetOneById_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {
                        Channel_GetOneById_Precedent {
                            user_access_token__in_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
                    Channel_GetOneById_Precedent_::Channel_NotFound => {
                        Channel_GetOneById_Precedent {
                            channel__not_found: true,
                            ..Default::default()
                        }
                    }
                    Channel_GetOneById_Precedent_::Channel_IsClose => {
                        Channel_GetOneById_Precedent {
                            channel__is_close: true,
                            ..Default::default()
                        }
                    }
                    Channel_GetOneById_Precedent_::ChannelToken_NotFound => {
                        Channel_GetOneById_Precedent {
                            channel_token__not_found: true,
                            ..Default::default()
                        }
                    }
                    Channel_GetOneById_Precedent_::ChannelToken_AlreadyExpired => {
                        Channel_GetOneById_Precedent {
                            channel_token__already_expired: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_one_by_id__deserialize_deallocate(c_result: Channel_GetOneById_CResult) -> () {
    if c_result.is_data && c_result.data.is_target && c_result.data.target.is_filled {
        Allocator::<CString>::deallocate(c_result.data.target.filled.channel__name);
        Allocator::<CString>::deallocate(c_result.data.target.filled.channel__linked_name);
        if c_result.data.target.filled.channel__description.is_data {
            Allocator::<CString>::deallocate(c_result.data.target.filled.channel__description.data);
        }
        if c_result.data.target.filled.channel__background_image_path.is_data {
            Allocator::<CString>::deallocate(c_result.data.target.filled.channel__background_image_path.data);
        }
        if c_result.data.target.filled.channel__cover_image_path.is_data {
            Allocator::<CString>::deallocate(c_result.data.target.filled.channel__cover_image_path.data);
        }
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.channel__orientation);
    }
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelSubscription_Create_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel__id: c_long,
    pub channel_subscription_token_hashed: ChannelSubscriptionTokenHashed,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_subscription__create__serialize_allocate(incoming: ChannelSubscription_Create_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ ChannelSubscription_Create_Incoming| -> Result<ChannelSubscription_Create_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            ChannelSubscription_Create_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user_access_token__id: incoming_.user_access_token_signed.user_access_token__id.get_as_str()?,
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    singature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel__id: incoming_.channel__id,
                channel_subscription_token_hashed: ChannelSubscriptionTokenHashed_ {
                    channel_subscription_token__expires_at: incoming_.channel_subscription_token_hashed.channel_subscription_token__expires_at,
                    hash: incoming_.channel_subscription_token_hashed.hash,
                },
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_subscription__create__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ChannelSubscription_Create_CResult = CResult<CUnifiedReport<CVoid, ChannelSubscription_Create_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ChannelSubscription_Create_Precedent {
    pub user_access_token__already_expired: bool,
    pub user_access_token__in_user_access_token_black_list: bool,
    pub channel__not_found: bool,
    pub channel__is_close: bool,
    pub user__is_channel__owner: bool,
    pub channel_subscription_token__already_expired: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_subscription__create__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> ChannelSubscription_Create_CResult {
    let converter = move |unified_report: UnifiedReport<Void, ChannelSubscription_Create_Precedent_>| -> Result<CUnifiedReport<CVoid, ChannelSubscription_Create_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ChannelSubscription_Create_Precedent_::UserAccessToken_AlreadyExpired => {
                        ChannelSubscription_Create_Precedent {
                            user_access_token__already_expired: true,
                            ..Default::default()
                        }
                    }
                    ChannelSubscription_Create_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {
                        ChannelSubscription_Create_Precedent {
                            user_access_token__in_user_access_token_black_list: true,
                            ..Default::default()
                        }
                    }
                    ChannelSubscription_Create_Precedent_::Channel_NotFound => {
                        ChannelSubscription_Create_Precedent {
                            channel__not_found: true,
                            ..Default::default()
                        }
                    }
                    ChannelSubscription_Create_Precedent_::Channel_IsClose => {
                        ChannelSubscription_Create_Precedent {
                            channel__is_close: true,
                            ..Default::default()
                        }
                    }
                    ChannelSubscription_Create_Precedent_::User_IsChannelOwner => {
                        ChannelSubscription_Create_Precedent {
                            user__is_channel__owner: true,
                            ..Default::default()
                        }
                    }
                    ChannelSubscription_Create_Precedent_::ChannelSubscriptionToken_AlreadyExpired => {
                        ChannelSubscription_Create_Precedent {
                            channel_subscription_token__already_expired: true,
                            ..Default::default()
                        }
                    }
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
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_subscription__create__deserialize_deallocate(_c_result: ChannelSubscription_Create_CResult) -> () {
    return ();
}
#[cfg(test)]
mod test {
    use {
        super::*,
        stats_alloc::{
            Region,
            StatsAlloc,
            INSTRUMENTED_SYSTEM,
        },
        std::alloc::System,
    };
    const NOT_EMPTY_STRING_LITERAL: &'static str = "qwerty";
    const NOT_EMPTY_ARRAY_LITERAL: [u8; 3] = [
        0,
        1,
        2,
    ];
    #[global_allocator]
    static GLOBAL_ALLOCATOR: &'static StatsAlloc<System> = &INSTRUMENTED_SYSTEM;
    // Everything must be done sequentially on 1 thread without parallel hidden allocations, so that there is no unaccounted
    // change in the amount of allocation. The variant with many tests (#[test]) and with crate `serial_test`` or with creating
    // a Mutex is not suitable, because this helps to run methods sequentially, but does not prevent allocations from being aliased
    // in other threads, since there is still more than one thread.
    #[test]
    fn all_sequentially() -> Result<(), Box<dyn StdError + 'static>> {
        fn get_function_name<F>(_: F) -> &'static str
        where
            F: FnOnce() -> Result<(), Box<dyn StdError + 'static>>,
        {
            return std::any::type_name::<F>();
        }
        let tests: Vec<(
            fn() -> Result<(), Box<dyn StdError + 'static>>,
            &'static str,
        )> = vec![
            (
                self::deallocation::c_vector_clone,
                get_function_name(self::deallocation::c_vector_clone),
            ),
            (
                self::deallocation::c_string_clone,
                get_function_name(self::deallocation::c_string_clone),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__authorize_by_first_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__authorize_by_first_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__authorize_by_first_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__authorize_by_first_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__authorize_by_first_step,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__authorize_by_first_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__authorize_by_last_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__authorize_by_last_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__authorize_by_last_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__authorize_by_last_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__authorize_by_last_step,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__authorize_by_last_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__check_email_for_existing,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__check_email_for_existing),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__check_email_for_existing,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__check_email_for_existing),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__check_email_for_existing,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__check_email_for_existing),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__check_nickname_for_existing,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__check_nickname_for_existing),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__check_nickname_for_existing,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__check_nickname_for_existing),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__check_nickname_for_existing,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__check_nickname_for_existing),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__deauthorize_from_all_devices,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__deauthorize_from_all_devices),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__deauthorize_from_all_devices,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__deauthorize_from_all_devices),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__deauthorize_from_all_devices,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__deauthorize_from_all_devices),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__deauthorize_from_one_device,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__deauthorize_from_one_device),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__deauthorize_from_one_device,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__deauthorize_from_one_device),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__deauthorize_from_one_device,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__deauthorize_from_one_device),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__refresh_access_token,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__refresh_access_token),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__refresh_access_token,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__refresh_access_token),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__refresh_access_token,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__refresh_access_token),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__register_by_first_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__register_by_first_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__register_by_first_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__register_by_first_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__register_by_first_step,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__register_by_first_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__register_by_second_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__register_by_second_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__register_by_second_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__register_by_second_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__register_by_second_step,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__register_by_second_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__register_by_last_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__register_by_last_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__register_by_last_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__register_by_last_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__register_by_last_step,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__register_by_last_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__reset_password_by_first_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__reset_password_by_first_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__reset_password_by_first_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__reset_password_by_first_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__reset_password_by_first_step,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__reset_password_by_first_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__reset_password_by_second_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__reset_password_by_second_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__reset_password_by_second_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__reset_password_by_second_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__reset_password_by_second_step,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__reset_password_by_second_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__reset_password_by_last_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__reset_password_by_last_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__reset_password_by_last_step,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__reset_password_by_last_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__reset_password_by_last_step,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__reset_password_by_last_step),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__send_email_for_register,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__send_email_for_register),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__send_email_for_register,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__send_email_for_register),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__send_email_for_register,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__send_email_for_register),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__send_email_for_authorize,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__send_email_for_authorize),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__send_email_for_authorize,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__send_email_for_authorize),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__send_email_for_authorize,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__send_email_for_authorize),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__user_authorization__send_email_for_reset_password,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__send_email_for_reset_password),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__user_authorization__send_email_for_reset_password,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__send_email_for_reset_password),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__user_authorization__send_email_for_reset_password,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__user_authorization__send_email_for_reset_password),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__channel__get_many_by_name_in_subscriptions,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__channel__get_many_by_name_in_subscriptions),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__channel__get_many_by_name_in_subscriptions,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__channel__get_many_by_name_in_subscriptions),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__channel__get_many_by_name_in_subscriptions,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__channel__get_many_by_name_in_subscriptions),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__channel__get_many_by_subscription,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__channel__get_many_by_subscription),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__channel__get_many_by_subscription,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__channel__get_many_by_subscription),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__channel__get_many_by_subscription,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__channel__get_many_by_subscription),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__channel__get_many_public_by_name,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__channel__get_many_public_by_name),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__channel__get_many_public_by_name,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__channel__get_many_public_by_name),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__channel__get_many_public_by_name,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__channel__get_many_public_by_name),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__channel__get_one_by_id,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__channel__get_one_by_id),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__channel__get_one_by_id,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__channel__get_one_by_id),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__channel__get_one_by_id,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__channel__get_one_by_id),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_empty__channel_subscription__create,
                get_function_name(self::deallocation::server_response_data_deserialization::target_empty__channel_subscription__create),
            ),
            (
                self::deallocation::server_response_data_deserialization::target_filled__channel_subscription__create,
                get_function_name(self::deallocation::server_response_data_deserialization::target_filled__channel_subscription__create),
            ),
            (
                self::deallocation::server_response_data_deserialization::precedent__channel_subscription__create,
                get_function_name(self::deallocation::server_response_data_deserialization::precedent__channel_subscription__create),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__authorize_by_first_step,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__authorize_by_first_step),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__authorize_by_last_step,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__authorize_by_last_step),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__check_email_for_existing,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__check_email_for_existing),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__check_nickname_for_existing,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__check_nickname_for_existing),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__deauthorize_from_all_devices,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__deauthorize_from_all_devices),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__deauthorize_from_one_device,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__deauthorize_from_one_device),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__refresh_access_token,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__refresh_access_token),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__register_by_first_step,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__register_by_first_step),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__register_by_second_step,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__register_by_second_step),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__register_by_last_step,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__register_by_last_step),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__reset_password_by_first_step,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__reset_password_by_first_step),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__reset_password_by_second_step,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__reset_password_by_second_step),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__reset_password_by_last_step,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__reset_password_by_last_step),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__send_email_for_register,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__send_email_for_register),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__send_email_for_authorize,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__send_email_for_authorize),
            ),
            (
                self::deallocation::server_request_data_serialization::user_authorization__send_email_for_reset_password,
                get_function_name(self::deallocation::server_request_data_serialization::user_authorization__send_email_for_reset_password),
            ),
            (
                self::deallocation::server_request_data_serialization::channel__get_many_by_name_in_subscriptions,
                get_function_name(self::deallocation::server_request_data_serialization::channel__get_many_by_name_in_subscriptions),
            ),
            (
                self::deallocation::server_request_data_serialization::channel__get_many_by_subscription,
                get_function_name(self::deallocation::server_request_data_serialization::channel__get_many_by_subscription),
            ),
            (
                self::deallocation::server_request_data_serialization::channel__get_many_public_by_name,
                get_function_name(self::deallocation::server_request_data_serialization::channel__get_many_public_by_name),
            ),
            (
                self::deallocation::server_request_data_serialization::channel__get_one_by_id,
                get_function_name(self::deallocation::server_request_data_serialization::channel__get_one_by_id),
            ),
            (
                self::deallocation::server_request_data_serialization::channel_subscription__create,
                get_function_name(self::deallocation::server_request_data_serialization::channel_subscription__create),
            ),
        ];
        // https://docs.rs/bitcode/0.6.3/src/bitcode/derive/mod.rs.html#68
        // When the `bitcode::encode` method is first called for a specific type, an additional byte is allocated and
        // is not deallocated until the program process completes. Accordingly, when the `bitcode::encode` method is called
        // again, no additional byte occurs and it becomes possible to expect that the number of allocated bytes will be
        // equal to the number of deallocated bytes.
        '_a: for test in tests.iter() {
            let _ = test.0();
        }
        '_a: for test in tests.iter() {
            let region = Region::new(&GLOBAL_ALLOCATOR);
            if let Result::Err(error) = test.0() {
                return Result::Err(format!("{}: {}", test.1, &error).into());
            }
            let statistics = region.change();
            if statistics.bytes_allocated != statistics.bytes_deallocated {
                return Result::Err(
                    format!(
                        "{}: {} Undeallocated bytes: {}",
                        test.1,
                        DEALLOCATION_ERROR,
                        statistics.bytes_allocated - statistics.bytes_deallocated,
                    )
                    .into(),
                );
            }
        }
        return Result::Ok(());
    }
    pub mod deallocation {
        use super::*;
        pub fn c_vector_clone() -> Result<(), Box<dyn StdError + 'static>> {
            let c_vector = Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec());
            {
                let _ = c_vector.clone_as_vec()?;
            }
            if c_vector.pointer.is_null() {
                return Result::Err(ALLOCATION_ERROR.into());
            }
            Allocator::<CVector<_>>::deallocate(c_vector);
            return Result::Ok(());
        }
        pub fn c_string_clone() -> Result<(), Box<dyn StdError + 'static>> {
            let c_string = Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string());
            {
                let _ = c_string.clone_as_string()?;
            }
            if c_string.pointer.is_null() {
                return Result::Err(ALLOCATION_ERROR.into());
            }
            Allocator::<CString>::deallocate(c_string);
            return Result::Ok(());
        }
        pub mod server_response_data_deserialization {
            use {
                super::*,
                dedicated::{
                    action_processor_incoming_outcoming::action_processor::channel::{
                        get_many_public_by_name::Data as Channel_GetManyPublicByName_Data_,
                        get_many_by_name_in_subscriptions::Data as Channel_GetManyByNameInSubscriptions_Data_,
                        get_many_by_subscription::Data as Channel_GetManyBySubscription_Data_,
                    },
                    user_access_token_signed::UserAccessTokenSigned_ as UserAccessTokenSigned__,
                },
            };
            fn run_by_template<'a, T, E>(
                data: &'a T,
                allocator: extern "C-unwind" fn(CVector<c_uchar>) -> CResult<E>,
                deallocator: extern "C-unwind" fn(CResult<E>) -> (),
            ) -> Result<(), Box<dyn StdError + 'static>>
            where
                T: Encode,
            {
                let c_vector = Allocator::<CVector<_>>::allocate(Serializer::serialize(data));
                deallocator(allocator(c_vector));
                Allocator::<CVector<_>>::deallocate(c_vector);
                return Result::Ok(());
            }
            pub fn target_empty__user_authorization__authorize_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_AuthorizeByFirstStep_Outcoming_, UserAuthorization_AuthorizeByFirstStep_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__authorize_by_first_step__deserialize_allocate,
                    user_authorization__authorize_by_first_step__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__authorize_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = UserAuthorization_AuthorizeByFirstStep_Outcoming_ {
                    user__id: 0,
                    verification_message_sent: false,
                    user_authorization_token__can_be_resent_from: 0,
                    user_authorization_token__wrong_enter_tries_quantity: 0,
                    user_authorization_token__wrong_enter_tries_quantity_limit: 0,
                };
                let unified_report =
                    UnifiedReport::<UserAuthorization_AuthorizeByFirstStep_Outcoming_, UserAuthorization_AuthorizeByFirstStep_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    user_authorization__authorize_by_first_step__deserialize_allocate,
                    user_authorization__authorize_by_first_step__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__authorize_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_AuthorizeByFirstStep_Precedent_::User_WrongEmailOrNicknameOrPassword {
                    UserAuthorization_AuthorizeByFirstStep_Precedent_::User_WrongEmailOrNicknameOrPassword => {}
                }
                let precedent = UserAuthorization_AuthorizeByFirstStep_Precedent_::User_WrongEmailOrNicknameOrPassword;
                let unified_report = UnifiedReport::<UserAuthorization_AuthorizeByFirstStep_Outcoming_, UserAuthorization_AuthorizeByFirstStep_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__authorize_by_first_step__deserialize_allocate,
                    user_authorization__authorize_by_first_step__deserialize_deallocate,
                );
            }
            pub fn target_empty__user_authorization__authorize_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_AuthorizeByLastStep_Outcoming_, UserAuthorization_AuthorizeByLastStep_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__authorize_by_last_step__deserialize_allocate,
                    user_authorization__authorize_by_last_step__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__authorize_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = UserAuthorization_AuthorizeByLastStep_Outcoming_ {
                    user_access_token_signed: UserAccessTokenSigned__ {
                        user_access_token__id: NOT_EMPTY_STRING_LITERAL.to_string(),
                        user__id: 0,
                        user_device__id: NOT_EMPTY_STRING_LITERAL.to_string(),
                        user_access_token__expires_at: 0,
                        signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                    },
                    user_access_refresh_token_signed: UserAccessRefreshTokenSigned_ {
                        user_access_refresh_token__expires_at: 0,
                        signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                    }
                };
                let unified_report = UnifiedReport::<UserAuthorization_AuthorizeByLastStep_Outcoming_, UserAuthorization_AuthorizeByLastStep_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    user_authorization__authorize_by_last_step__deserialize_allocate,
                    user_authorization__authorize_by_last_step__deserialize_deallocate,
                );
            }
            fn _precedent__user_authorization__authorize_by_last_step(precedent: UserAuthorization_AuthorizeByLastStep_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_AuthorizeByLastStep_Outcoming_, UserAuthorization_AuthorizeByLastStep_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__authorize_by_last_step__deserialize_allocate,
                    user_authorization__authorize_by_last_step__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__authorize_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken_NotFound {
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken_NotFound => {}
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken_AlreadyExpired => {}
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken_WrongValue {
                        user_authorization_token__wrong_enter_tries_quantity: _,
                    } => {}
                    UserAuthorization_AuthorizeByLastStep_Precedent_::User_NotFound => {}
                }
                let mut precedents: Vec<UserAuthorization_AuthorizeByLastStep_Precedent_> = vec![];
                precedents.push(UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken_NotFound);
                precedents.push(UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken_AlreadyExpired);
                precedents.push(
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken_WrongValue {
                        user_authorization_token__wrong_enter_tries_quantity: 0,
                    },
                );
                precedents.push(UserAuthorization_AuthorizeByLastStep_Precedent_::User_NotFound);
                '_a: for precedent in precedents {
                    _precedent__user_authorization__authorize_by_last_step(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__user_authorization__check_email_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_CheckEmailForExisting_Outcoming_, Void>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__check_email_for_existing__deserialize_allocate,
                    user_authorization__check_email_for_existing__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__check_email_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = UserAuthorization_CheckEmailForExisting_Outcoming_ {
                    result: false,
                };
                let unified_report = UnifiedReport::<UserAuthorization_CheckEmailForExisting_Outcoming_, Void>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    user_authorization__check_email_for_existing__deserialize_allocate,
                    user_authorization__check_email_for_existing__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__check_email_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            pub fn target_empty__user_authorization__check_nickname_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_CheckNicknameForExisting_Outcoming_, Void>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__check_nickname_for_existing__deserialize_allocate,
                    user_authorization__check_nickname_for_existing__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__check_nickname_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = UserAuthorization_CheckNicknameForExisting_Outcoming_ {
                    result: false,
                };
                let unified_report = UnifiedReport::<UserAuthorization_CheckNicknameForExisting_Outcoming_, Void>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    user_authorization__check_nickname_for_existing__deserialize_allocate,
                    user_authorization__check_nickname_for_existing__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__check_nickname_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            pub fn target_empty__user_authorization__deauthorize_from_all_devices() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, UserAuthorization_DeauthorizeFromAllDevices_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__deauthorize_from_all_devices__deserialize_allocate,
                    user_authorization__deauthorize_from_all_devices__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__deauthorize_from_all_devices() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            fn _precedent__user_authorization__deauthorize_from_all_devices(
                precedent: UserAuthorization_DeauthorizeFromAllDevices_Precedent_,
            ) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, UserAuthorization_DeauthorizeFromAllDevices_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__deauthorize_from_all_devices__deserialize_allocate,
                    user_authorization__deauthorize_from_all_devices__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__deauthorize_from_all_devices() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_DeauthorizeFromAllDevices_Precedent_::UserAccessToken_AlreadyExpired {
                    UserAuthorization_DeauthorizeFromAllDevices_Precedent_::UserAccessToken_AlreadyExpired => {}
                    UserAuthorization_DeauthorizeFromAllDevices_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {}
                }
                let mut precedents: Vec<UserAuthorization_DeauthorizeFromAllDevices_Precedent_> = vec![];
                precedents.push(UserAuthorization_DeauthorizeFromAllDevices_Precedent_::UserAccessToken_AlreadyExpired);
                precedents.push(UserAuthorization_DeauthorizeFromAllDevices_Precedent_::UserAccessToken_InUserAccessTokenBlackList);
                '_a: for precedent in precedents {
                    _precedent__user_authorization__deauthorize_from_all_devices(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__user_authorization__deauthorize_from_one_device() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, UserAuthorization_DeauthorizeFromOneDevice_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__deauthorize_from_one_device__deserialize_allocate,
                    user_authorization__deauthorize_from_one_device__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__deauthorize_from_one_device() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            fn _precedent__user_authorization__deauthorize_from_one_device(
                precedent: UserAuthorization_DeauthorizeFromOneDevice_Precedent_,
            ) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, UserAuthorization_DeauthorizeFromOneDevice_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__deauthorize_from_one_device__deserialize_allocate,
                    user_authorization__deauthorize_from_one_device__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__deauthorize_from_one_device() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_DeauthorizeFromOneDevice_Precedent_::UserAccessToken_AlreadyExpired {
                    UserAuthorization_DeauthorizeFromOneDevice_Precedent_::UserAccessToken_AlreadyExpired => {}
                    UserAuthorization_DeauthorizeFromOneDevice_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {}
                }
                let mut precedents: Vec<UserAuthorization_DeauthorizeFromOneDevice_Precedent_> = vec![];
                precedents.push(UserAuthorization_DeauthorizeFromOneDevice_Precedent_::UserAccessToken_AlreadyExpired);
                precedents.push(UserAuthorization_DeauthorizeFromOneDevice_Precedent_::UserAccessToken_InUserAccessTokenBlackList);
                '_a: for precedent in precedents {
                    _precedent__user_authorization__deauthorize_from_one_device(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__user_authorization__refresh_access_token() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_RefreshAccessToken_Outcoming_, UserAuthorization_RefreshAccessToken_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__refresh_access_token__deserialize_allocate,
                    user_authorization__refresh_access_token__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__refresh_access_token() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = UserAuthorization_RefreshAccessToken_Outcoming_ {
                    user_access_token_signed: UserAccessTokenSigned__ {
                        user_access_token__id: NOT_EMPTY_STRING_LITERAL.to_string(),
                        user__id: 0,
                        user_device__id: NOT_EMPTY_STRING_LITERAL.to_string(),
                        user_access_token__expires_at: 0,
                        signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                    },
                    user_access_refresh_token_signed: UserAccessRefreshTokenSigned_ {
                        user_access_refresh_token__expires_at: 0,
                        signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                    },
                };
                let unified_report = UnifiedReport::<UserAuthorization_RefreshAccessToken_Outcoming_, UserAuthorization_RefreshAccessToken_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    user_authorization__refresh_access_token__deserialize_allocate,
                    user_authorization__refresh_access_token__deserialize_deallocate,
                );
            }
            fn _precedent__user_authorization__refresh_access_token(precedent: UserAuthorization_RefreshAccessToken_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_RefreshAccessToken_Outcoming_, UserAuthorization_RefreshAccessToken_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__refresh_access_token__deserialize_allocate,
                    user_authorization__refresh_access_token__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__refresh_access_token() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken_NotFound {
                    UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken_NotFound => {}
                    UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken_AlreadyExpired => {}
                }
                let mut precedents: Vec<UserAuthorization_RefreshAccessToken_Precedent_> = vec![];
                precedents.push(UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken_NotFound);
                precedents.push(UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken_AlreadyExpired);
                '_a: for precedent in precedents {
                    _precedent__user_authorization__refresh_access_token(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__user_authorization__register_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_RegisterByFirstStep_Outcoming_, UserAuthorization_RegisterByFirstStep_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__register_by_first_step__deserialize_allocate,
                    user_authorization__register_by_first_step__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__register_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = UserAuthorization_RegisterByFirstStep_Outcoming_ {
                    verification_message_sent: false,
                    user_registration_token__can_be_resent_from: 0,
                    user_registration_token__wrong_enter_tries_quantity: 0,
                    user_registration_token__wrong_enter_tries_quantity_limit: 0,
                };
                let unified_report = UnifiedReport::<UserAuthorization_RegisterByFirstStep_Outcoming_, UserAuthorization_RegisterByFirstStep_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    user_authorization__register_by_first_step__deserialize_allocate,
                    user_authorization__register_by_first_step__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__register_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_RegisterByFirstStep_Precedent_::User_EmailAlreadyExist {
                    UserAuthorization_RegisterByFirstStep_Precedent_::User_EmailAlreadyExist => {}
                }
                let precedent = UserAuthorization_RegisterByFirstStep_Precedent_::User_EmailAlreadyExist;
                let unified_report = UnifiedReport::<UserAuthorization_RegisterByFirstStep_Outcoming_, UserAuthorization_RegisterByFirstStep_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__register_by_first_step__deserialize_allocate,
                    user_authorization__register_by_first_step__deserialize_deallocate,
                );
            }
            pub fn target_empty__user_authorization__register_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, UserAuthorization_RegisterBySecondStep_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__register_by_second_step__deserialize_allocate,
                    user_authorization__register_by_second_step__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__register_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            fn _precedent__user_authorization__register_by_second_step(precedent: UserAuthorization_RegisterBySecondStep_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, UserAuthorization_RegisterBySecondStep_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__register_by_second_step__deserialize_allocate,
                    user_authorization__register_by_second_step__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__register_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken_NotFound {
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken_NotFound => {}
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken_AlreadyExpired => {}
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken_AlreadyApproved => {}
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken_WrongValue {
                        user_registration_token__wrong_enter_tries_quantity: _,
                    } => {}
                }
                let mut precedents: Vec<UserAuthorization_RegisterBySecondStep_Precedent_> = vec![];
                precedents.push(UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken_NotFound);
                precedents.push(UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken_AlreadyExpired);
                precedents.push(UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken_AlreadyApproved);
                precedents.push(
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken_WrongValue {
                        user_registration_token__wrong_enter_tries_quantity: 0,
                    },
                );
                '_a: for precedent in precedents {
                    _precedent__user_authorization__register_by_second_step(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__user_authorization__register_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_RegisterByLastStep_Outcoming_, UserAuthorization_RegisterByLastStep_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__register_by_last_step__deserialize_allocate,
                    user_authorization__register_by_last_step__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__register_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = UserAuthorization_RegisterByLastStep_Outcoming_ {
                    user_access_token_signed: UserAccessTokenSigned__ {
                        user_access_token__id: NOT_EMPTY_STRING_LITERAL.to_string(),
                        user__id: 0,
                        user_device__id: NOT_EMPTY_STRING_LITERAL.to_string(),
                        user_access_token__expires_at: 0,
                        signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                    },
                    user_access_refresh_token_signed: UserAccessRefreshTokenSigned_ {
                        user_access_refresh_token__expires_at: 0,
                        signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                    },
                };
                let unified_report = UnifiedReport::<UserAuthorization_RegisterByLastStep_Outcoming_, UserAuthorization_RegisterByLastStep_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    user_authorization__register_by_last_step__deserialize_allocate,
                    user_authorization__register_by_last_step__deserialize_deallocate,
                );
            }
            fn _precedent__user_authorization__register_by_last_step(precedent: UserAuthorization_RegisterByLastStep_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_RegisterByLastStep_Outcoming_, UserAuthorization_RegisterByLastStep_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__register_by_last_step__deserialize_allocate,
                    user_authorization__register_by_last_step__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__register_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_RegisterByLastStep_Precedent_::User_NicknameAlreadyExist {
                    UserAuthorization_RegisterByLastStep_Precedent_::User_NicknameAlreadyExist => {}
                    UserAuthorization_RegisterByLastStep_Precedent_::User_EmailAlreadyExist => {}
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken_NotFound => {}
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken_AlreadyExpired => {}
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken_IsNotApproved => {}
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken_WrongValue => {}
                }
                let mut precedents: Vec<UserAuthorization_RegisterByLastStep_Precedent_> = vec![];
                precedents.push(UserAuthorization_RegisterByLastStep_Precedent_::User_NicknameAlreadyExist);
                precedents.push(UserAuthorization_RegisterByLastStep_Precedent_::User_EmailAlreadyExist);
                precedents.push(UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken_NotFound);
                precedents.push(UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken_AlreadyExpired);
                precedents.push(UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken_IsNotApproved);
                precedents.push(UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken_WrongValue);
                '_a: for precedent in precedents {
                    _precedent__user_authorization__register_by_last_step(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__user_authorization__reset_password_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_ResetPasswordByFirstStep_Outcoming_, UserAuthorization_ResetPasswordByFirstStep_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__reset_password_by_first_step__deserialize_allocate,
                    user_authorization__reset_password_by_first_step__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__reset_password_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = UserAuthorization_ResetPasswordByFirstStep_Outcoming_ {
                    user__id: 0,
                    verification_message_sent: false,
                    user_reset_password_token__can_be_resent_from: 0,
                    user_reset_password_token__wrong_enter_tries_quantity: 0,
                    user_reset_password_token__wrong_enter_tries_quantity_limit: 0,
                };
                let unified_report =
                    UnifiedReport::<UserAuthorization_ResetPasswordByFirstStep_Outcoming_, UserAuthorization_ResetPasswordByFirstStep_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    user_authorization__reset_password_by_first_step__deserialize_allocate,
                    user_authorization__reset_password_by_first_step__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__reset_password_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_ResetPasswordByFirstStep_Precedent_::User_NotFound {
                    UserAuthorization_ResetPasswordByFirstStep_Precedent_::User_NotFound => {}
                }
                let precedent = UserAuthorization_ResetPasswordByFirstStep_Precedent_::User_NotFound;
                let unified_report =
                    UnifiedReport::<UserAuthorization_ResetPasswordByFirstStep_Outcoming_, UserAuthorization_ResetPasswordByFirstStep_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__reset_password_by_first_step__deserialize_allocate,
                    user_authorization__reset_password_by_first_step__deserialize_deallocate,
                );
            }
            pub fn target_empty__user_authorization__reset_password_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, UserAuthorization_ResetPasswordBySecondStep_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__reset_password_by_second_step__deserialize_allocate,
                    user_authorization__reset_password_by_second_step__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__reset_password_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            fn _precedent__user_authorization__reset_password_by_second_step(
                precedent: UserAuthorization_ResetPasswordBySecondStep_Precedent_,
            ) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, UserAuthorization_ResetPasswordBySecondStep_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__reset_password_by_second_step__deserialize_allocate,
                    user_authorization__reset_password_by_second_step__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__reset_password_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken_NotFound {
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken_NotFound => {}
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken_AlreadyExpired => {}
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken_AlreadyApproved => {}
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken_WrongValue {
                        user_reset_password_token__wrong_enter_tries_quantity: _,
                    } => {}
                }
                let mut precedents: Vec<UserAuthorization_ResetPasswordBySecondStep_Precedent_> = vec![];
                precedents.push(UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken_NotFound);
                precedents.push(UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken_AlreadyExpired);
                precedents.push(UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken_AlreadyApproved);
                precedents.push(
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken_WrongValue {
                        user_reset_password_token__wrong_enter_tries_quantity: 0,
                    },
                );
                '_a: for precedent in precedents {
                    _precedent__user_authorization__reset_password_by_second_step(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__user_authorization__reset_password_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, UserAuthorization_ResetPasswordByLastStep_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__reset_password_by_last_step__deserialize_allocate,
                    user_authorization__reset_password_by_last_step__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__reset_password_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            fn _precedent__user_authorization__reset_password_by_last_step(
                precedent: UserAuthorization_ResetPasswordByLastStep_Precedent_,
            ) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, UserAuthorization_ResetPasswordByLastStep_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__reset_password_by_last_step__deserialize_allocate,
                    user_authorization__reset_password_by_last_step__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__reset_password_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_ResetPasswordByLastStep_Precedent_::User_NotFound {
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::User_NotFound => {}
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken_NotFound => {}
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken_AlreadyExpired => {}
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken_IsNotApproved => {}
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken_WrongValue => {}
                }
                let mut precedents: Vec<UserAuthorization_ResetPasswordByLastStep_Precedent_> = vec![];
                precedents.push(UserAuthorization_ResetPasswordByLastStep_Precedent_::User_NotFound);
                precedents.push(UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken_NotFound);
                precedents.push(UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken_AlreadyExpired);
                precedents.push(UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken_IsNotApproved);
                precedents.push(UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken_WrongValue);
                '_a: for precedent in precedents {
                    _precedent__user_authorization__reset_password_by_last_step(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__user_authorization__send_email_for_register() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_SendEmailForRegister_Outcoming_, UserAuthorization_SendEmailForRegister_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__send_email_for_register__deserialize_allocate,
                    user_authorization__send_email_for_register__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__send_email_for_register() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = UserAuthorization_SendEmailForRegister_Outcoming_ {
                    user_registration_token__can_be_resent_from: 0,
                };
                let unified_report =
                    UnifiedReport::<UserAuthorization_SendEmailForRegister_Outcoming_, UserAuthorization_SendEmailForRegister_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    user_authorization__send_email_for_register__deserialize_allocate,
                    user_authorization__send_email_for_register__deserialize_deallocate,
                );
            }
            fn _precedent__user_authorization__send_email_for_register(precedent: UserAuthorization_SendEmailForRegister_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_SendEmailForRegister_Outcoming_, UserAuthorization_SendEmailForRegister_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__send_email_for_register__deserialize_allocate,
                    user_authorization__send_email_for_register__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__send_email_for_register() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken_NotFound {
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken_NotFound => {}
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken_AlreadyExpired => {}
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken_AlreadyApproved => {}
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken_TimeToResendHasNotCome => {}
                }
                let mut precedents: Vec<UserAuthorization_SendEmailForRegister_Precedent_> = vec![];
                precedents.push(UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken_NotFound);
                precedents.push(UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken_AlreadyExpired);
                precedents.push(UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken_AlreadyApproved);
                precedents.push(UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken_TimeToResendHasNotCome);
                '_a: for precedent in precedents {
                    _precedent__user_authorization__send_email_for_register(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__user_authorization__send_email_for_authorize() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_SendEmailForAuthorize_Outcoming_, UserAuthorization_SendEmailForAuthorize_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__send_email_for_authorize__deserialize_allocate,
                    user_authorization__send_email_for_authorize__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__send_email_for_authorize() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = UserAuthorization_SendEmailForAuthorize_Outcoming_ {
                    user_authorization_token__can_be_resent_from: 0,
                };
                let unified_report =
                    UnifiedReport::<UserAuthorization_SendEmailForAuthorize_Outcoming_, UserAuthorization_SendEmailForAuthorize_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    user_authorization__send_email_for_authorize__deserialize_allocate,
                    user_authorization__send_email_for_authorize__deserialize_deallocate,
                );
            }
            fn _precedent__user_authorization__send_email_for_authorize(precedent: UserAuthorization_SendEmailForAuthorize_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_SendEmailForAuthorize_Outcoming_, UserAuthorization_SendEmailForAuthorize_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__send_email_for_authorize__deserialize_allocate,
                    user_authorization__send_email_for_authorize__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__send_email_for_authorize() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_SendEmailForAuthorize_Precedent_::User_NotFound {
                    UserAuthorization_SendEmailForAuthorize_Precedent_::User_NotFound => {}
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken_NotFound => {}
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken_AlreadyExpired => {}
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken_TimeToResendHasNotCome => {}
                }
                let mut precedents: Vec<UserAuthorization_SendEmailForAuthorize_Precedent_> = vec![];
                precedents.push(UserAuthorization_SendEmailForAuthorize_Precedent_::User_NotFound);
                precedents.push(UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken_NotFound);
                precedents.push(UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken_AlreadyExpired);
                precedents.push(UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken_TimeToResendHasNotCome);
                '_a: for precedent in precedents {
                    _precedent__user_authorization__send_email_for_authorize(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__user_authorization__send_email_for_reset_password() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report =
                    UnifiedReport::<UserAuthorization_SendEmailForResetPassword_Outcoming_, UserAuthorization_SendEmailForResetPassword_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    user_authorization__send_email_for_reset_password__deserialize_allocate,
                    user_authorization__send_email_for_reset_password__deserialize_deallocate,
                );
            }
            pub fn target_filled__user_authorization__send_email_for_reset_password() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = UserAuthorization_SendEmailForResetPassword_Outcoming_ {
                    user_reset_password_token__can_be_resent_from: 0,
                };
                let unified_report =
                    UnifiedReport::<UserAuthorization_SendEmailForResetPassword_Outcoming_, UserAuthorization_SendEmailForResetPassword_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    user_authorization__send_email_for_reset_password__deserialize_allocate,
                    user_authorization__send_email_for_reset_password__deserialize_deallocate,
                );
            }
            fn _precedent__user_authorization__send_email_for_reset_password(
                precedent: UserAuthorization_SendEmailForResetPassword_Precedent_,
            ) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report =
                    UnifiedReport::<UserAuthorization_SendEmailForResetPassword_Outcoming_, UserAuthorization_SendEmailForResetPassword_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__send_email_for_reset_password__deserialize_allocate,
                    user_authorization__send_email_for_reset_password__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__send_email_for_reset_password() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_SendEmailForResetPassword_Precedent_::User_NotFound {
                    UserAuthorization_SendEmailForResetPassword_Precedent_::User_NotFound => {}
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken_NotFound => {}
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken_AlreadyExpired => {}
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken_AlreadyApproved => {}
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken_TimeToResendHasNotCome => {}
                }
                let mut precedents: Vec<UserAuthorization_SendEmailForResetPassword_Precedent_> = vec![];
                precedents.push(UserAuthorization_SendEmailForResetPassword_Precedent_::User_NotFound);
                precedents.push(UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken_NotFound);
                precedents.push(UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken_AlreadyExpired);
                precedents.push(UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken_AlreadyApproved);
                precedents.push(UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken_TimeToResendHasNotCome);
                '_a: for precedent in precedents {
                    _precedent__user_authorization__send_email_for_reset_password(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel__get_many_by_name_in_subscriptions() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_GetManyByNameInSubscriptions_Outcoming_, Channel_GetManyByNameInSubscriptions_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel__get_many_by_name_in_subscriptions__deserialize_allocate,
                    channel__get_many_by_name_in_subscriptions__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel__get_many_by_name_in_subscriptions() -> Result<(), Box<dyn StdError + 'static>> {
                let mut data_registry: Vec<Channel_GetManyByNameInSubscriptions_Data_> = vec![];
                '_a: for _ in 1..=5 {
                    let data = Channel_GetManyByNameInSubscriptions_Data_ {
                        channel__id: 0,
                        channel__name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__linked_name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__access_modifier: 0,
                        channel__visability_modifier: 0,
                        channel__background_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel__cover_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                    };
                    data_registry.push(data);
                }
                let outcoming = Channel_GetManyByNameInSubscriptions_Outcoming_ {
                    data_registry,
                };
                let unified_report = UnifiedReport::<Channel_GetManyByNameInSubscriptions_Outcoming_, Channel_GetManyByNameInSubscriptions_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    channel__get_many_by_name_in_subscriptions__deserialize_allocate,
                    channel__get_many_by_name_in_subscriptions__deserialize_deallocate,
                );
            }
            fn _precedent__channel__get_many_by_name_in_subscriptions(precedent: Channel_GetManyByNameInSubscriptions_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_GetManyByNameInSubscriptions_Outcoming_, Channel_GetManyByNameInSubscriptions_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel__get_many_by_name_in_subscriptions__deserialize_allocate,
                    channel__get_many_by_name_in_subscriptions__deserialize_deallocate,
                );
            }
            pub fn precedent__channel__get_many_by_name_in_subscriptions() -> Result<(), Box<dyn StdError + 'static>> {
                match Channel_GetManyByNameInSubscriptions_Precedent_::UserAccessToken_AlreadyExpired {
                    Channel_GetManyByNameInSubscriptions_Precedent_::UserAccessToken_AlreadyExpired => {}
                    Channel_GetManyByNameInSubscriptions_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {}
                }
                let mut precedents: Vec<Channel_GetManyByNameInSubscriptions_Precedent_> = vec![];
                precedents.push(Channel_GetManyByNameInSubscriptions_Precedent_::UserAccessToken_AlreadyExpired);
                precedents.push(Channel_GetManyByNameInSubscriptions_Precedent_::UserAccessToken_InUserAccessTokenBlackList);
                '_a: for precedent in precedents {
                    _precedent__channel__get_many_by_name_in_subscriptions(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel__get_many_by_subscription() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_GetManyBySubscription_Outcoming_, Channel_GetManyBySubscription_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel__get_many_by_subscription__deserialize_allocate,
                    channel__get_many_by_subscription__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel__get_many_by_subscription() -> Result<(), Box<dyn StdError + 'static>> {
                let mut data_registry: Vec<Channel_GetManyBySubscription_Data_> = vec![];
                '_a: for _ in 1..=2 {
                    let data = Channel_GetManyBySubscription_Data_ {
                        channel__id: 0,
                        channel__name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__linked_name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__access_modifier: 0,
                        channel__visability_modifier: 0,
                        channel__background_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel__cover_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                    };
                    data_registry.push(data);
                }
                let outcoming = Channel_GetManyBySubscription_Outcoming_ {
                    data_registry,
                };
                let unified_report = UnifiedReport::<Channel_GetManyBySubscription_Outcoming_, Channel_GetManyBySubscription_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    channel__get_many_by_subscription__deserialize_allocate,
                    channel__get_many_by_subscription__deserialize_deallocate,
                );
            }
            fn _precedent__channel__get_many_by_subscription(precedent: Channel_GetManyBySubscription_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_GetManyBySubscription_Outcoming_, Channel_GetManyBySubscription_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel__get_many_by_subscription__deserialize_allocate,
                    channel__get_many_by_subscription__deserialize_deallocate,
                );
            }
            pub fn precedent__channel__get_many_by_subscription() -> Result<(), Box<dyn StdError + 'static>> {
                match Channel_GetManyBySubscription_Precedent_::UserAccessToken_AlreadyExpired {
                    Channel_GetManyBySubscription_Precedent_::UserAccessToken_AlreadyExpired => {}
                    Channel_GetManyBySubscription_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {}
                }
                let mut precedents: Vec<Channel_GetManyBySubscription_Precedent_> = vec![];
                precedents.push(Channel_GetManyBySubscription_Precedent_::UserAccessToken_AlreadyExpired);
                precedents.push(Channel_GetManyBySubscription_Precedent_::UserAccessToken_InUserAccessTokenBlackList);
                '_a: for precedent in precedents {
                    _precedent__channel__get_many_by_subscription(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel__get_many_public_by_name() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_GetManyPublicByName_Outcoming_, Channel_GetManyPublicByName_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel__get_many_public_by_name__deserialize_allocate,
                    channel__get_many_public_by_name__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel__get_many_public_by_name() -> Result<(), Box<dyn StdError + 'static>> {
                let mut data_registry: Vec<Channel_GetManyPublicByName_Data_> = vec![];
                '_a: for _ in 1..=5 {
                    let data = Channel_GetManyPublicByName_Data_ {
                        channel__id: 0,
                        channel__name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__linked_name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__access_modifier: 0,
                        channel__background_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel__cover_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel_token_hashed_for_unsubscribed_users: Option::Some(
                            ChannelTokenHashed_ {
                                channel_token__expires_at: 0,
                                hash: 0,
                            }
                        ),
                    };
                    data_registry.push(data);
                }
                let outcoming = Channel_GetManyPublicByName_Outcoming_ {
                    data_registry,
                };
                let unified_report = UnifiedReport::<Channel_GetManyPublicByName_Outcoming_, Channel_GetManyPublicByName_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    channel__get_many_public_by_name__deserialize_allocate,
                    channel__get_many_public_by_name__deserialize_deallocate,
                );
            }
            fn _precedent__channel__get_many_public_by_name(precedent: Channel_GetManyPublicByName_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_GetManyPublicByName_Outcoming_, Channel_GetManyPublicByName_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel__get_many_public_by_name__deserialize_allocate,
                    channel__get_many_public_by_name__deserialize_deallocate,
                );
            }
            pub fn precedent__channel__get_many_public_by_name() -> Result<(), Box<dyn StdError + 'static>> {
                match Channel_GetManyPublicByName_Precedent_::UserAccessToken_AlreadyExpired {
                    Channel_GetManyPublicByName_Precedent_::UserAccessToken_AlreadyExpired => {}
                    Channel_GetManyPublicByName_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {}
                }
                let mut precedents: Vec<Channel_GetManyPublicByName_Precedent_> = vec![];
                precedents.push(Channel_GetManyPublicByName_Precedent_::UserAccessToken_AlreadyExpired);
                precedents.push(Channel_GetManyPublicByName_Precedent_::UserAccessToken_InUserAccessTokenBlackList);
                '_a: for precedent in precedents {
                    _precedent__channel__get_many_public_by_name(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel__get_one_by_id() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_GetOneById_Outcoming_, Channel_GetOneById_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel__get_one_by_id__deserialize_allocate,
                    channel__get_one_by_id__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel__get_one_by_id() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = Channel_GetOneById_Outcoming_ {
                    channel__name: NOT_EMPTY_STRING_LITERAL.to_string(),
                    channel__linked_name: NOT_EMPTY_STRING_LITERAL.to_string(),
                    channel__description: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                    channel__access_modifier: 0,
                    channel__visability_modifier: 0,
                    channel__orientation: vec![0, 0, 0],
                    channel__background_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                    channel__cover_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                    channel__subscribers_quantity: 0,
                    channel__marks_quantity: 0,
                    channel__viewing_quantity: 0,
                    user_is_channel_owner: true,
                    channel_subscription_token_hashed: ChannelSubscriptionTokenHashed_ {
                        channel_subscription_token__expires_at: 0,
                        hash: 0,
                    },
                };
                let unified_report = UnifiedReport::<Channel_GetOneById_Outcoming_, Channel_GetOneById_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    channel__get_one_by_id__deserialize_allocate,
                    channel__get_one_by_id__deserialize_deallocate,
                );
            }
            fn _precedent__channel__get_one_by_id(precedent: Channel_GetOneById_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_GetOneById_Outcoming_, Channel_GetOneById_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel__get_one_by_id__deserialize_allocate,
                    channel__get_one_by_id__deserialize_deallocate,
                );
            }
            pub fn precedent__channel__get_one_by_id() -> Result<(), Box<dyn StdError + 'static>> {
                match Channel_GetOneById_Precedent_::UserAccessToken_AlreadyExpired {
                    Channel_GetOneById_Precedent_::UserAccessToken_AlreadyExpired => {}
                    Channel_GetOneById_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {}
                    Channel_GetOneById_Precedent_::Channel_NotFound => {}
                    Channel_GetOneById_Precedent_::Channel_IsClose => {}
                    Channel_GetOneById_Precedent_::ChannelToken_NotFound => {}
                    Channel_GetOneById_Precedent_::ChannelToken_AlreadyExpired => {}
                }
                let mut precedents: Vec<Channel_GetOneById_Precedent_> = vec![];
                precedents.push(Channel_GetOneById_Precedent_::UserAccessToken_AlreadyExpired);
                precedents.push(Channel_GetOneById_Precedent_::UserAccessToken_InUserAccessTokenBlackList);
                precedents.push(Channel_GetOneById_Precedent_::Channel_NotFound);
                precedents.push(Channel_GetOneById_Precedent_::Channel_IsClose);
                precedents.push(Channel_GetOneById_Precedent_::ChannelToken_NotFound);
                precedents.push(Channel_GetOneById_Precedent_::ChannelToken_AlreadyExpired);
                '_a: for precedent in precedents {
                    _precedent__channel__get_one_by_id(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel_subscription__create() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelSubscription_Create_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel_subscription__create__deserialize_allocate,
                    channel_subscription__create__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel_subscription__create() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            fn _precedent__channel_subscription__create(precedent: ChannelSubscription_Create_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelSubscription_Create_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel_subscription__create__deserialize_allocate,
                    channel_subscription__create__deserialize_deallocate,
                );
            }
            pub fn precedent__channel_subscription__create() -> Result<(), Box<dyn StdError + 'static>> {
                match ChannelSubscription_Create_Precedent_::UserAccessToken_AlreadyExpired {
                    ChannelSubscription_Create_Precedent_::UserAccessToken_AlreadyExpired => {}
                    ChannelSubscription_Create_Precedent_::UserAccessToken_InUserAccessTokenBlackList => {}
                    ChannelSubscription_Create_Precedent_::Channel_NotFound => {}
                    ChannelSubscription_Create_Precedent_::Channel_IsClose => {}
                    ChannelSubscription_Create_Precedent_::User_IsChannelOwner => {}
                    ChannelSubscription_Create_Precedent_::ChannelSubscriptionToken_AlreadyExpired => {}
                }
                let mut precedents: Vec<ChannelSubscription_Create_Precedent_> = vec![];
                precedents.push(ChannelSubscription_Create_Precedent_::UserAccessToken_AlreadyExpired);
                precedents.push(ChannelSubscription_Create_Precedent_::UserAccessToken_InUserAccessTokenBlackList);
                precedents.push(ChannelSubscription_Create_Precedent_::Channel_NotFound);
                precedents.push(ChannelSubscription_Create_Precedent_::Channel_IsClose);
                precedents.push(ChannelSubscription_Create_Precedent_::User_IsChannelOwner);
                precedents.push(ChannelSubscription_Create_Precedent_::ChannelSubscriptionToken_AlreadyExpired);
                '_a: for precedent in precedents {
                    _precedent__channel_subscription__create(precedent)?;
                }
                return Result::Ok(());
            }
        }
        pub mod server_request_data_serialization {
            use super::*;
            fn run_by_template<I>(
                incoming: I,
                allocator: extern "C-unwind" fn(I) -> CResult<CVector<c_uchar>>,
                deallocator: extern "C-unwind" fn(CResult<CVector<c_uchar>>) -> (),
            ) -> Result<(), Box<dyn StdError + 'static>> {
                deallocator(allocator(incoming));
                return Result::Ok(());
            }
            pub fn user_authorization__authorize_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_AuthorizeByFirstStep_Incoming {
                    user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user__email___or___user__nickname: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user__password: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    user_authorization__authorize_by_first_step__serialize_allocate,
                    user_authorization__authorize_by_first_step__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_device__id);
                Allocator::<CString>::deallocate(incoming.user__email___or___user__nickname);
                Allocator::<CString>::deallocate(incoming.user__password);
                return Result::Ok(());
            }
            pub fn user_authorization__authorize_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_AuthorizeByLastStep_Incoming {
                    user__id: 0,
                    user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user_authorization_token__value: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    user_authorization__authorize_by_last_step__serialize_allocate,
                    user_authorization__authorize_by_last_step__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_device__id);
                Allocator::<CString>::deallocate(incoming.user_authorization_token__value);
                return Result::Ok(());
            }
            pub fn user_authorization__check_email_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_CheckEmailForExisting_Incoming {
                    user__email: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    user_authorization__check_email_for_existing__serialize_allocate,
                    user_authorization__check_email_for_existing__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user__email);
                return Result::Ok(());
            }
            pub fn user_authorization__check_nickname_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_CheckNicknameForExisting_Incoming {
                    user__nickname: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    user_authorization__check_nickname_for_existing__serialize_allocate,
                    user_authorization__check_nickname_for_existing__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user__nickname);
                return Result::Ok(());
            }
            pub fn user_authorization__deauthorize_from_all_devices() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_DeauthorizeFromAllDevices_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user_access_token__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    user_authorization__deauthorize_from_all_devices__serialize_allocate,
                    user_authorization__deauthorize_from_all_devices__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_access_token__id);
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                return Result::Ok(());
            }
            pub fn user_authorization__deauthorize_from_one_device() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_DeauthorizeFromOneDevice_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user_access_token__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    user_authorization__deauthorize_from_one_device__serialize_allocate,
                    user_authorization__deauthorize_from_one_device__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_access_token__id);
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                return Result::Ok(());
            }
            pub fn user_authorization__refresh_access_token() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_RefreshAccessToken_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user_access_token__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    user_access_refresh_token_signed: UserAccessRefreshTokenSigned {
                        user_access_refresh_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    user_authorization__refresh_access_token__serialize_allocate,
                    user_authorization__refresh_access_token__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_access_token__id);
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_refresh_token_signed.signature);
                return Result::Ok(());
            }
            pub fn user_authorization__register_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_RegisterByFirstStep_Incoming {
                    user__email: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    user_authorization__register_by_first_step__serialize_allocate,
                    user_authorization__register_by_first_step__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user__email);
                Allocator::<CString>::deallocate(incoming.user_device__id);
                return Result::Ok(());
            }
            pub fn user_authorization__register_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_RegisterBySecondStep_Incoming {
                    user__email: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user_registration_token__value: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    user_authorization__register_by_second_step__serialize_allocate,
                    user_authorization__register_by_second_step__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user__email);
                Allocator::<CString>::deallocate(incoming.user_device__id);
                Allocator::<CString>::deallocate(incoming.user_registration_token__value);
                return Result::Ok(());
            }
            pub fn user_authorization__register_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_RegisterByLastStep_Incoming {
                    user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user__nickname: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user__password: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user__email: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user_registration_token__value: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    user_authorization__register_by_last_step__serialize_allocate,
                    user_authorization__register_by_last_step__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_device__id);
                Allocator::<CString>::deallocate(incoming.user__nickname);
                Allocator::<CString>::deallocate(incoming.user__password);
                Allocator::<CString>::deallocate(incoming.user__email);
                Allocator::<CString>::deallocate(incoming.user_registration_token__value);
                return Result::Ok(());
            }
            pub fn user_authorization__reset_password_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_ResetPasswordByFirstStep_Incoming {
                    user__email: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    user_authorization__reset_password_by_first_step__serialize_allocate,
                    user_authorization__reset_password_by_first_step__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user__email);
                Allocator::<CString>::deallocate(incoming.user_device__id);
                return Result::Ok(());
            }
            pub fn user_authorization__reset_password_by_second_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_ResetPasswordBySecondStep_Incoming {
                    user__id: 0,
                    user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user_reset_password_token__value: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    user_authorization__reset_password_by_second_step__serialize_allocate,
                    user_authorization__reset_password_by_second_step__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_device__id);
                Allocator::<CString>::deallocate(incoming.user_reset_password_token__value);
                return Result::Ok(());
            }
            pub fn user_authorization__reset_password_by_last_step() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_ResetPasswordByLastStep_Incoming {
                    user__id: 0,
                    user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user__password: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user_reset_password_token__value: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    user_authorization__reset_password_by_last_step__serialize_allocate,
                    user_authorization__reset_password_by_last_step__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_device__id);
                Allocator::<CString>::deallocate(incoming.user__password);
                Allocator::<CString>::deallocate(incoming.user_reset_password_token__value);
                return Result::Ok(());
            }
            pub fn user_authorization__send_email_for_register() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_SendEmailForRegister_Incoming {
                    user__email: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    user_authorization__send_email_for_register__serialize_allocate,
                    user_authorization__send_email_for_register__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user__email);
                Allocator::<CString>::deallocate(incoming.user_device__id);
                return Result::Ok(());
            }
            pub fn user_authorization__send_email_for_authorize() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_SendEmailForAuthorize_Incoming {
                    user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    user__id: 0,
                };
                run_by_template(
                    incoming,
                    user_authorization__send_email_for_authorize__serialize_allocate,
                    user_authorization__send_email_for_authorize__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_device__id);
                return Result::Ok(());
            }
            pub fn user_authorization__send_email_for_reset_password() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_SendEmailForResetPassword_Incoming {
                    user__id: 0,
                    user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    user_authorization__send_email_for_reset_password__serialize_allocate,
                    user_authorization__send_email_for_reset_password__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_device__id);
                return Result::Ok(());
            }
            pub fn channel__get_many_by_name_in_subscriptions() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel_GetManyByNameInSubscriptions_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user_access_token__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel__name: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    requery___channel__name: COption::data(Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string())),
                    limit: 0,
                };
                run_by_template(
                    incoming,
                    channel__get_many_by_name_in_subscriptions__serialize_allocate,
                    channel__get_many_by_name_in_subscriptions__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_access_token__id);
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CString>::deallocate(incoming.channel__name);
                Allocator::<CString>::deallocate(incoming.requery___channel__name.data);
                return Result::Ok(());
            }
            pub fn channel__get_many_by_subscription() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel_GetManyBySubscription_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user_access_token__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    requery___channel__id: COption::data(0),
                    limit: 0,
                };
                run_by_template(
                    incoming,
                    channel__get_many_by_subscription__serialize_allocate,
                    channel__get_many_by_subscription__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_access_token__id);
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                return Result::Ok(());
            }
            pub fn channel__get_many_public_by_name() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel_GetManyPublicByName_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user_access_token__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel__name: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    requery___channel__name: COption::data(Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string())),
                    limit: 0,
                };
                run_by_template(
                    incoming,
                    channel__get_many_public_by_name__serialize_allocate,
                    channel__get_many_public_by_name__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_access_token__id);
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CString>::deallocate(incoming.channel__name);
                Allocator::<CString>::deallocate(incoming.requery___channel__name.data);
                return Result::Ok(());
            }
            pub fn channel__get_one_by_id() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel_GetOneById_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user_access_token__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel__id: 0,
                    channel_token_hashed: COption::data(
                        ChannelTokenHashed {
                            channel_token__expires_at: 0,
                            hash: 0,
                        }
                    ),
                };
                run_by_template(
                    incoming,
                    channel__get_one_by_id__serialize_allocate,
                    channel__get_one_by_id__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_access_token__id);
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                return Result::Ok(());
            }
            pub fn channel_subscription__create() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ChannelSubscription_Create_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user_access_token__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel__id: 0,
                    channel_subscription_token_hashed: ChannelSubscriptionTokenHashed {
                        channel_subscription_token__expires_at: 0,
                        hash: 0,
                    }
                };
                run_by_template(
                    incoming,
                    channel_subscription__create__serialize_allocate,
                    channel_subscription__create__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_access_token__id);
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                return Result::Ok(());
            }
        }
    }
}
