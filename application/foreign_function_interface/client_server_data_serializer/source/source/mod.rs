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
                check_linked_name_for_existing::{
                    Incoming as Channel_CheckLinkedNameForExisting_Incoming_,
                    Outcoming as Channel_CheckLinkedNameForExisting_Outcoming_,
                    Precedent as Channel_CheckLinkedNameForExisting_Precedent_,
                },
                check_name_for_existing::{
                    Incoming as Channel_CheckNameForExisting_Incoming_,
                    Outcoming as Channel_CheckNameForExisting_Outcoming_,
                    Precedent as Channel_CheckNameForExisting_Precedent_,
                },
                create::{
                    Incoming as Channel_Create_Incoming_,
                    Outcoming as Channel_Create_Outcoming_,
                    Precedent as Channel_Create_Precedent_,
                },
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
            channel_publication1::{
                create::{
                    Incoming as ChannelPublication1_Create_Incoming_,
                    Outcoming as ChannelPublication1_Create_Outcoming_,
                    Precedent as ChannelPublication1_Create_Precedent_,
                },
                delete::{
                    Incoming as ChannelPublication1_Delete_Incoming_,
                    Precedent as ChannelPublication1_Delete_Precedent_,
                },
                get_many::{
                    Incoming as ChannelPublication1_GetMany_Incoming_,
                    Outcoming as ChannelPublication1_GetMany_Outcoming_,
                    Precedent as ChannelPublication1_GetMany_Precedent_,
                },
            },
            channel_publication1_commentary::{
                create::{
                    Incoming as ChannelPublication1Commentary_Create_Incoming_,
                    Outcoming as ChannelPublication1Commentary_Create_Outcoming_,
                    Precedent as ChannelPublication1Commentary_Create_Precedent_,
                },
                delete::{
                    Incoming as ChannelPublication1Commentary_Delete_Incoming_,
                    Precedent as ChannelPublication1Commentary_Delete_Precedent_,
                },
            },
            channel_publication1_mark::{
                create::{
                    Incoming as ChannelPublication1Mark_Create_Incoming_,
                    Precedent as ChannelPublication1Mark_Create_Precedent_,
                },
                delete::{
                    Incoming as ChannelPublication1Mark_Delete_Incoming_,
                    Precedent as ChannelPublication1Mark_Delete_Precedent_,
                },
            },
            channel_publication1_view::create::{
                Incoming as ChannelPublication1View_Create_Incoming_,
                Precedent as ChannelPublication1View_Create_Precedent_,
            },
            channel_subscription::{
                create::{
                    Incoming as ChannelSubscription_Create_Incoming_,
                    Precedent as ChannelSubscription_Create_Precedent_,
                },
                delete::{
                    Incoming as ChannelSubscription_Delete_Incoming_,
                    Precedent as ChannelSubscription_Delete_Precedent_,
                },
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
        channel_publication1_token_signed::ChannelPublication1TokenSigned as ChannelPublication1TokenSigned_,
        channel_token_signed::ChannelTokenSigned as ChannelTokenSigned_,
        unified_report::{
            Data,
            UnifiedReport,
        },
        user_access_refresh_token_signed::UserAccessRefreshTokenSigned as UserAccessRefreshTokenSigned_,
        user_access_token_signed::UserAccessTokenSigned as UserAccessTokenSigned_,
        void::Void,
    },
    libc::{
        c_char,
        c_long,
        c_uchar,
        c_uint,
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
    pub user__id: c_long,
    pub user_device__id: CString,
    pub user_access_token__obfuscation_value: c_long,
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
pub struct ChannelTokenSigned {
    pub channel__id: c_long,
    pub channel_token__obfuscation_value: c_long,
    pub channel_token__expires_at: c_long,
    pub channel_token__is_user_subscribed: bool,
    pub channel_token__is_user_the_owner: bool,
    pub signature: CVector<c_uchar>,
}
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct ChannelPublication1TokenSigned {
    pub channel__id: c_long,
    pub channel_publication1__id: c_long,
    pub channel_publication1_token__obfuscation_value: c_long,
    pub channel_publication1_token__expires_at: c_long,
    pub signature: CVector<c_uchar>,
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
    pub user_authorization_token__wrong_enter_tries_quantity: c_uchar,
    pub user_authorization_token__wrong_enter_tries_quantity_limit: c_uchar,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_AuthorizeByFirstStep_Precedent {
    pub user___wrong_email_or_nickname_or_password: bool,
    pub parallel_execution: bool,
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
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let outcoming = UserAuthorization_AuthorizeByFirstStep_Outcoming {
                            user__id: data_.user__id,
                            verification_message_sent: data_.verification_message_sent,
                            user_authorization_token__can_be_resent_from: data_.user_authorization_token__can_be_resent_from,
                            user_authorization_token__wrong_enter_tries_quantity: data_.user_authorization_token__wrong_enter_tries_quantity,
                            user_authorization_token__wrong_enter_tries_quantity_limit: data_.user_authorization_token__wrong_enter_tries_quantity_limit,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_AuthorizeByFirstStep_Precedent_::User__WrongEmailOrNicknameOrPassword => {
                        UserAuthorization_AuthorizeByFirstStep_Precedent {
                            user___wrong_email_or_nickname_or_password: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_AuthorizeByFirstStep_Precedent_::ParallelExecution => {
                        UserAuthorization_AuthorizeByFirstStep_Precedent {
                            parallel_execution: true,
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
    pub user_authorization_token___not_found: bool,
    pub user_authorization_token___already_expired: bool,
    pub user_authorization_token___wrong_value: UserAuthorizationToken__WrongValue,
    pub user___not_found: bool,
    pub parallel_execution: bool,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorizationToken__WrongValue {
    pub is_exist: bool,
    pub user_authorization_token__wrong_enter_tries_quantity: c_uchar,
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
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let outcoming = UserAuthorization_AuthorizeByLastStep_Outcoming {
                            user_access_token_signed: UserAccessTokenSigned {
                                user__id: data_.user_access_token_signed.user__id,
                                user_device__id: Allocator::<CString>::allocate(data_.user_access_token_signed.user_device__id),
                                user_access_token__obfuscation_value: data_.user_access_token_signed.user_access_token__obfuscation_value,
                                user_access_token__expires_at: data_.user_access_token_signed.user_access_token__expires_at,
                                signature: Allocator::<CVector<_>>::allocate(data_.user_access_token_signed.signature),
                            },
                            user_access_refresh_token_signed: UserAccessRefreshTokenSigned {
                                user_access_refresh_token__expires_at: data_.user_access_refresh_token_signed.user_access_refresh_token__expires_at,
                                signature: Allocator::<CVector<_>>::allocate(data_.user_access_refresh_token_signed.signature),
                            },
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken__AlreadyExpired => {
                        UserAuthorization_AuthorizeByLastStep_Precedent {
                            user_authorization_token___already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken__NotFound => {
                        UserAuthorization_AuthorizeByLastStep_Precedent {
                            user_authorization_token___not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken__WrongValue {
                        user_authorization_token__wrong_enter_tries_quantity,
                    } => {
                        UserAuthorization_AuthorizeByLastStep_Precedent {
                            user_authorization_token___wrong_value: UserAuthorizationToken__WrongValue {
                                is_exist: true,
                                user_authorization_token__wrong_enter_tries_quantity,
                            },
                            ..Default::default()
                        }
                    }
                    UserAuthorization_AuthorizeByLastStep_Precedent_::User__NotFound => {
                        UserAuthorization_AuthorizeByLastStep_Precedent {
                            user___not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_AuthorizeByLastStep_Precedent_::ParallelExecution => {
                        UserAuthorization_AuthorizeByLastStep_Precedent {
                            parallel_execution: true,
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
        let c_data = match data {
            Data::Empty => {
                CData::empty()
            }
            Data::Filled { data: data_ } => {
                let outcoming = UserAuthorization_CheckEmailForExisting_Outcoming {
                    result: data_.result,
                };
                CData::filled(outcoming)
            }
        };
        return Result::Ok(CUnifiedReport::target(c_data));
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
        let c_data = match data {
            Data::Empty => {
                CData::empty()
            }
            Data::Filled { data: data_ } => {
                let outcoming = UserAuthorization_CheckNicknameForExisting_Outcoming {
                    result: data_.result,
                };
                CData::filled(outcoming)
            }
        };
        return Result::Ok(CUnifiedReport::target(c_data));
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
                        user__id: incoming_.user_access_token_signed.user__id,
                        user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                        user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                        user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                        signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
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
    pub user_access_token___already_expired: bool,
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
                    UserAuthorization_DeauthorizeFromAllDevices_Precedent_::UserAccessToken__AlreadyExpired => {
                        UserAuthorization_DeauthorizeFromAllDevices_Precedent {
                            user_access_token___already_expired: true,
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
    pub user_access_token___already_expired: bool,
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
                        user__id: incoming_.user_access_token_signed.user__id,
                        user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                        user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                        user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                        signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
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
                    UserAuthorization_DeauthorizeFromOneDevice_Precedent_::UserAccessToken__AlreadyExpired => {
                        UserAuthorization_DeauthorizeFromOneDevice_Precedent {
                            user_access_token___already_expired: true,
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
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
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
    pub user_access_refresh_token___not_found: bool,
    pub user_access_refresh_token___already_expired: bool,
    pub parallel_execution: bool,
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
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let outcoming = UserAuthorization_RefreshAccessToken_Outcoming {
                            user_access_token_signed: UserAccessTokenSigned {
                                user__id: data_.user_access_token_signed.user__id,
                                user_device__id: Allocator::<CString>::allocate(data_.user_access_token_signed.user_device__id),
                                user_access_token__obfuscation_value: data_.user_access_token_signed.user_access_token__obfuscation_value,
                                user_access_token__expires_at: data_.user_access_token_signed.user_access_token__expires_at,
                                signature: Allocator::<CVector<_>>::allocate(data_.user_access_token_signed.signature),
                            },
                            user_access_refresh_token_signed: UserAccessRefreshTokenSigned {
                                user_access_refresh_token__expires_at: data_.user_access_refresh_token_signed.user_access_refresh_token__expires_at,
                                signature: Allocator::<CVector<_>>::allocate(data_.user_access_refresh_token_signed.signature),
                            },
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken__AlreadyExpired => {
                        UserAuthorization_RefreshAccessToken_Precedent {
                            user_access_refresh_token___already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken__NotFound => {
                        UserAuthorization_RefreshAccessToken_Precedent {
                            user_access_refresh_token___not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RefreshAccessToken_Precedent_::ParallelExecution => {
                        UserAuthorization_RefreshAccessToken_Precedent {
                            parallel_execution: true,
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
    pub user_registration_token__wrong_enter_tries_quantity: c_uchar,
    pub user_registration_token__wrong_enter_tries_quantity_limit: c_uchar,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_RegisterByFirstStep_Precedent {
    pub user___email_already_exist: bool,
    pub parallel_execution: bool,
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
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let outcoming = UserAuthorization_RegisterByFirstStep_Outcoming {
                            verification_message_sent: data_.verification_message_sent,
                            user_registration_token__can_be_resent_from: data_.user_registration_token__can_be_resent_from,
                            user_registration_token__wrong_enter_tries_quantity: data_.user_registration_token__wrong_enter_tries_quantity,
                            user_registration_token__wrong_enter_tries_quantity_limit: data_.user_registration_token__wrong_enter_tries_quantity_limit,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_RegisterByFirstStep_Precedent_::User__EmailAlreadyExist => {
                        UserAuthorization_RegisterByFirstStep_Precedent {
                            user___email_already_exist: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterByFirstStep_Precedent_::ParallelExecution => {
                        UserAuthorization_RegisterByFirstStep_Precedent {
                            parallel_execution: true,
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
    pub user_registration_token___not_found: bool,
    pub user_registration_token___already_expired: bool,
    pub user_registration_token___already_approved: bool,
    pub user_registration_token___wrong_value: UserRegistrationToken__WrongValue,
    pub parallel_execution: bool,
}
#[repr(C)]
#[derive(Default)]
pub struct UserRegistrationToken__WrongValue {
    pub is_exist: bool,
    pub user_registration_token__wrong_enter_tries_quantity: c_uchar,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn user_authorization__register_by_second_step__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> UserAuthorization_RegisterBySecondStep_CResult {
    let converter = move |unified_report: UnifiedReport<Void, UserAuthorization_RegisterBySecondStep_Precedent_>| -> Result<CUnifiedReport<CVoid, UserAuthorization_RegisterBySecondStep_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken__NotFound => UserAuthorization_RegisterBySecondStep_Precedent {
                        user_registration_token___not_found: true,
                        ..Default::default()
                    },
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken__AlreadyExpired => UserAuthorization_RegisterBySecondStep_Precedent {
                        user_registration_token___already_expired: true,
                        ..Default::default()
                    },
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken__AlreadyApproved =>
                    UserAuthorization_RegisterBySecondStep_Precedent {
                        user_registration_token___already_approved: true,
                        ..Default::default()
                    },
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken__WrongValue {
                        user_registration_token__wrong_enter_tries_quantity
                    } => UserAuthorization_RegisterBySecondStep_Precedent {
                        user_registration_token___wrong_value: UserRegistrationToken__WrongValue {
                            is_exist: true,
                            user_registration_token__wrong_enter_tries_quantity,
                        },
                        ..Default::default()
                    },
                    UserAuthorization_RegisterBySecondStep_Precedent_::ParallelExecution => UserAuthorization_RegisterBySecondStep_Precedent {
                        parallel_execution: true,
                        ..Default::default()
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
    pub user___nickname_already_exist: bool,
    pub user___email_already_exist: bool,
    pub user_registration_token___not_found: bool,
    pub user_registration_token___already_expired: bool,
    pub user_registration_token___is_not_approved: bool,
    pub user_registration_token___wrong_value: bool,
    pub parallel_execution: bool,
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
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let outcoming = UserAuthorization_RegisterByLastStep_Outcoming {
                            user_access_token_signed: UserAccessTokenSigned {
                                user__id: data_.user_access_token_signed.user__id,
                                user_device__id: Allocator::<CString>::allocate(data_.user_access_token_signed.user_device__id),
                                user_access_token__obfuscation_value: data_.user_access_token_signed.user_access_token__obfuscation_value,
                                user_access_token__expires_at: data_.user_access_token_signed.user_access_token__expires_at,
                                signature: Allocator::<CVector<_>>::allocate(data_.user_access_token_signed.signature),
                            },
                            user_access_refresh_token_signed: UserAccessRefreshTokenSigned {
                                user_access_refresh_token__expires_at: data_.user_access_refresh_token_signed.user_access_refresh_token__expires_at,
                                signature: Allocator::<CVector<_>>::allocate(data_.user_access_refresh_token_signed.signature),
                            },
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_RegisterByLastStep_Precedent_::User__NicknameAlreadyExist => {
                        UserAuthorization_RegisterByLastStep_Precedent {
                            user___nickname_already_exist: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterByLastStep_Precedent_::User__EmailAlreadyExist => {
                        UserAuthorization_RegisterByLastStep_Precedent {
                            user___email_already_exist: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken__NotFound => {
                        UserAuthorization_RegisterByLastStep_Precedent {
                            user_registration_token___not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken__AlreadyExpired => {
                        UserAuthorization_RegisterByLastStep_Precedent {
                            user_registration_token___already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken__IsNotApproved => {
                        UserAuthorization_RegisterByLastStep_Precedent {
                            user_registration_token___is_not_approved: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken__WrongValue => {
                        UserAuthorization_RegisterByLastStep_Precedent {
                            user_registration_token___wrong_value: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_RegisterByLastStep_Precedent_::ParallelExecution => {
                        UserAuthorization_RegisterByLastStep_Precedent {
                            parallel_execution: true,
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
    pub user_reset_password_token__wrong_enter_tries_quantity: c_uchar,
    pub user_reset_password_token__wrong_enter_tries_quantity_limit: c_uchar,
}
#[repr(C)]
#[derive(Default)]
pub struct UserAuthorization_ResetPasswordByFirstStep_Precedent {
    pub user___not_found: bool,
    pub parallel_execution: bool,
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
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let outcoming = UserAuthorization_ResetPasswordByFirstStep_Outcoming {
                            user__id: data_.user__id,
                            verification_message_sent: data_.verification_message_sent,
                            user_reset_password_token__can_be_resent_from: data_.user_reset_password_token__can_be_resent_from,
                            user_reset_password_token__wrong_enter_tries_quantity: data_.user_reset_password_token__wrong_enter_tries_quantity,
                            user_reset_password_token__wrong_enter_tries_quantity_limit: data_.user_reset_password_token__wrong_enter_tries_quantity_limit,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_ResetPasswordByFirstStep_Precedent_::User__NotFound => {
                        UserAuthorization_ResetPasswordByFirstStep_Precedent {
                            user___not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_ResetPasswordByFirstStep_Precedent_::ParallelExecution => {
                        UserAuthorization_ResetPasswordByFirstStep_Precedent {
                            parallel_execution: true,
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
    pub user_reset_password_token___not_found: bool,
    pub user_reset_password_token___already_expired: bool,
    pub user_reset_password_token___already_approved: bool,
    pub user_reset_password_token___wrong_value: UserResetPasswordToken__WrongValue,
    pub parallel_execution: bool,
}
#[repr(C)]
#[derive(Default)]
pub struct UserResetPasswordToken__WrongValue {
    pub is_exist: bool,
    pub user_reset_password_token__wrong_enter_tries_quantity: c_uchar,
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
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken__NotFound => UserAuthorization_ResetPasswordBySecondStep_Precedent {
                        user_reset_password_token___not_found: true,
                        ..Default::default()
                    },
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken__AlreadyExpired => UserAuthorization_ResetPasswordBySecondStep_Precedent {
                        user_reset_password_token___already_expired: true,
                        ..Default::default()
                    },
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken__AlreadyApproved => UserAuthorization_ResetPasswordBySecondStep_Precedent {
                        user_reset_password_token___already_approved: true,
                        ..Default::default()
                    },
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken__WrongValue {
                        user_reset_password_token__wrong_enter_tries_quantity
                    } => UserAuthorization_ResetPasswordBySecondStep_Precedent {
                        user_reset_password_token___wrong_value: UserResetPasswordToken__WrongValue {
                            is_exist: true,
                            user_reset_password_token__wrong_enter_tries_quantity,
                        },
                        ..Default::default()
                    },
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::ParallelExecution => UserAuthorization_ResetPasswordBySecondStep_Precedent {
                        parallel_execution: true,
                        ..Default::default()
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
    pub user___not_found: bool,
    pub user_reset_password_token___not_found: bool,
    pub user_reset_password_token___already_expired: bool,
    pub user_reset_password_token___is_not_approved: bool,
    pub user_reset_password_token___wrong_value: bool,
    pub parallel_execution: bool,
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
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::User__NotFound => UserAuthorization_ResetPasswordByLastStep_Precedent {
                        user___not_found: true,
                        ..Default::default()
                    },
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken__NotFound => UserAuthorization_ResetPasswordByLastStep_Precedent {
                        user_reset_password_token___not_found: true,
                        ..Default::default()
                    },
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken__AlreadyExpired => UserAuthorization_ResetPasswordByLastStep_Precedent {
                        user_reset_password_token___already_expired: true,
                        ..Default::default()
                    },
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken__IsNotApproved => UserAuthorization_ResetPasswordByLastStep_Precedent {
                        user_reset_password_token___is_not_approved: true,
                        ..Default::default()
                    },
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken__WrongValue => UserAuthorization_ResetPasswordByLastStep_Precedent {
                        user_reset_password_token___wrong_value: true,
                        ..Default::default()
                    },
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::ParallelExecution => UserAuthorization_ResetPasswordByLastStep_Precedent {
                        parallel_execution: true,
                        ..Default::default()
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
    pub user_registration_token___not_found: bool,
    pub user_registration_token___already_expired: bool,
    pub user_registration_token___already_approved: bool,
    pub user_registration_token___time_to_resend_has_not_come: bool,
    pub parallel_execution: bool,
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
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let outcoming = UserAuthorization_SendEmailForRegister_Outcoming {
                            user_registration_token__can_be_resent_from: data_.user_registration_token__can_be_resent_from,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken__NotFound => {
                        UserAuthorization_SendEmailForRegister_Precedent {
                            user_registration_token___not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken__AlreadyExpired => {
                        UserAuthorization_SendEmailForRegister_Precedent {
                            user_registration_token___already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken__AlreadyApproved => {
                        UserAuthorization_SendEmailForRegister_Precedent {
                            user_registration_token___already_approved: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken__TimeToResendHasNotCome => {
                        UserAuthorization_SendEmailForRegister_Precedent {
                            user_registration_token___time_to_resend_has_not_come: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForRegister_Precedent_::ParallelExecution => {
                        UserAuthorization_SendEmailForRegister_Precedent {
                            parallel_execution: true,
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
    pub user___not_found: bool,
    pub user_authorization_token___not_found: bool,
    pub user_authorization_token___already_expired: bool,
    pub user_authorization_token___time_to_resend_has_not_come: bool,
    pub parallel_execution: bool,
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
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let outcoming = UserAuthorization_SendEmailForAuthorize_Outcoming {
                            user_authorization_token__can_be_resent_from: data_.user_authorization_token__can_be_resent_from,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_SendEmailForAuthorize_Precedent_::User__NotFound => {
                        UserAuthorization_SendEmailForAuthorize_Precedent {
                            user___not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken__NotFound => {
                        UserAuthorization_SendEmailForAuthorize_Precedent {
                            user_authorization_token___not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken__AlreadyExpired => {
                        UserAuthorization_SendEmailForAuthorize_Precedent {
                            user_authorization_token___already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken__TimeToResendHasNotCome => {
                        UserAuthorization_SendEmailForAuthorize_Precedent {
                            user_authorization_token___time_to_resend_has_not_come: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForAuthorize_Precedent_::ParallelExecution => {
                        UserAuthorization_SendEmailForAuthorize_Precedent {
                            parallel_execution: true,
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
    pub user___not_found: bool,
    pub user_reset_password_token___not_found: bool,
    pub user_reset_password_token___already_expired: bool,
    pub user_reset_password_token___already_approved: bool,
    pub user_reset_password_token___time_to_resend_has_not_come: bool,
    pub parallel_execution: bool,
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
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let outcoming = UserAuthorization_SendEmailForResetPassword_Outcoming {
                            user_resep_password_token_can_be_resent_from: data_.user_reset_password_token__can_be_resent_from,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    UserAuthorization_SendEmailForResetPassword_Precedent_::User__NotFound => {
                        UserAuthorization_SendEmailForResetPassword_Precedent {
                            user___not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken__NotFound => {
                        UserAuthorization_SendEmailForResetPassword_Precedent {
                            user_reset_password_token___not_found: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken__AlreadyExpired => {
                        UserAuthorization_SendEmailForResetPassword_Precedent {
                            user_reset_password_token___already_expired: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken__AlreadyApproved => {
                        UserAuthorization_SendEmailForResetPassword_Precedent {
                            user_reset_password_token___already_approved: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken__TimeToResendHasNotCome => {
                        UserAuthorization_SendEmailForResetPassword_Precedent {
                            user_reset_password_token___time_to_resend_has_not_come: true,
                            ..Default::default()
                        }
                    }
                    UserAuthorization_SendEmailForResetPassword_Precedent_::ParallelExecution => {
                        UserAuthorization_SendEmailForResetPassword_Precedent {
                            parallel_execution: true,
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
    pub limit: c_uchar,
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
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
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
    pub channel__name: CString,
    pub channel__linked_name: CString,
    pub channel__access_modifier: c_uchar,
    pub channel__visability_modifier: c_uchar,
    pub channel__cover_image_path: COption<CString>,
    pub channel__background_image_path: COption<CString>,
    pub channel_token_signed: ChannelTokenSigned,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyByNameInSubscriptions_Outcoming {
    pub data_registry: CVector<Channel_GetManyByNameInSubscriptions_Data>,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyByNameInSubscriptions_Precedent {
    pub user_access_token___already_expired: bool,
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
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let mut data_registry: Vec<Channel_GetManyByNameInSubscriptions_Data> = Vec::with_capacity(data_.data_registry.len());
                        '_a: for data__ in data_.data_registry {
                            let channel__cover_image_path = match data__.channel__cover_image_path {
                                Option::Some(channel__cover_image_path_) => COption::data(Allocator::<CString>::allocate(channel__cover_image_path_)),
                                Option::None => COption::none(),
                            };
                            let channel__background_image_path = match data__.channel__background_image_path {
                                Option::Some(channel__background_image_path_) => COption::data(Allocator::<CString>::allocate(channel__background_image_path_)),
                                Option::None => COption::none(),
                            };
                            data_registry.push(
                                Channel_GetManyByNameInSubscriptions_Data {
                                    channel__name: Allocator::<CString>::allocate(data__.channel__name),
                                    channel__linked_name: Allocator::<CString>::allocate(data__.channel__linked_name),
                                    channel__access_modifier: data__.channel__access_modifier,
                                    channel__visability_modifier: data__.channel__visability_modifier,
                                    channel__cover_image_path,
                                    channel__background_image_path,
                                    channel_token_signed: ChannelTokenSigned {
                                        channel__id: data__.channel_token_signed.channel__id,
                                        channel_token__obfuscation_value: data__.channel_token_signed.channel_token__obfuscation_value,
                                        channel_token__expires_at: data__.channel_token_signed.channel_token__expires_at,
                                        channel_token__is_user_subscribed: data__.channel_token_signed.channel_token__is_user_subscribed,
                                        channel_token__is_user_the_owner: data__.channel_token_signed.channel_token__is_user_the_owner,
                                        signature: Allocator::<CVector<_>>::allocate(data__.channel_token_signed.signature),
                                    },
                                },
                            );
                        }
                        let outcoming = Channel_GetManyByNameInSubscriptions_Outcoming {
                            data_registry: Allocator::<CVector<_>>::allocate(data_registry),
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    Channel_GetManyByNameInSubscriptions_Precedent_::UserAccessToken__AlreadyExpired => {
                        Channel_GetManyByNameInSubscriptions_Precedent {
                            user_access_token___already_expired: true,
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
        '_a: for data in data_registry {
            Allocator::<CString>::deallocate(data.channel__name);
            Allocator::<CString>::deallocate(data.channel__linked_name);
            if data.channel__background_image_path.is_data {
                Allocator::<CString>::deallocate(data.channel__background_image_path.data);
            }
            if data.channel__cover_image_path.is_data {
                Allocator::<CString>::deallocate(data.channel__cover_image_path.data);
            }
            Allocator::<CVector<_>>::deallocate(data.channel_token_signed.signature);
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
    pub limit: c_uchar,
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
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
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
    pub channel__name: CString,
    pub channel__linked_name: CString,
    pub channel__access_modifier: c_uchar,
    pub channel__visability_modifier: c_uchar,
    pub channel__cover_image_path: COption<CString>,
    pub channel__background_image_path: COption<CString>,
    pub channel_token_signed: ChannelTokenSigned,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyBySubscription_Outcoming {
    pub data_registry: CVector<Channel_GetManyBySubscription_Data>,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyBySubscription_Precedent {
    pub user_access_token___already_expired: bool,
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
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let mut data_registry: Vec<Channel_GetManyBySubscription_Data> = Vec::with_capacity(data_.data_registry.len());
                        '_a: for data__ in data_.data_registry {
                            let channel__cover_image_path = match data__.channel__cover_image_path {
                                Option::Some(channel__cover_image_path_) => COption::data(Allocator::<CString>::allocate(channel__cover_image_path_)),
                                Option::None => COption::none(),
                            };
                            let channel__background_image_path = match data__.channel__background_image_path {
                                Option::Some(channel__background_image_path_) => COption::data(Allocator::<CString>::allocate(channel__background_image_path_)),
                                Option::None => COption::none(),
                            };
                            data_registry.push(
                                Channel_GetManyBySubscription_Data {
                                    channel__name: Allocator::<CString>::allocate(data__.channel__name),
                                    channel__linked_name: Allocator::<CString>::allocate(data__.channel__linked_name),
                                    channel__access_modifier: data__.channel__access_modifier,
                                    channel__visability_modifier: data__.channel__visability_modifier,
                                    channel__cover_image_path,
                                    channel__background_image_path,
                                    channel_token_signed: ChannelTokenSigned {
                                        channel__id: data__.channel_token_signed.channel__id,
                                        channel_token__obfuscation_value: data__.channel_token_signed.channel_token__obfuscation_value,
                                        channel_token__expires_at: data__.channel_token_signed.channel_token__expires_at,
                                        channel_token__is_user_subscribed: data__.channel_token_signed.channel_token__is_user_subscribed,
                                        channel_token__is_user_the_owner: data__.channel_token_signed.channel_token__is_user_the_owner,
                                        signature: Allocator::<CVector<_>>::allocate(data__.channel_token_signed.signature),
                                    },
                                }
                            );
                        }
                        let outcoming = Channel_GetManyBySubscription_Outcoming {
                            data_registry: Allocator::<CVector<_>>::allocate(data_registry),
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    Channel_GetManyBySubscription_Precedent_::UserAccessToken__AlreadyExpired => Channel_GetManyBySubscription_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
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
        '_a: for data in data_registry {
            Allocator::<CString>::deallocate(data.channel__name);
            Allocator::<CString>::deallocate(data.channel__linked_name);
            if data.channel__background_image_path.is_data {
                Allocator::<CString>::deallocate(data.channel__background_image_path.data);
            }
            if data.channel__cover_image_path.is_data {
                Allocator::<CString>::deallocate(data.channel__cover_image_path.data);
            }
            Allocator::<CVector<_>>::deallocate(data.channel_token_signed.signature);
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
    pub limit: c_uchar,
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
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
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
    pub channel__name: CString,
    pub channel__linked_name: CString,
    pub channel__access_modifier: c_uchar,
    pub channel__cover_image_path: COption<CString>,
    pub channel__background_image_path: COption<CString>,
    pub channel_token_signed: ChannelTokenSigned,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyPublicByName_Outcoming {
    pub data_registry: CVector<Channel_GetManyPublicByName_Data>,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetManyPublicByName_Precedent {
    pub user_access_token___already_expired: bool,
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
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let mut data_registry: Vec<Channel_GetManyPublicByName_Data> = Vec::with_capacity(data_.data_registry.len());
                        '_a: for data__ in data_.data_registry {
                            let channel__cover_image_path = match data__.channel__cover_image_path {
                                Option::Some(channel__cover_image_path_) => COption::data(Allocator::<CString>::allocate(channel__cover_image_path_)),
                                Option::None => COption::none(),
                            };
                            let channel__background_image_path = match data__.channel__background_image_path {
                                Option::Some(channel__background_image_path_) => COption::data(Allocator::<CString>::allocate(channel__background_image_path_)),
                                Option::None => COption::none(),
                            };
                            data_registry.push(
                                Channel_GetManyPublicByName_Data {
                                    channel__name: Allocator::<CString>::allocate(data__.channel__name),
                                    channel__linked_name: Allocator::<CString>::allocate(data__.channel__linked_name),
                                    channel__access_modifier: data__.channel__access_modifier,
                                    channel__cover_image_path,
                                    channel__background_image_path,
                                    channel_token_signed: ChannelTokenSigned {
                                        channel__id: data__.channel_token_signed.channel__id,
                                        channel_token__obfuscation_value: data__.channel_token_signed.channel_token__obfuscation_value,
                                        channel_token__expires_at: data__.channel_token_signed.channel_token__expires_at,
                                        channel_token__is_user_subscribed: data__.channel_token_signed.channel_token__is_user_subscribed,
                                        channel_token__is_user_the_owner: data__.channel_token_signed.channel_token__is_user_the_owner,
                                        signature: Allocator::<CVector<_>>::allocate(data__.channel_token_signed.signature),
                                    },
                                }
                            );
                        }
                        let outcoming = Channel_GetManyPublicByName_Outcoming {
                            data_registry: Allocator::<CVector<_>>::allocate(data_registry),
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    Channel_GetManyPublicByName_Precedent_::UserAccessToken__AlreadyExpired => Channel_GetManyPublicByName_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
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
        '_a: for data in data_registry {
            Allocator::<CString>::deallocate(data.channel__name);
            Allocator::<CString>::deallocate(data.channel__linked_name);
            if data.channel__background_image_path.is_data {
                Allocator::<CString>::deallocate(data.channel__background_image_path.data);
            }
            if data.channel__cover_image_path.is_data {
                Allocator::<CString>::deallocate(data.channel__cover_image_path.data);
            }
            Allocator::<CVector<_>>::deallocate(data.channel_token_signed.signature);
        }
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.data_registry);
    }
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Channel_GetOneById_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel_token_signed: ChannelTokenSigned,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_one_by_id__serialize_allocate(incoming: Channel_GetOneById_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ Channel_GetOneById_Incoming| -> Result<Channel_GetOneById_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            Channel_GetOneById_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel_token_signed: ChannelTokenSigned_ {
                    channel__id: incoming_.channel_token_signed.channel__id,
                    channel_token__obfuscation_value: incoming_.channel_token_signed.channel_token__obfuscation_value,
                    channel_token__expires_at: incoming_.channel_token_signed.channel_token__expires_at,
                    channel_token__is_user_subscribed: incoming_.channel_token_signed.channel_token__is_user_subscribed,
                    channel_token__is_user_the_owner: incoming_.channel_token_signed.channel_token__is_user_the_owner,
                    signature: incoming_.channel_token_signed.signature.clone_as_vec()?,
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
    pub channel__access_modifier: c_uchar,
    pub channel__visability_modifier: c_uchar,
    pub channel__cover_image_path: COption<CString>,
    pub channel__background_image_path: COption<CString>,
    pub channel__subscribers_quantity: c_uint,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_GetOneById_Precedent {
    pub user_access_token___already_expired: bool,
    pub channel___not_found: bool,
    pub channel___is_close: bool,
    pub channel_token___already_expired: bool,
    pub channel_token___user_is_not_owner: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__get_one_by_id__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> Channel_GetOneById_CResult {
    let converter = move |unified_report: UnifiedReport<Channel_GetOneById_Outcoming_, Channel_GetOneById_Precedent_>| -> Result<CUnifiedReport<Channel_GetOneById_Outcoming, Channel_GetOneById_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let c_data = match data {
                    Data::Empty => {
                        CData::empty()
                    }
                    Data::Filled {
                        data: data_
                    } => {
                        let channel__description = match data_.channel__description {
                            Option::Some(channel__description_) => COption::data(Allocator::<CString>::allocate(channel__description_)),
                            Option::None => COption::none()
                        };
                        let channel__cover_image_path = match data_.channel__cover_image_path {
                            Option::Some(channel__cover_image_path_) => COption::data(Allocator::<CString>::allocate(channel__cover_image_path_)),
                            Option::None => COption::none()
                        };
                        let channel__background_image_path = match data_.channel__background_image_path {
                            Option::Some(channel__background_image_path_) => COption::data(Allocator::<CString>::allocate(channel__background_image_path_)),
                            Option::None => COption::none()
                        };
                        let outcoming = Channel_GetOneById_Outcoming {
                            channel__name: Allocator::<CString>::allocate(data_.channel__name),
                            channel__linked_name: Allocator::<CString>::allocate(data_.channel__linked_name),
                            channel__description,
                            channel__access_modifier: data_.channel__access_modifier,
                            channel__visability_modifier: data_.channel__visability_modifier,
                            channel__cover_image_path,
                            channel__background_image_path,
                            channel__subscribers_quantity: data_.channel__subscribers_quantity,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    Channel_GetOneById_Precedent_::UserAccessToken__AlreadyExpired => Channel_GetOneById_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
                    },
                    Channel_GetOneById_Precedent_::Channel__NotFound => Channel_GetOneById_Precedent {
                        channel___not_found: true,
                        ..Default::default()
                    },
                    Channel_GetOneById_Precedent_::Channel__IsClose => Channel_GetOneById_Precedent {
                        channel___is_close: true,
                        ..Default::default()
                    },
                    Channel_GetOneById_Precedent_::ChannelToken__AlreadyExpired => Channel_GetOneById_Precedent {
                        channel_token___already_expired: true,
                        ..Default::default()
                    },
                    Channel_GetOneById_Precedent_::ChannelToken__UserIsNotOwner => Channel_GetOneById_Precedent {
                        channel_token___user_is_not_owner: true,
                        ..Default::default()
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
    }
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Channel_CheckNameForExisting_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel__name: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__check_name_for_existing__serialize_allocate(incoming: Channel_CheckNameForExisting_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ Channel_CheckNameForExisting_Incoming| -> Result<Channel_CheckNameForExisting_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            Channel_CheckNameForExisting_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel__name: incoming_.channel__name.get_as_str()?,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__check_name_for_existing__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type Channel_CheckNameForExisting_CResult = CResult<CUnifiedReport<Channel_CheckNameForExisting_Outcoming, Channel_CheckNameForExisting_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct Channel_CheckNameForExisting_Outcoming {
    pub result: bool,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_CheckNameForExisting_Precedent {
    pub user_access_token___already_expired: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__check_name_for_existing__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> Channel_CheckNameForExisting_CResult {
    let converter = move |unified_report: UnifiedReport<Channel_CheckNameForExisting_Outcoming_, Channel_CheckNameForExisting_Precedent_>| -> Result<CUnifiedReport<Channel_CheckNameForExisting_Outcoming, Channel_CheckNameForExisting_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let c_data = match data {
                    Data::Empty => {
                        CData::empty()
                    }
                    Data::Filled {
                        data: data_
                    } => {
                        let outcoming = Channel_CheckNameForExisting_Outcoming {
                            result: data_.result,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    Channel_CheckNameForExisting_Precedent_::UserAccessToken__AlreadyExpired => Channel_CheckNameForExisting_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
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
pub extern "C-unwind" fn channel__check_name_for_existing__deserialize_deallocate(_c_result: Channel_CheckNameForExisting_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Channel_CheckLinkedNameForExisting_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel__linked_name: CString,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__check_linked_name_for_existing__serialize_allocate(incoming: Channel_CheckLinkedNameForExisting_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ Channel_CheckLinkedNameForExisting_Incoming| -> Result<Channel_CheckLinkedNameForExisting_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            Channel_CheckLinkedNameForExisting_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel__linked_name: incoming_.channel__linked_name.get_as_str()?,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__check_linked_name_for_existing__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type Channel_CheckLinkedNameForExisting_CResult = CResult<CUnifiedReport<Channel_CheckLinkedNameForExisting_Outcoming, Channel_CheckLinkedNameForExisting_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct Channel_CheckLinkedNameForExisting_Outcoming {
    pub result: bool,
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_CheckLinkedNameForExisting_Precedent {
    pub user_access_token___already_expired: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__check_linked_name_for_existing__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> Channel_CheckLinkedNameForExisting_CResult {
    let converter = move |unified_report: UnifiedReport<Channel_CheckLinkedNameForExisting_Outcoming_, Channel_CheckLinkedNameForExisting_Precedent_>| -> Result<CUnifiedReport<Channel_CheckLinkedNameForExisting_Outcoming, Channel_CheckLinkedNameForExisting_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let c_data = match data {
                    Data::Empty => {
                        CData::empty()
                    }
                    Data::Filled {
                        data: data_
                    } => {
                        let outcoming = Channel_CheckLinkedNameForExisting_Outcoming {
                            result: data_.result,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    Channel_CheckLinkedNameForExisting_Precedent_::UserAccessToken__AlreadyExpired => Channel_CheckLinkedNameForExisting_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
                    },
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
pub extern "C-unwind" fn channel__check_linked_name_for_existing__deserialize_deallocate(_c_result: Channel_CheckLinkedNameForExisting_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Channel_Create_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel__name: CString,
    pub channel__linked_name: CString,
    pub channel__access_modifier: c_uchar,
    pub channel__visability_modifier: c_uchar,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__create__serialize_allocate(incoming: Channel_Create_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ Channel_Create_Incoming| -> Result<Channel_Create_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            Channel_Create_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel__name: incoming_.channel__name.get_as_str()?,
                channel__linked_name: incoming_.channel__linked_name.get_as_str()?,
                channel__access_modifier: incoming_.channel__access_modifier,
                channel__visability_modifier: incoming_.channel__visability_modifier,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__create__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type Channel_Create_CResult = CResult<CUnifiedReport<Channel_Create_Outcoming, Channel_Create_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct Channel_Create_Outcoming {
    pub channel_token_signed: ChannelTokenSigned
}
#[repr(C)]
#[derive(Default)]
pub struct Channel_Create_Precedent {
    pub user_access_token___already_expired: bool,
    pub channel___name_already_exist: bool,
    pub channel___linked_name_already_exist: bool,
    pub parallel_execution: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel__create__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> Channel_Create_CResult {
    let converter = move |unified_report: UnifiedReport<Channel_Create_Outcoming_, Channel_Create_Precedent_>| -> Result<CUnifiedReport<Channel_Create_Outcoming, Channel_Create_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data } => {
                let c_data = match data {
                    Data::Empty => {
                        CData::empty()
                    }
                    Data::Filled {
                        data: data_
                    } => {
                        let outcoming = Channel_Create_Outcoming {
                            channel_token_signed: ChannelTokenSigned {
                                channel__id: data_.channel_token_signed.channel__id,
                                channel_token__obfuscation_value: data_.channel_token_signed.channel_token__obfuscation_value,
                                channel_token__expires_at: data_.channel_token_signed.channel_token__expires_at,
                                channel_token__is_user_subscribed: data_.channel_token_signed.channel_token__is_user_subscribed,
                                channel_token__is_user_the_owner: data_.channel_token_signed.channel_token__is_user_the_owner,
                                signature: Allocator::<CVector<_>>::allocate(data_.channel_token_signed.signature),
                            },
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    Channel_Create_Precedent_::UserAccessToken__AlreadyExpired => Channel_Create_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
                    },
                    Channel_Create_Precedent_::Channel__NameAlreadyExist => Channel_Create_Precedent {
                        channel___name_already_exist: true,
                        ..Default::default()
                    },
                    Channel_Create_Precedent_::Channel__LinkedNameAlreadyExist => Channel_Create_Precedent {
                        channel___linked_name_already_exist: true,
                        ..Default::default()
                    },
                    Channel_Create_Precedent_::ParallelExecution => Channel_Create_Precedent {
                        parallel_execution: true,
                        ..Default::default()
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
pub extern "C-unwind" fn channel__create__deserialize_deallocate(c_result: Channel_Create_CResult) -> () {
    if c_result.is_data && c_result.data.is_target && c_result.data.target.is_filled {
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.channel_token_signed.signature);
    }
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelSubscription_Create_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel_token_signed: ChannelTokenSigned,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_subscription__create__serialize_allocate(incoming: ChannelSubscription_Create_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ ChannelSubscription_Create_Incoming| -> Result<ChannelSubscription_Create_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            ChannelSubscription_Create_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel_token_signed: ChannelTokenSigned_ {
                    channel__id: incoming_.channel_token_signed.channel__id,
                    channel_token__obfuscation_value: incoming_.channel_token_signed.channel_token__obfuscation_value,
                    channel_token__expires_at: incoming_.channel_token_signed.channel_token__expires_at,
                    channel_token__is_user_subscribed: incoming_.channel_token_signed.channel_token__is_user_subscribed,
                    channel_token__is_user_the_owner: incoming_.channel_token_signed.channel_token__is_user_the_owner,
                    signature: incoming_.channel_token_signed.signature.clone_as_vec()?,
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
    pub user_access_token___already_expired: bool,
    pub channel_token___already_expired: bool,
    pub channel___not_found: bool,
    pub channel___user_is_owner: bool,
    pub channel_subscription___already_exist: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_subscription__create__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> ChannelSubscription_Create_CResult {
    let converter = move |unified_report: UnifiedReport<Void, ChannelSubscription_Create_Precedent_>| -> Result<CUnifiedReport<CVoid, ChannelSubscription_Create_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ChannelSubscription_Create_Precedent_::UserAccessToken__AlreadyExpired => ChannelSubscription_Create_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelSubscription_Create_Precedent_::ChannelToken__AlreadyExpired => ChannelSubscription_Create_Precedent {
                        channel_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelSubscription_Create_Precedent_::Channel__NotFound => ChannelSubscription_Create_Precedent {
                        channel___not_found: true,
                        ..Default::default()
                    },
                    ChannelSubscription_Create_Precedent_::Channel__UserIsOwner => ChannelSubscription_Create_Precedent {
                        channel___user_is_owner: true,
                        ..Default::default()
                    },
                    ChannelSubscription_Create_Precedent_::ChannelSubscription__AlreadyExist => ChannelSubscription_Create_Precedent {
                        channel_subscription___already_exist: true,
                        ..Default::default()
                    },
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
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelSubscription_Delete_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel_token_signed: ChannelTokenSigned,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_subscription__delete__serialize_allocate(incoming: ChannelSubscription_Delete_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ ChannelSubscription_Delete_Incoming| -> Result<ChannelSubscription_Delete_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            ChannelSubscription_Delete_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel_token_signed: ChannelTokenSigned_ {
                    channel__id: incoming_.channel_token_signed.channel__id,
                    channel_token__obfuscation_value: incoming_.channel_token_signed.channel_token__obfuscation_value,
                    channel_token__expires_at: incoming_.channel_token_signed.channel_token__expires_at,
                    channel_token__is_user_subscribed: incoming_.channel_token_signed.channel_token__is_user_subscribed,
                    channel_token__is_user_the_owner: incoming_.channel_token_signed.channel_token__is_user_the_owner,
                    signature: incoming_.channel_token_signed.signature.clone_as_vec()?,
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
pub extern "C-unwind" fn channel_subscription__delete__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ChannelSubscription_Delete_CResult = CResult<CUnifiedReport<CVoid, ChannelSubscription_Delete_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ChannelSubscription_Delete_Precedent {
    pub user_access_token___already_expired: bool,
    pub channel_token___already_expired: bool,
    pub channel___user_is_owner: bool,
    pub channel_subscription___not_found: bool,
    pub channel___not_found: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_subscription__delete__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> ChannelSubscription_Delete_CResult {
    let converter = move |unified_report: UnifiedReport<Void, ChannelSubscription_Delete_Precedent_>| -> Result<CUnifiedReport<CVoid, ChannelSubscription_Delete_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ChannelSubscription_Delete_Precedent_::UserAccessToken__AlreadyExpired => ChannelSubscription_Delete_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelSubscription_Delete_Precedent_::ChannelToken__AlreadyExpired => ChannelSubscription_Delete_Precedent {
                        channel_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelSubscription_Delete_Precedent_::Channel__UserIsOwner => ChannelSubscription_Delete_Precedent {
                        channel___user_is_owner: true,
                        ..Default::default()
                    },
                    ChannelSubscription_Delete_Precedent_::ChannelSubscription__NotFound => ChannelSubscription_Delete_Precedent {
                        channel_subscription___not_found: true,
                        ..Default::default()
                    },
                    ChannelSubscription_Delete_Precedent_::Channel__NotFound => ChannelSubscription_Delete_Precedent {
                        channel___not_found: true,
                        ..Default::default()
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
pub extern "C-unwind" fn channel_subscription__delete__deserialize_deallocate(_c_result: ChannelSubscription_Delete_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelPublication1_GetMany_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel_token_signed: ChannelTokenSigned,
    pub channel_publication1__created_at: c_long,
    pub limit: c_uchar,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1__get_many__serialize_allocate(incoming: ChannelPublication1_GetMany_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ ChannelPublication1_GetMany_Incoming| -> Result<ChannelPublication1_GetMany_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            ChannelPublication1_GetMany_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel_token_signed: ChannelTokenSigned_ {
                    channel__id: incoming_.channel_token_signed.channel__id,
                    channel_token__obfuscation_value: incoming_.channel_token_signed.channel_token__obfuscation_value,
                    channel_token__expires_at: incoming_.channel_token_signed.channel_token__expires_at,
                    channel_token__is_user_subscribed: incoming_.channel_token_signed.channel_token__is_user_subscribed,
                    channel_token__is_user_the_owner: incoming_.channel_token_signed.channel_token__is_user_the_owner,
                    signature: incoming_.channel_token_signed.signature.clone_as_vec()?,
                },
                channel_publication1__created_at: incoming_.channel_publication1__created_at,
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
pub extern "C-unwind" fn channel_publication1__get_many__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ChannelPublication1_GetMany_CResult = CResult<CUnifiedReport<ChannelPublication1_GetMany_Outcoming, ChannelPublication1_GetMany_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ChannelPublication1_GetMany_Data {
    pub channel_publication1__images_pathes: CVector<CString>,
    pub channel_publication1__text: COption<CString>,
    pub channel_publication1__commentaries_quantity: c_uint,
    pub channel_publication1__marks_quantity: c_uint,
    pub channel_publication1__view_quantity: c_uint,
    pub channel_publication1__created_at: c_long,
    pub channel_publication1_mark__created_at: COption<c_long>,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
}
#[repr(C)]
#[derive(Default)]
pub struct ChannelPublication1_GetMany_Outcoming {
    pub data_registry: CVector<ChannelPublication1_GetMany_Data>,
}
#[repr(C)]
#[derive(Default)]
pub struct ChannelPublication1_GetMany_Precedent {
    pub user_access_token___already_expired: bool,
    pub channel_token___already_expired: bool,
    pub channel_token___user_is_not_owner: bool,
    pub channel___not_found: bool,
    pub channel___is_close: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1__get_many__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> ChannelPublication1_GetMany_CResult {
    let converter = move |unified_report: UnifiedReport<ChannelPublication1_GetMany_Outcoming_, ChannelPublication1_GetMany_Precedent_>| -> Result<CUnifiedReport<ChannelPublication1_GetMany_Outcoming, ChannelPublication1_GetMany_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data
            } => {
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let mut data_registry: Vec<ChannelPublication1_GetMany_Data> = Vec::with_capacity(data_.data_registry.len());
                        '_a: for data__ in data_.data_registry {
                            let channel_publication1__text = match data__.channel_publication1__text {
                                Option::Some(channel_publication1__text_) => COption::data(Allocator::<CString>::allocate(channel_publication1__text_)),
                                Option::None => COption::none(),
                            };
                            let mut channel_publication1__images_pathes: Vec<CString> = Vec::with_capacity(data__.channel_publication1__images_pathes.len());
                            '_b: for channel_publication1__image_pathe in data__.channel_publication1__images_pathes {
                                channel_publication1__images_pathes.push(
                                    Allocator::<CString>::allocate(channel_publication1__image_pathe),
                                );
                            }
                            let channel_publication1_mark__created_at = match data__.channel_publication1_mark__created_at {
                                Option::Some(channel_publication1_mark__created_at_) => COption::data(channel_publication1_mark__created_at_),
                                Option::None => COption::none(),
                            };
                            data_registry.push(
                                ChannelPublication1_GetMany_Data {
                                    channel_publication1__images_pathes: Allocator::<CVector<_>>::allocate(channel_publication1__images_pathes),
                                    channel_publication1__text,
                                    channel_publication1__commentaries_quantity: data__.channel_publication1__commentaries_quantity,
                                    channel_publication1__marks_quantity: data__.channel_publication1__marks_quantity,
                                    channel_publication1__view_quantity: data__.channel_publication1__view_quantity,
                                    channel_publication1__created_at: data__.channel_publication1__created_at,
                                    channel_publication1_mark__created_at,
                                    channel_publication1_token_signed: ChannelPublication1TokenSigned {
                                        channel__id: data__.channel_publication1_token_signed.channel__id,
                                        channel_publication1__id: data__.channel_publication1_token_signed.channel_publication1__id,
                                        channel_publication1_token__obfuscation_value: data__.channel_publication1_token_signed.channel_publication1_token__obfuscation_value,
                                        channel_publication1_token__expires_at: data__.channel_publication1_token_signed.channel_publication1_token__expires_at,
                                        signature: Allocator::<CVector<_>>::allocate(data__.channel_publication1_token_signed.signature),
                                    },
                                }
                            );
                        }
                        let outcoming = ChannelPublication1_GetMany_Outcoming {
                            data_registry: Allocator::<CVector<_>>::allocate(data_registry),
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            },
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ChannelPublication1_GetMany_Precedent_::UserAccessToken__AlreadyExpired => ChannelPublication1_GetMany_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1_GetMany_Precedent_::ChannelToken__AlreadyExpired => ChannelPublication1_GetMany_Precedent {
                        channel_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1_GetMany_Precedent_::ChannelToken__UserIsNotOwner => ChannelPublication1_GetMany_Precedent {
                        channel_token___user_is_not_owner: true,
                        ..Default::default()
                    },
                    ChannelPublication1_GetMany_Precedent_::Channel__NotFound => ChannelPublication1_GetMany_Precedent {
                        channel___not_found: true,
                        ..Default::default()
                    },
                    ChannelPublication1_GetMany_Precedent_::Channel__IsClose => ChannelPublication1_GetMany_Precedent {
                        channel___is_close: true,
                        ..Default::default()
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
pub extern "C-unwind" fn channel_publication1__get_many__deserialize_deallocate(c_result: ChannelPublication1_GetMany_CResult) -> () {
    if c_result.is_data && c_result.data.is_target && c_result.data.target.is_filled {
        let data_registry = c_result.data.target.filled.data_registry.as_slice_unchecked();
        '_a: for data in data_registry {
            if data.channel_publication1__text.is_data {
                Allocator::<CString>::deallocate(data.channel_publication1__text.data);
            }
            let channel_publication1__images_pathes = data.channel_publication1__images_pathes.as_slice_unchecked();
            '_b: for channel_publication1__image_pathe in channel_publication1__images_pathes {
                Allocator::<CString>::deallocate(*channel_publication1__image_pathe);
            }
            Allocator::<CVector<_>>::deallocate(data.channel_publication1__images_pathes);
            Allocator::<CVector<_>>::deallocate(data.channel_publication1_token_signed.signature);
        }
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.data_registry);
    }
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelPublication1_Create_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel_token_signed: ChannelTokenSigned,
    pub channel_publication1__images_pathes: CVector<CString>,
    pub channel_publication1__text: COption<CString>,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1__create__serialize_allocate(incoming: ChannelPublication1_Create_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ ChannelPublication1_Create_Incoming| -> Result<ChannelPublication1_Create_Incoming_, Box<dyn StdError + 'static>> {
        let mut channel_publication1__images_pathes: Vec<&'_ str> = Vec::with_capacity(incoming_.channel_publication1__images_pathes.length);
        '_a: for channel_publication1__image_path in incoming_.channel_publication1__images_pathes.as_slice()? {
            channel_publication1__images_pathes.push(channel_publication1__image_path.get_as_str()?);
        }
        let channel_publication1__text = if incoming_.channel_publication1__text.is_data {
            Option::Some(incoming_.channel_publication1__text.data.get_as_str()?)
        } else {
            Option::None
        };
        return Result::Ok(
            ChannelPublication1_Create_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel_token_signed: ChannelTokenSigned_ {
                    channel__id: incoming_.channel_token_signed.channel__id,
                    channel_token__obfuscation_value: incoming_.channel_token_signed.channel_token__obfuscation_value,
                    channel_token__expires_at: incoming_.channel_token_signed.channel_token__expires_at,
                    channel_token__is_user_subscribed: incoming_.channel_token_signed.channel_token__is_user_subscribed,
                    channel_token__is_user_the_owner: incoming_.channel_token_signed.channel_token__is_user_the_owner,
                    signature: incoming_.channel_token_signed.signature.clone_as_vec()?,
                },
                channel_publication1__images_pathes,
                channel_publication1__text,
            },
        );
    };
    return Transformer::<ServerRequestData>::transform(
        incoming,
        converter,
    );
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1__create__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ChannelPublication1_Create_CResult = CResult<CUnifiedReport<ChannelPublication1_Create_Outcoming, ChannelPublication1_Create_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ChannelPublication1_Create_Outcoming {
    pub channel_publication1__created_at: c_long,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
}
#[repr(C)]
#[derive(Default)]
pub struct ChannelPublication1_Create_Precedent {
    pub user_access_token___already_expired: bool,
    pub channel_token___already_expired: bool,
    pub user___is_not_channel_owner: bool,
    pub parallel_execution: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1__create__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> ChannelPublication1_Create_CResult {
    let converter = move |unified_report: UnifiedReport<ChannelPublication1_Create_Outcoming_, ChannelPublication1_Create_Precedent_>| -> Result<CUnifiedReport<ChannelPublication1_Create_Outcoming, ChannelPublication1_Create_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data
            } => {
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let outcoming = ChannelPublication1_Create_Outcoming {
                            channel_publication1__created_at: data_.channel_publication1__created_at,
                            channel_publication1_token_signed: ChannelPublication1TokenSigned {
                                channel__id: data_.channel_publication1_token_signed.channel__id,
                                channel_publication1__id: data_.channel_publication1_token_signed.channel_publication1__id,
                                channel_publication1_token__obfuscation_value: data_.channel_publication1_token_signed.channel_publication1_token__obfuscation_value,
                                channel_publication1_token__expires_at: data_.channel_publication1_token_signed.channel_publication1_token__expires_at,
                                signature: Allocator::<CVector<_>>::allocate(data_.channel_publication1_token_signed.signature),
                            },
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            },
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ChannelPublication1_Create_Precedent_::UserAccessToken__AlreadyExpired => ChannelPublication1_Create_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1_Create_Precedent_::ChannelToken__AlreadyExpired => ChannelPublication1_Create_Precedent {
                        channel_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1_Create_Precedent_::User__IsNotChannelOwner => ChannelPublication1_Create_Precedent {
                        user___is_not_channel_owner: true,
                        ..Default::default()
                    },
                    ChannelPublication1_Create_Precedent_::ParallelExecution => ChannelPublication1_Create_Precedent {
                        parallel_execution: true,
                        ..Default::default()
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
pub extern "C-unwind" fn channel_publication1__create__deserialize_deallocate(c_result: ChannelPublication1_Create_CResult) -> () {
    if c_result.is_data && c_result.data.is_target && c_result.data.target.is_filled {
        Allocator::<CVector<_>>::deallocate(c_result.data.target.filled.channel_publication1_token_signed.signature);
    }
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelPublication1_Delete_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel_token_signed: ChannelTokenSigned,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1__delete__serialize_allocate(incoming: ChannelPublication1_Delete_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ ChannelPublication1_Delete_Incoming| -> Result<ChannelPublication1_Delete_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            ChannelPublication1_Delete_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel_token_signed: ChannelTokenSigned_ {
                    channel__id: incoming_.channel_token_signed.channel__id,
                    channel_token__obfuscation_value: incoming_.channel_token_signed.channel_token__obfuscation_value,
                    channel_token__expires_at: incoming_.channel_token_signed.channel_token__expires_at,
                    channel_token__is_user_subscribed: incoming_.channel_token_signed.channel_token__is_user_subscribed,
                    channel_token__is_user_the_owner: incoming_.channel_token_signed.channel_token__is_user_the_owner,
                    signature: incoming_.channel_token_signed.signature.clone_as_vec()?,
                },
                channel_publication1_token_signed: ChannelPublication1TokenSigned_ {
                    channel__id: incoming_.channel_publication1_token_signed.channel__id,
                    channel_publication1__id: incoming_.channel_publication1_token_signed.channel_publication1__id,
                    channel_publication1_token__obfuscation_value: incoming_.channel_publication1_token_signed.channel_publication1_token__obfuscation_value,
                    channel_publication1_token__expires_at: incoming_.channel_publication1_token_signed.channel_publication1_token__expires_at,
                    signature: incoming_.channel_publication1_token_signed.signature.clone_as_vec()?,
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
pub extern "C-unwind" fn channel_publication1__delete__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ChannelPublication1_Delete_CResult = CResult<CUnifiedReport<CVoid, ChannelPublication1_Delete_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ChannelPublication1_Delete_Precedent {
    pub user_access_token___already_expired: bool,
    pub channel_token___already_expired: bool,
    pub channel_publication1_token___already_expired: bool,
    pub user___is_not_channel_owner: bool,
    pub channel_publication1___not_found: bool,
    pub parallel_execution: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1__delete__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> ChannelPublication1_Delete_CResult {
    let converter = move |unified_report: UnifiedReport<Void, ChannelPublication1_Delete_Precedent_>| -> Result<CUnifiedReport<CVoid, ChannelPublication1_Delete_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ChannelPublication1_Delete_Precedent_::UserAccessToken__AlreadyExpired => ChannelPublication1_Delete_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1_Delete_Precedent_::ChannelToken__AlreadyExpired => ChannelPublication1_Delete_Precedent {
                        channel_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1_Delete_Precedent_::ChannelPublication1Token__AlreadyExpired => ChannelPublication1_Delete_Precedent {
                        channel_publication1_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1_Delete_Precedent_::User__IsNotChannelOwner => ChannelPublication1_Delete_Precedent {
                        user___is_not_channel_owner: true,
                        ..Default::default()
                    },
                    ChannelPublication1_Delete_Precedent_::ChannelPublication1__NotFound => ChannelPublication1_Delete_Precedent {
                        channel_publication1___not_found: true,
                        ..Default::default()
                    },
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
pub extern "C-unwind" fn channel_publication1__delete__deserialize_deallocate(_c_result: ChannelPublication1_Delete_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelPublication1Mark_Create_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1_mark__create__serialize_allocate(incoming: ChannelPublication1Mark_Create_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ ChannelPublication1Mark_Create_Incoming| -> Result<ChannelPublication1Mark_Create_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            ChannelPublication1Mark_Create_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel_publication1_token_signed: ChannelPublication1TokenSigned_ {
                    channel__id: incoming_.channel_publication1_token_signed.channel__id,
                    channel_publication1__id: incoming_.channel_publication1_token_signed.channel_publication1__id,
                    channel_publication1_token__obfuscation_value: incoming_.channel_publication1_token_signed.channel_publication1_token__obfuscation_value,
                    channel_publication1_token__expires_at: incoming_.channel_publication1_token_signed.channel_publication1_token__expires_at,
                    signature: incoming_.channel_publication1_token_signed.signature.clone_as_vec()?,
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
pub extern "C-unwind" fn channel_publication1_mark__create__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ChannelPublication1Mark_Create_CResult = CResult<CUnifiedReport<CVoid, ChannelPublication1Mark_Create_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ChannelPublication1Mark_Create_Precedent {
    pub user_access_token___already_expired: bool,
    pub channel_publication1_token___already_expired: bool,
    pub channel_publication1_mark___already_exist: bool,
    pub channel_publication1__not_found: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1_mark__create__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> ChannelPublication1Mark_Create_CResult {
    let converter = move |unified_report: UnifiedReport<Void, ChannelPublication1Mark_Create_Precedent_>| -> Result<CUnifiedReport<CVoid, ChannelPublication1Mark_Create_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ChannelPublication1Mark_Create_Precedent_::UserAccessToken__AlreadyExpired => ChannelPublication1Mark_Create_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1Mark_Create_Precedent_::ChannelPublication1Token__AlreadyExpired => ChannelPublication1Mark_Create_Precedent {
                        channel_publication1_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1Mark_Create_Precedent_::ChannelPublication1Mark__AlreadyExist => ChannelPublication1Mark_Create_Precedent {
                        channel_publication1_mark___already_exist: true,
                        ..Default::default()
                    },
                    ChannelPublication1Mark_Create_Precedent_::ChannelPublication1__NotFound => ChannelPublication1Mark_Create_Precedent {
                        channel_publication1__not_found: true,
                        ..Default::default()
                    },
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
pub extern "C-unwind" fn channel_publication1_mark__create__deserialize_deallocate(_c_result: ChannelPublication1Mark_Create_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelPublication1Mark_Delete_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1_mark__delete__serialize_allocate(incoming: ChannelPublication1Mark_Delete_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ ChannelPublication1Mark_Delete_Incoming| -> Result<ChannelPublication1Mark_Delete_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            ChannelPublication1Mark_Delete_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel_publication1_token_signed: ChannelPublication1TokenSigned_ {
                    channel__id: incoming_.channel_publication1_token_signed.channel__id,
                    channel_publication1__id: incoming_.channel_publication1_token_signed.channel_publication1__id,
                    channel_publication1_token__obfuscation_value: incoming_.channel_publication1_token_signed.channel_publication1_token__obfuscation_value,
                    channel_publication1_token__expires_at: incoming_.channel_publication1_token_signed.channel_publication1_token__expires_at,
                    signature: incoming_.channel_publication1_token_signed.signature.clone_as_vec()?,
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
pub extern "C-unwind" fn channel_publication1_mark__delete__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ChannelPublication1Mark_Delete_CResult = CResult<CUnifiedReport<CVoid, ChannelPublication1Mark_Delete_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ChannelPublication1Mark_Delete_Precedent {
    pub user_access_token___already_expired: bool,
    pub channel_publication1_token___already_expired: bool,
    pub channel_publication1_mark___not_found: bool,
    pub channel_publication1__not_found: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1_mark__delete__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> ChannelPublication1Mark_Delete_CResult {
    let converter = move |unified_report: UnifiedReport<Void, ChannelPublication1Mark_Delete_Precedent_>| -> Result<CUnifiedReport<CVoid, ChannelPublication1Mark_Delete_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ChannelPublication1Mark_Delete_Precedent_::UserAccessToken__AlreadyExpired => ChannelPublication1Mark_Delete_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1Mark_Delete_Precedent_::ChannelPublication1Token__AlreadyExpired => ChannelPublication1Mark_Delete_Precedent {
                        channel_publication1_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1Mark_Delete_Precedent_::ChannelPublication1Mark__NotFound => ChannelPublication1Mark_Delete_Precedent {
                        channel_publication1_mark___not_found: true,
                        ..Default::default()
                    },
                    ChannelPublication1Mark_Delete_Precedent_::ChannelPublication1__NotFound => ChannelPublication1Mark_Delete_Precedent {
                        channel_publication1__not_found: true,
                        ..Default::default()
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
pub extern "C-unwind" fn channel_publication1_mark__delete__deserialize_deallocate(_c_result: ChannelPublication1Mark_Delete_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelPublication1View_Create_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1_view__create__serialize_allocate(incoming: ChannelPublication1View_Create_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ ChannelPublication1View_Create_Incoming| -> Result<ChannelPublication1View_Create_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            ChannelPublication1View_Create_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel_publication1_token_signed: ChannelPublication1TokenSigned_ {
                    channel__id: incoming_.channel_publication1_token_signed.channel__id,
                    channel_publication1__id: incoming_.channel_publication1_token_signed.channel_publication1__id,
                    channel_publication1_token__obfuscation_value: incoming_.channel_publication1_token_signed.channel_publication1_token__obfuscation_value,
                    channel_publication1_token__expires_at: incoming_.channel_publication1_token_signed.channel_publication1_token__expires_at,
                    signature: incoming_.channel_publication1_token_signed.signature.clone_as_vec()?,
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
pub extern "C-unwind" fn channel_publication1_view__create__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ChannelPublication1View_Create_CResult = CResult<CUnifiedReport<CVoid, ChannelPublication1View_Create_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ChannelPublication1View_Create_Precedent {
    pub user_access_token___already_expired: bool,
    pub channel_publication1_token___already_expired: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1_view__create__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> ChannelPublication1View_Create_CResult {
    let converter = move |unified_report: UnifiedReport<Void, ChannelPublication1View_Create_Precedent_>| -> Result<CUnifiedReport<CVoid, ChannelPublication1View_Create_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ChannelPublication1View_Create_Precedent_::UserAccessToken__AlreadyExpired => ChannelPublication1View_Create_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1View_Create_Precedent_::ChannelPublication1Token__AlreadyExpired => ChannelPublication1View_Create_Precedent {
                        channel_publication1_token___already_expired: true,
                        ..Default::default()
                    },
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
pub extern "C-unwind" fn channel_publication1_view__create__deserialize_deallocate(_c_result: ChannelPublication1View_Create_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelPublication1Commentary_Create_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel_publication1_commentary__text: CString,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1_commentary__create__serialize_allocate(incoming: ChannelPublication1Commentary_Create_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ ChannelPublication1Commentary_Create_Incoming| -> Result<ChannelPublication1Commentary_Create_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            ChannelPublication1Commentary_Create_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel_publication1_commentary__text: incoming_.channel_publication1_commentary__text.get_as_str()?,
                channel_publication1_token_signed: ChannelPublication1TokenSigned_ {
                    channel__id: incoming_.channel_publication1_token_signed.channel__id,
                    channel_publication1__id: incoming_.channel_publication1_token_signed.channel_publication1__id,
                    channel_publication1_token__obfuscation_value: incoming_.channel_publication1_token_signed.channel_publication1_token__obfuscation_value,
                    channel_publication1_token__expires_at: incoming_.channel_publication1_token_signed.channel_publication1_token__expires_at,
                    signature: incoming_.channel_publication1_token_signed.signature.clone_as_vec()?,
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
pub extern "C-unwind" fn channel_publication1_commentary__create__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ChannelPublication1Commentary_Create_CResult = CResult<CUnifiedReport<ChannelPublication1Commentary_Create_Outcoming, ChannelPublication1Commentary_Create_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ChannelPublication1Commentary_Create_Outcoming {
    pub channel_publication1__id: c_long,
    pub channel_publication1_commentary__created_at: c_long,
}
#[repr(C)]
#[derive(Default)]
pub struct ChannelPublication1Commentary_Create_Precedent {
    pub user_access_token___already_expired: bool,
    pub channel_publication1_token___already_expired: bool,
    pub parallel_execution: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1_commentary__create__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> ChannelPublication1Commentary_Create_CResult {
    let converter = move |unified_report: UnifiedReport<ChannelPublication1Commentary_Create_Outcoming_, ChannelPublication1Commentary_Create_Precedent_>| -> Result<
        CUnifiedReport<ChannelPublication1Commentary_Create_Outcoming, ChannelPublication1Commentary_Create_Precedent>,
        Box<dyn StdError + 'static>,
    > {
        let unified_report_ = match unified_report {
            UnifiedReport::Target {
                data,
            } => {
                let c_data = match data {
                    Data::Empty => CData::empty(),
                    Data::Filled {
                        data: data_,
                    } => {
                        let outcoming = ChannelPublication1Commentary_Create_Outcoming {
                            channel_publication1__id: data_.channel_publication1_commentary__id,
                            channel_publication1_commentary__created_at: data_.channel_publication1_commentary__created_at,
                        };
                        CData::filled(outcoming)
                    }
                };
                CUnifiedReport::target(c_data)
            }
            UnifiedReport::Precedent {
                precedent,
            } => {
                let precedent_ = match precedent {
                    ChannelPublication1Commentary_Create_Precedent_::UserAccessToken__AlreadyExpired => {
                        ChannelPublication1Commentary_Create_Precedent {
                            user_access_token___already_expired: true,
                            ..Default::default()
                        }
                    }
                    ChannelPublication1Commentary_Create_Precedent_::ChannelPublication1Token__AlreadyExpired => {
                        ChannelPublication1Commentary_Create_Precedent {
                            channel_publication1_token___already_expired: true,
                            ..Default::default()
                        }
                    }
                    ChannelPublication1Commentary_Create_Precedent_::ParallelExecution => {
                        ChannelPublication1Commentary_Create_Precedent {
                            parallel_execution: true,
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
pub extern "C-unwind" fn channel_publication1_commentary__create__deserialize_deallocate(_c_result: ChannelPublication1Commentary_Create_CResult) -> () {
    return ();
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ChannelPublication1Commentary_Delete_Incoming {
    pub user_access_token_signed: UserAccessTokenSigned,
    pub channel_publication1_commentary__id: i64,
    pub channel_publication1_token_signed: ChannelPublication1TokenSigned,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1_commentary__delete__serialize_allocate(incoming: ChannelPublication1Commentary_Delete_Incoming) -> CResult<CVector<c_uchar>> {
    let converter = move |incoming_: &'_ ChannelPublication1Commentary_Delete_Incoming| -> Result<ChannelPublication1Commentary_Delete_Incoming_, Box<dyn StdError + 'static>> {
        return Result::Ok(
            ChannelPublication1Commentary_Delete_Incoming_ {
                user_access_token_signed: UserAccessTokenSigned_ {
                    user__id: incoming_.user_access_token_signed.user__id,
                    user_device__id: incoming_.user_access_token_signed.user_device__id.get_as_str()?,
                    user_access_token__obfuscation_value: incoming_.user_access_token_signed.user_access_token__obfuscation_value,
                    user_access_token__expires_at: incoming_.user_access_token_signed.user_access_token__expires_at,
                    signature: incoming_.user_access_token_signed.signature.clone_as_vec()?,
                },
                channel_publication1_commentary__id: incoming_.channel_publication1_commentary__id,
                channel_publication1_token_signed: ChannelPublication1TokenSigned_ {
                    channel__id: incoming_.channel_publication1_token_signed.channel__id,
                    channel_publication1__id: incoming_.channel_publication1_token_signed.channel_publication1__id,
                    channel_publication1_token__obfuscation_value: incoming_.channel_publication1_token_signed.channel_publication1_token__obfuscation_value,
                    channel_publication1_token__expires_at: incoming_.channel_publication1_token_signed.channel_publication1_token__expires_at,
                    signature: incoming_.channel_publication1_token_signed.signature.clone_as_vec()?,
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
pub extern "C-unwind" fn channel_publication1_commentary__delete__serialize_deallocate(c_result: CResult<CVector<c_uchar>>) -> () {
    Allocator::<CResult<CVector<c_uchar>>>::deallocate(c_result);
    return ();
}
type ChannelPublication1Commentary_Delete_CResult = CResult<CUnifiedReport<CVoid, ChannelPublication1Commentary_Delete_Precedent>>;
#[repr(C)]
#[derive(Default)]
pub struct ChannelPublication1Commentary_Delete_Precedent {
    pub user_access_token___already_expired: bool,
    pub channel_publication1_token___already_expired: bool,
    pub channel_publication1_commentary___not_found: bool,
}
#[unsafe(no_mangle)]
pub extern "C-unwind" fn channel_publication1_commentary__delete__deserialize_allocate(c_vector_of_bytes: CVector<c_uchar>) -> ChannelPublication1Commentary_Delete_CResult {
    let converter = move |unified_report: UnifiedReport<Void, ChannelPublication1Commentary_Delete_Precedent_>| -> Result<CUnifiedReport<CVoid, ChannelPublication1Commentary_Delete_Precedent>, Box<dyn StdError + 'static>> {
        let unified_report_ = match unified_report {
            UnifiedReport::Target { data: _ } => CUnifiedReport::target(CData::empty()),
            UnifiedReport::Precedent { precedent } => {
                let precedent_ = match precedent {
                    ChannelPublication1Commentary_Delete_Precedent_::UserAccessToken__AlreadyExpired => ChannelPublication1Commentary_Delete_Precedent {
                        user_access_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1Commentary_Delete_Precedent_::ChannelPublication1Token__AlreadyExpired => ChannelPublication1Commentary_Delete_Precedent {
                        channel_publication1_token___already_expired: true,
                        ..Default::default()
                    },
                    ChannelPublication1Commentary_Delete_Precedent_::ChannelPublication1Commentary__NotFound => ChannelPublication1Commentary_Delete_Precedent {
                        channel_publication1_commentary___not_found: true,
                        ..Default::default()
                    },

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
pub extern "C-unwind" fn channel_publication1_commentary__delete__deserialize_deallocate(_c_result: ChannelPublication1Commentary_Delete_CResult) -> () {
    return ();
}
#[cfg(test)]
mod test {
    use {
        super::*,
        stats_alloc::{
            INSTRUMENTED_SYSTEM,
            Region,
            StatsAlloc,
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
        macro_rules! with_name {
            ($function_item:expr) => {
                (
                    $function_item,
                    get_function_name($function_item),
                )
            };
        }
        let tests: Vec<
            Vec<(
                fn() -> Result<(), Box<dyn StdError + 'static>>,
                &'static str,
            )>,
        > = vec![
            vec![
                with_name!(self::deallocation::c_vector_clone),
                with_name!(self::deallocation::c_string_get_as_str),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__authorize_by_first_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__authorize_by_first_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__authorize_by_first_step),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__authorize_by_first_step),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__authorize_by_last_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__authorize_by_last_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__authorize_by_last_step),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__authorize_by_last_step),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__check_email_for_existing),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__check_email_for_existing),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__check_email_for_existing),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__check_email_for_existing),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__check_nickname_for_existing),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__check_nickname_for_existing),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__check_nickname_for_existing),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__check_nickname_for_existing),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__deauthorize_from_all_devices),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__deauthorize_from_all_devices),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__deauthorize_from_all_devices),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__deauthorize_from_all_devices),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__deauthorize_from_one_device),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__deauthorize_from_one_device),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__deauthorize_from_one_device),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__deauthorize_from_one_device),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__refresh_access_token),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__refresh_access_token),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__refresh_access_token),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__refresh_access_token),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__register_by_first_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__register_by_first_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__register_by_first_step),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__register_by_first_step),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__register_by_second_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__register_by_second_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__register_by_second_step),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__register_by_second_step),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__register_by_last_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__register_by_last_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__register_by_last_step),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__register_by_last_step),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__reset_password_by_first_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__reset_password_by_first_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__reset_password_by_first_step),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__reset_password_by_first_step),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__reset_password_by_second_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__reset_password_by_second_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__reset_password_by_second_step),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__reset_password_by_second_step),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__reset_password_by_last_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__reset_password_by_last_step),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__reset_password_by_last_step),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__reset_password_by_last_step),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__send_email_for_register),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__send_email_for_register),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__send_email_for_register),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__send_email_for_register),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__send_email_for_authorize),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__send_email_for_authorize),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__send_email_for_authorize),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__send_email_for_authorize),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::user_authorization__send_email_for_reset_password),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__user_authorization__send_email_for_reset_password),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__user_authorization__send_email_for_reset_password),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__user_authorization__send_email_for_reset_password),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel__get_many_by_name_in_subscriptions),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel__get_many_by_name_in_subscriptions),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel__get_many_by_name_in_subscriptions),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel__get_many_by_name_in_subscriptions),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel__get_many_by_subscription),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel__get_many_by_subscription),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel__get_many_by_subscription),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel__get_many_by_subscription),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel__get_many_public_by_name),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel__get_many_public_by_name),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel__get_many_public_by_name),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel__get_many_public_by_name),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel__get_one_by_id),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel__get_one_by_id),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel__get_one_by_id),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel__get_one_by_id),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel__check_name_for_existing),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel__check_name_for_existing),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel__check_name_for_existing),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel__check_name_for_existing),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel__check_linked_name_for_existing),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel__check_linked_name_for_existing),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel__check_linked_name_for_existing),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel__check_linked_name_for_existing),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel__create),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel__create),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel__create),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel__create),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel_subscription__create),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel_subscription__create),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel_subscription__create),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel_subscription__create),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel_subscription__delete),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel_subscription__delete),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel_subscription__delete),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel_subscription__delete),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel_publication1__get_many),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel_publication1__get_many),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel_publication1__get_many),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel_publication1__get_many),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel_publication1__create),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel_publication1__create),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel_publication1__create),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel_publication1__create),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel_publication1__delete),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel_publication1__delete),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel_publication1__delete),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel_publication1__delete),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel_publication1_mark__create),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel_publication1_mark__create),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel_publication1_mark__create),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel_publication1_mark__create),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel_publication1_mark__delete),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel_publication1_mark__delete),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel_publication1_mark__delete),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel_publication1_mark__delete),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel_publication1_view__create),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel_publication1_view__create),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel_publication1_view__create),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel_publication1_view__create),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel_publication1_commentary__create),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel_publication1_commentary__create),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel_publication1_commentary__create),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel_publication1_commentary__create),
            ],
            vec![
                with_name!(self::deallocation::server_request_data_serialization::channel_publication1_commentary__delete),
                with_name!(self::deallocation::server_response_data_deserialization::target_empty__channel_publication1_commentary__delete),
                with_name!(self::deallocation::server_response_data_deserialization::target_filled__channel_publication1_commentary__delete),
                with_name!(self::deallocation::server_response_data_deserialization::precedent__channel_publication1_commentary__delete),
            ],
        ];
        // https://docs.rs/bitcode/0.6.3/src/bitcode/derive/mod.rs.html#68
        // When the `bitcode::encode` method is first called for a specific type, an additional byte is allocated and
        // is not deallocated until the program process completes. Accordingly, when the `bitcode::encode` method is called
        // again, no additional byte occurs and it becomes possible to expect that the number of allocated bytes will be
        // equal to the number of deallocated bytes.
        '_a: for tests_ in tests.iter() {
            '_b: for test in tests_.iter() {
                let _ = test.0();
            }
        }
        '_a: for tests_ in tests.iter() {
            '_b: for test in tests_.iter() {
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
        pub fn c_string_get_as_str() -> Result<(), Box<dyn StdError + 'static>> {
            let c_string = Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string());
            {
                let _ = c_string.get_as_str()?;
            }
            if c_string.pointer.is_null() {
                return Result::Err(ALLOCATION_ERROR.into());
            }
            Allocator::<CString>::deallocate(c_string);
            return Result::Ok(());
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
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    user_authorization__deauthorize_from_all_devices__serialize_allocate,
                    user_authorization__deauthorize_from_all_devices__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                return Result::Ok(());
            }
            pub fn user_authorization__deauthorize_from_one_device() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_DeauthorizeFromOneDevice_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    user_authorization__deauthorize_from_one_device__serialize_allocate,
                    user_authorization__deauthorize_from_one_device__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                return Result::Ok(());
            }
            pub fn user_authorization__refresh_access_token() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = UserAuthorization_RefreshAccessToken_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
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
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
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
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CString>::deallocate(incoming.channel__name);
                Allocator::<CString>::deallocate(incoming.requery___channel__name.data);
                return Result::Ok(());
            }
            pub fn channel__get_many_by_subscription() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel_GetManyBySubscription_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
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
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                return Result::Ok(());
            }
            pub fn channel__get_many_public_by_name() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel_GetManyPublicByName_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
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
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CString>::deallocate(incoming.channel__name);
                Allocator::<CString>::deallocate(incoming.requery___channel__name.data);
                return Result::Ok(());
            }
            pub fn channel__get_one_by_id() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel_GetOneById_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_token_signed: ChannelTokenSigned {
                        channel__id: 0,
                        channel_token__obfuscation_value: 0,
                        channel_token__expires_at: 0,
                        channel_token__is_user_subscribed: false,
                        channel_token__is_user_the_owner: false,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    channel__get_one_by_id__serialize_allocate,
                    channel__get_one_by_id__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CVector<_>>::deallocate(incoming.channel_token_signed.signature);
                return Result::Ok(());
            }
            pub fn channel__check_name_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel_CheckNameForExisting_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel__name: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    channel__check_name_for_existing__serialize_allocate,
                    channel__check_name_for_existing__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CString>::deallocate(incoming.channel__name);
                return Result::Ok(());
            }
            pub fn channel__check_linked_name_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel_CheckLinkedNameForExisting_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel__linked_name: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                };
                run_by_template(
                    incoming,
                    channel__check_linked_name_for_existing__serialize_allocate,
                    channel__check_linked_name_for_existing__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CString>::deallocate(incoming.channel__linked_name);
                return Result::Ok(());
            }
            pub fn channel__create() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = Channel_Create_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel__name: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    channel__linked_name: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    channel__access_modifier: 0,
                    channel__visability_modifier: 0,
                };
                run_by_template(
                    incoming,
                    channel__create__serialize_allocate,
                    channel__create__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CString>::deallocate(incoming.channel__name);
                Allocator::<CString>::deallocate(incoming.channel__linked_name);
                return Result::Ok(());
            }
            pub fn channel_subscription__create() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ChannelSubscription_Create_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_token_signed: ChannelTokenSigned {
                        channel__id: 0,
                        channel_token__obfuscation_value: 0,
                        channel_token__expires_at: 0,
                        channel_token__is_user_subscribed: false,
                        channel_token__is_user_the_owner: false,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    channel_subscription__create__serialize_allocate,
                    channel_subscription__create__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CVector<_>>::deallocate(incoming.channel_token_signed.signature);
                return Result::Ok(());
            }
            pub fn channel_subscription__delete() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ChannelSubscription_Delete_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_token_signed: ChannelTokenSigned {
                        channel__id: 0,
                        channel_token__obfuscation_value: 0,
                        channel_token__expires_at: 0,
                        channel_token__is_user_subscribed: false,
                        channel_token__is_user_the_owner: false,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    channel_subscription__delete__serialize_allocate,
                    channel_subscription__delete__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CVector<_>>::deallocate(incoming.channel_token_signed.signature);
                return Result::Ok(());
            }
            pub fn channel_publication1__get_many() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ChannelPublication1_GetMany_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_token_signed: ChannelTokenSigned {
                        channel__id: 0,
                        channel_token__obfuscation_value: 0,
                        channel_token__expires_at: 0,
                        channel_token__is_user_subscribed: false,
                        channel_token__is_user_the_owner: false,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_publication1__created_at: 0,
                    limit: 0,
                };
                run_by_template(
                    incoming,
                    channel_publication1__get_many__serialize_allocate,
                    channel_publication1__get_many__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CVector<_>>::deallocate(incoming.channel_token_signed.signature);
                return Result::Ok(());
            }
            pub fn channel_publication1__create() -> Result<(), Box<dyn StdError + 'static>> {
                let c_string_1 = Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string());
                let c_string_2 = Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string());
                let c_string_3 = Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string());
                let incoming = ChannelPublication1_Create_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_token_signed: ChannelTokenSigned {
                        channel__id: 0,
                        channel_token__obfuscation_value: 0,
                        channel_token__expires_at: 0,
                        channel_token__is_user_subscribed: false,
                        channel_token__is_user_the_owner: false,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_publication1__images_pathes: Allocator::<CVector<_>>::allocate(vec![c_string_1, c_string_2, c_string_3]),
                    channel_publication1__text: COption::data(Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string())),
                };
                run_by_template(
                    incoming,
                    channel_publication1__create__serialize_allocate,
                    channel_publication1__create__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(c_string_1);
                Allocator::<CString>::deallocate(c_string_2);
                Allocator::<CString>::deallocate(c_string_3);
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CVector<_>>::deallocate(incoming.channel_token_signed.signature);
                Allocator::<CVector<_>>::deallocate(incoming.channel_publication1__images_pathes);
                Allocator::<CString>::deallocate(incoming.channel_publication1__text.data);
                return Result::Ok(());
            }
            pub fn channel_publication1__delete() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ChannelPublication1_Delete_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_token_signed: ChannelTokenSigned {
                        channel__id: 0,
                        channel_token__obfuscation_value: 0,
                        channel_token__expires_at: 0,
                        channel_token__is_user_subscribed: false,
                        channel_token__is_user_the_owner: false,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_publication1_token_signed: ChannelPublication1TokenSigned {
                        channel__id: 0,
                        channel_publication1__id: 0,
                        channel_publication1_token__obfuscation_value: 0,
                        channel_publication1_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    channel_publication1__delete__serialize_allocate,
                    channel_publication1__delete__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.channel_token_signed.signature);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CVector<_>>::deallocate(incoming.channel_publication1_token_signed.signature);
                return Result::Ok(());
            }
            pub fn channel_publication1_mark__create() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ChannelPublication1Mark_Create_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_publication1_token_signed: ChannelPublication1TokenSigned {
                        channel__id: 0,
                        channel_publication1__id: 0,
                        channel_publication1_token__obfuscation_value: 0,
                        channel_publication1_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    channel_publication1_mark__create__serialize_allocate,
                    channel_publication1_mark__create__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CVector<_>>::deallocate(incoming.channel_publication1_token_signed.signature);
                return Result::Ok(());
            }
            pub fn channel_publication1_mark__delete() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ChannelPublication1Mark_Delete_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_publication1_token_signed: ChannelPublication1TokenSigned {
                        channel__id: 0,
                        channel_publication1__id: 0,
                        channel_publication1_token__obfuscation_value: 0,
                        channel_publication1_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    channel_publication1_mark__delete__serialize_allocate,
                    channel_publication1_mark__delete__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CVector<_>>::deallocate(incoming.channel_publication1_token_signed.signature);
                return Result::Ok(());
            }
            pub fn channel_publication1_view__create() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ChannelPublication1View_Create_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_publication1_token_signed: ChannelPublication1TokenSigned {
                        channel__id: 0,
                        channel_publication1__id: 0,
                        channel_publication1_token__obfuscation_value: 0,
                        channel_publication1_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    channel_publication1_view__create__serialize_allocate,
                    channel_publication1_view__create__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CVector<_>>::deallocate(incoming.channel_publication1_token_signed.signature);
                return Result::Ok(());
            }
            pub fn channel_publication1_commentary__create() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ChannelPublication1Commentary_Create_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_publication1_commentary__text: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                    channel_publication1_token_signed: ChannelPublication1TokenSigned {
                        channel__id: 0,
                        channel_publication1__id: 0,
                        channel_publication1_token__obfuscation_value: 0,
                        channel_publication1_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    channel_publication1_commentary__create__serialize_allocate,
                    channel_publication1_commentary__create__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CString>::deallocate(incoming.channel_publication1_commentary__text);
                Allocator::<CVector<_>>::deallocate(incoming.channel_publication1_token_signed.signature);
                return Result::Ok(());
            }
            pub fn channel_publication1_commentary__delete() -> Result<(), Box<dyn StdError + 'static>> {
                let incoming = ChannelPublication1Commentary_Delete_Incoming {
                    user_access_token_signed: UserAccessTokenSigned {
                        user__id: 0,
                        user_device__id: Allocator::<CString>::allocate(NOT_EMPTY_STRING_LITERAL.to_string()),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                    channel_publication1_commentary__id: 0,
                    channel_publication1_token_signed: ChannelPublication1TokenSigned {
                        channel__id: 0,
                        channel_publication1__id: 0,
                        channel_publication1_token__obfuscation_value: 0,
                        channel_publication1_token__expires_at: 0,
                        signature: Allocator::<CVector<_>>::allocate(NOT_EMPTY_ARRAY_LITERAL.to_vec()),
                    },
                };
                run_by_template(
                    incoming,
                    channel_publication1_commentary__delete__serialize_allocate,
                    channel_publication1_commentary__delete__serialize_deallocate,
                )?;
                Allocator::<CString>::deallocate(incoming.user_access_token_signed.user_device__id);
                Allocator::<CVector<_>>::deallocate(incoming.user_access_token_signed.signature);
                Allocator::<CVector<_>>::deallocate(incoming.channel_publication1_token_signed.signature);
                return Result::Ok(());
            }
        }
        pub mod server_response_data_deserialization {
            use {
                super::*,
                dedicated::{
                    action_processor_incoming_outcoming::action_processor::{
                        channel::{
                            get_many_by_name_in_subscriptions::Data as Channel_GetManyByNameInSubscriptions_Data_,
                            get_many_by_subscription::Data as Channel_GetManyBySubscription_Data_,
                            get_many_public_by_name::Data as Channel_GetManyPublicByName_Data_,
                        },
                        channel_publication1::get_many::Data as ChannelPublication1_GetMany_Data_,
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
            fn _precedent__user_authorization__authorize_by_first_step(precedent: UserAuthorization_AuthorizeByFirstStep_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<UserAuthorization_AuthorizeByFirstStep_Outcoming_, UserAuthorization_AuthorizeByFirstStep_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__authorize_by_first_step__deserialize_allocate,
                    user_authorization__authorize_by_first_step__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__authorize_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_AuthorizeByFirstStep_Precedent_::User__WrongEmailOrNicknameOrPassword {
                    UserAuthorization_AuthorizeByFirstStep_Precedent_::User__WrongEmailOrNicknameOrPassword => {}
                    UserAuthorization_AuthorizeByFirstStep_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<UserAuthorization_AuthorizeByFirstStep_Precedent_> = vec![
                    UserAuthorization_AuthorizeByFirstStep_Precedent_::User__WrongEmailOrNicknameOrPassword,
                    UserAuthorization_AuthorizeByFirstStep_Precedent_::ParallelExecution,
                ];
                '_a: for precedent in precedents {
                    _precedent__user_authorization__authorize_by_first_step(precedent)?;
                }
                return Result::Ok(());
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
                        user__id: 0,
                        user_device__id: NOT_EMPTY_STRING_LITERAL.to_string(),
                        user_access_token__obfuscation_value: 0,
                        user_access_token__expires_at: 0,
                        signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                    },
                    user_access_refresh_token_signed: UserAccessRefreshTokenSigned_ {
                        user_access_refresh_token__expires_at: 0,
                        signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                    },
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
                match UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken__NotFound {
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken__NotFound => {}
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken__AlreadyExpired => {}
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken__WrongValue {
                        user_authorization_token__wrong_enter_tries_quantity: _,
                    } => {}
                    UserAuthorization_AuthorizeByLastStep_Precedent_::User__NotFound => {}
                    UserAuthorization_AuthorizeByLastStep_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<UserAuthorization_AuthorizeByLastStep_Precedent_> = vec![
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken__NotFound,
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken__AlreadyExpired,
                    UserAuthorization_AuthorizeByLastStep_Precedent_::UserAuthorizationToken__WrongValue {
                        user_authorization_token__wrong_enter_tries_quantity: 0,
                    },
                    UserAuthorization_AuthorizeByLastStep_Precedent_::User__NotFound,
                    UserAuthorization_AuthorizeByLastStep_Precedent_::ParallelExecution,
                ];
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
                match UserAuthorization_DeauthorizeFromAllDevices_Precedent_::UserAccessToken__AlreadyExpired {
                    UserAuthorization_DeauthorizeFromAllDevices_Precedent_::UserAccessToken__AlreadyExpired => {}
                }
                let precedents: Vec<UserAuthorization_DeauthorizeFromAllDevices_Precedent_> = vec![
                    UserAuthorization_DeauthorizeFromAllDevices_Precedent_::UserAccessToken__AlreadyExpired,
                ];
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
                match UserAuthorization_DeauthorizeFromOneDevice_Precedent_::UserAccessToken__AlreadyExpired {
                    UserAuthorization_DeauthorizeFromOneDevice_Precedent_::UserAccessToken__AlreadyExpired => {}
                }
                let precedents: Vec<UserAuthorization_DeauthorizeFromOneDevice_Precedent_> = vec![
                    UserAuthorization_DeauthorizeFromOneDevice_Precedent_::UserAccessToken__AlreadyExpired,
                ];
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
                        user__id: 0,
                        user_device__id: NOT_EMPTY_STRING_LITERAL.to_string(),
                        user_access_token__obfuscation_value: 0,
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
                match UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken__NotFound {
                    UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken__NotFound => {}
                    UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken__AlreadyExpired => {}
                    UserAuthorization_RefreshAccessToken_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<UserAuthorization_RefreshAccessToken_Precedent_> = vec![
                    UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken__NotFound,
                    UserAuthorization_RefreshAccessToken_Precedent_::UserAccessRefreshToken__AlreadyExpired,
                    UserAuthorization_RefreshAccessToken_Precedent_::ParallelExecution,
                ];
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
            fn _precedent__user_authorization__register_by_first_step(precedent: UserAuthorization_RegisterByFirstStep_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, UserAuthorization_RegisterByFirstStep_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__register_by_first_step__deserialize_allocate,
                    user_authorization__register_by_first_step__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__register_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_RegisterByFirstStep_Precedent_::User__EmailAlreadyExist {
                    UserAuthorization_RegisterByFirstStep_Precedent_::User__EmailAlreadyExist => {}
                    UserAuthorization_RegisterByFirstStep_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<UserAuthorization_RegisterByFirstStep_Precedent_> = vec![
                    UserAuthorization_RegisterByFirstStep_Precedent_::User__EmailAlreadyExist,
                    UserAuthorization_RegisterByFirstStep_Precedent_::ParallelExecution,
                ];
                '_a: for precedent in precedents {
                    _precedent__user_authorization__register_by_first_step(precedent)?;
                }
                return Result::Ok(());
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
                match UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken__NotFound {
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken__NotFound => {}
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken__AlreadyExpired => {}
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken__AlreadyApproved => {}
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken__WrongValue {
                        user_registration_token__wrong_enter_tries_quantity: _,
                    } => {}
                    UserAuthorization_RegisterBySecondStep_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<UserAuthorization_RegisterBySecondStep_Precedent_> = vec![
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken__NotFound,
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken__AlreadyExpired,
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken__AlreadyApproved,
                    UserAuthorization_RegisterBySecondStep_Precedent_::UserRegistrationToken__WrongValue {
                        user_registration_token__wrong_enter_tries_quantity: 0,
                    },
                    UserAuthorization_RegisterBySecondStep_Precedent_::ParallelExecution,
                ];
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
                        user__id: 0,
                        user_device__id: NOT_EMPTY_STRING_LITERAL.to_string(),
                        user_access_token__obfuscation_value: 0,
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
                match UserAuthorization_RegisterByLastStep_Precedent_::User__NicknameAlreadyExist {
                    UserAuthorization_RegisterByLastStep_Precedent_::User__NicknameAlreadyExist => {}
                    UserAuthorization_RegisterByLastStep_Precedent_::User__EmailAlreadyExist => {}
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken__NotFound => {}
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken__AlreadyExpired => {}
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken__IsNotApproved => {}
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken__WrongValue => {}
                    UserAuthorization_RegisterByLastStep_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<UserAuthorization_RegisterByLastStep_Precedent_> = vec![
                    UserAuthorization_RegisterByLastStep_Precedent_::User__NicknameAlreadyExist,
                    UserAuthorization_RegisterByLastStep_Precedent_::User__EmailAlreadyExist,
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken__NotFound,
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken__AlreadyExpired,
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken__IsNotApproved,
                    UserAuthorization_RegisterByLastStep_Precedent_::UserRegistrationToken__WrongValue,
                    UserAuthorization_RegisterByLastStep_Precedent_::ParallelExecution,
                ];
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
            fn _precedent__user_authorization__reset_password_by_first_step(
                precedent: UserAuthorization_ResetPasswordByFirstStep_Precedent_,
            ) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, UserAuthorization_ResetPasswordByFirstStep_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    user_authorization__reset_password_by_first_step__deserialize_allocate,
                    user_authorization__reset_password_by_first_step__deserialize_deallocate,
                );
            }
            pub fn precedent__user_authorization__reset_password_by_first_step() -> Result<(), Box<dyn StdError + 'static>> {
                match UserAuthorization_ResetPasswordByFirstStep_Precedent_::User__NotFound {
                    UserAuthorization_ResetPasswordByFirstStep_Precedent_::User__NotFound => {}
                    UserAuthorization_ResetPasswordByFirstStep_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<UserAuthorization_ResetPasswordByFirstStep_Precedent_> = vec![
                    UserAuthorization_ResetPasswordByFirstStep_Precedent_::User__NotFound,
                    UserAuthorization_ResetPasswordByFirstStep_Precedent_::ParallelExecution,
                ];
                '_a: for precedent in precedents {
                    _precedent__user_authorization__reset_password_by_first_step(precedent)?;
                }
                return Result::Ok(());
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
                match UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken__NotFound {
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken__NotFound => {}
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken__AlreadyExpired => {}
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken__AlreadyApproved => {}
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken__WrongValue {
                        user_reset_password_token__wrong_enter_tries_quantity: _,
                    } => {}
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<UserAuthorization_ResetPasswordBySecondStep_Precedent_> = vec![
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken__NotFound,
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken__AlreadyExpired,
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken__AlreadyApproved,
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::UserResetPasswordToken__WrongValue {
                        user_reset_password_token__wrong_enter_tries_quantity: 0,
                    },
                    UserAuthorization_ResetPasswordBySecondStep_Precedent_::ParallelExecution,
                ];
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
                match UserAuthorization_ResetPasswordByLastStep_Precedent_::User__NotFound {
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::User__NotFound => {}
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken__NotFound => {}
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken__AlreadyExpired => {}
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken__IsNotApproved => {}
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken__WrongValue => {}
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<UserAuthorization_ResetPasswordByLastStep_Precedent_> = vec![
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::User__NotFound,
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken__NotFound,
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken__AlreadyExpired,
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken__IsNotApproved,
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::UserResetPasswordToken__WrongValue,
                    UserAuthorization_ResetPasswordByLastStep_Precedent_::ParallelExecution,
                ];
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
                match UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken__NotFound {
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken__NotFound => {}
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken__AlreadyExpired => {}
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken__AlreadyApproved => {}
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken__TimeToResendHasNotCome => {}
                    UserAuthorization_SendEmailForRegister_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<UserAuthorization_SendEmailForRegister_Precedent_> = vec![
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken__NotFound,
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken__AlreadyExpired,
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken__AlreadyApproved,
                    UserAuthorization_SendEmailForRegister_Precedent_::UserRegistrationToken__TimeToResendHasNotCome,
                    UserAuthorization_SendEmailForRegister_Precedent_::ParallelExecution,
                ];
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
                match UserAuthorization_SendEmailForAuthorize_Precedent_::User__NotFound {
                    UserAuthorization_SendEmailForAuthorize_Precedent_::User__NotFound => {}
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken__NotFound => {}
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken__AlreadyExpired => {}
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken__TimeToResendHasNotCome => {}
                    UserAuthorization_SendEmailForAuthorize_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<UserAuthorization_SendEmailForAuthorize_Precedent_> = vec![
                    UserAuthorization_SendEmailForAuthorize_Precedent_::User__NotFound,
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken__NotFound,
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken__AlreadyExpired,
                    UserAuthorization_SendEmailForAuthorize_Precedent_::UserAuthorizationToken__TimeToResendHasNotCome,
                    UserAuthorization_SendEmailForAuthorize_Precedent_::ParallelExecution,
                ];
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
                match UserAuthorization_SendEmailForResetPassword_Precedent_::User__NotFound {
                    UserAuthorization_SendEmailForResetPassword_Precedent_::User__NotFound => {}
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken__NotFound => {}
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken__AlreadyExpired => {}
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken__AlreadyApproved => {}
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken__TimeToResendHasNotCome => {}
                    UserAuthorization_SendEmailForResetPassword_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<UserAuthorization_SendEmailForResetPassword_Precedent_> = vec![
                    UserAuthorization_SendEmailForResetPassword_Precedent_::User__NotFound,
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken__NotFound,
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken__AlreadyExpired,
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken__AlreadyApproved,
                    UserAuthorization_SendEmailForResetPassword_Precedent_::UserResetPasswordToken__TimeToResendHasNotCome,
                    UserAuthorization_SendEmailForResetPassword_Precedent_::ParallelExecution,
                ];
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
                        channel__name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__linked_name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__access_modifier: 0,
                        channel__visability_modifier: 0,
                        channel__background_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel__cover_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel_token_signed: ChannelTokenSigned_ {
                            channel__id: 0,
                            channel_token__obfuscation_value: 0,
                            channel_token__expires_at: 0,
                            channel_token__is_user_subscribed: false,
                            channel_token__is_user_the_owner: false,
                            signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                        },
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
                match Channel_GetManyByNameInSubscriptions_Precedent_::UserAccessToken__AlreadyExpired {
                    Channel_GetManyByNameInSubscriptions_Precedent_::UserAccessToken__AlreadyExpired => {}
                }
                let precedents: Vec<Channel_GetManyByNameInSubscriptions_Precedent_> = vec![
                    Channel_GetManyByNameInSubscriptions_Precedent_::UserAccessToken__AlreadyExpired,
                ];
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
                        channel__name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__linked_name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__access_modifier: 0,
                        channel__visability_modifier: 0,
                        channel__background_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel__cover_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel_token_signed: ChannelTokenSigned_ {
                            channel__id: 0,
                            channel_token__obfuscation_value: 0,
                            channel_token__expires_at: 0,
                            channel_token__is_user_subscribed: false,
                            channel_token__is_user_the_owner: false,
                            signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                        },
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
                match Channel_GetManyBySubscription_Precedent_::UserAccessToken__AlreadyExpired {
                    Channel_GetManyBySubscription_Precedent_::UserAccessToken__AlreadyExpired => {}
                }
                let precedents: Vec<Channel_GetManyBySubscription_Precedent_> = vec![
                    Channel_GetManyBySubscription_Precedent_::UserAccessToken__AlreadyExpired,
                ];
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
                        channel__name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__linked_name: NOT_EMPTY_STRING_LITERAL.to_string(),
                        channel__access_modifier: 0,
                        channel__background_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel__cover_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel_token_signed: ChannelTokenSigned_ {
                            channel__id: 0,
                            channel_token__obfuscation_value: 0,
                            channel_token__expires_at: 0,
                            channel_token__is_user_subscribed: false,
                            channel_token__is_user_the_owner: false,
                            signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                        },
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
                match Channel_GetManyPublicByName_Precedent_::UserAccessToken__AlreadyExpired {
                    Channel_GetManyPublicByName_Precedent_::UserAccessToken__AlreadyExpired => {}
                }
                let precedents: Vec<Channel_GetManyPublicByName_Precedent_> = vec![
                    Channel_GetManyPublicByName_Precedent_::UserAccessToken__AlreadyExpired,
                ];
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
                    channel__background_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                    channel__cover_image_path: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                    channel__subscribers_quantity: 0,
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
                match Channel_GetOneById_Precedent_::UserAccessToken__AlreadyExpired {
                    Channel_GetOneById_Precedent_::UserAccessToken__AlreadyExpired => {}
                    Channel_GetOneById_Precedent_::Channel__NotFound => {}
                    Channel_GetOneById_Precedent_::Channel__IsClose => {}
                    Channel_GetOneById_Precedent_::ChannelToken__AlreadyExpired => {}
                    Channel_GetOneById_Precedent_::ChannelToken__UserIsNotOwner => {}
                }
                let precedents: Vec<Channel_GetOneById_Precedent_> = vec![
                    Channel_GetOneById_Precedent_::UserAccessToken__AlreadyExpired,
                    Channel_GetOneById_Precedent_::Channel__NotFound,
                    Channel_GetOneById_Precedent_::Channel__IsClose,
                    Channel_GetOneById_Precedent_::ChannelToken__AlreadyExpired,
                    Channel_GetOneById_Precedent_::ChannelToken__UserIsNotOwner,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel__get_one_by_id(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel__check_name_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_CheckNameForExisting_Outcoming_, Channel_CheckNameForExisting_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel__check_name_for_existing__deserialize_allocate,
                    channel__check_name_for_existing__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel__check_name_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = Channel_CheckNameForExisting_Outcoming_ {
                    result: false,
                };
                let unified_report = UnifiedReport::<Channel_CheckNameForExisting_Outcoming_, Channel_CheckNameForExisting_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    channel__check_name_for_existing__deserialize_allocate,
                    channel__check_name_for_existing__deserialize_deallocate,
                );
            }
            fn _precedent__channel__check_name_for_existing(precedent: Channel_CheckNameForExisting_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_CheckNameForExisting_Outcoming_, Channel_CheckNameForExisting_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel__check_name_for_existing__deserialize_allocate,
                    channel__check_name_for_existing__deserialize_deallocate,
                );
            }
            pub fn precedent__channel__check_name_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                match Channel_CheckNameForExisting_Precedent_::UserAccessToken__AlreadyExpired {
                    Channel_CheckNameForExisting_Precedent_::UserAccessToken__AlreadyExpired => {}
                }
                let precedents: Vec<Channel_CheckNameForExisting_Precedent_> = vec![
                    Channel_CheckNameForExisting_Precedent_::UserAccessToken__AlreadyExpired,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel__check_name_for_existing(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel__check_linked_name_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_CheckLinkedNameForExisting_Outcoming_, Channel_CheckLinkedNameForExisting_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel__check_linked_name_for_existing__deserialize_allocate,
                    channel__check_linked_name_for_existing__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel__check_linked_name_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = Channel_CheckLinkedNameForExisting_Outcoming_ {
                    result: false,
                };
                let unified_report = UnifiedReport::<Channel_CheckLinkedNameForExisting_Outcoming_, Channel_CheckLinkedNameForExisting_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    channel__check_linked_name_for_existing__deserialize_allocate,
                    channel__check_linked_name_for_existing__deserialize_deallocate,
                );
            }
            fn _precedent__channel__check_linked_name_for_existing(precedent: Channel_CheckLinkedNameForExisting_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_CheckLinkedNameForExisting_Outcoming_, Channel_CheckLinkedNameForExisting_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel__check_linked_name_for_existing__deserialize_allocate,
                    channel__check_linked_name_for_existing__deserialize_deallocate,
                );
            }
            pub fn precedent__channel__check_linked_name_for_existing() -> Result<(), Box<dyn StdError + 'static>> {
                match Channel_CheckLinkedNameForExisting_Precedent_::UserAccessToken__AlreadyExpired {
                    Channel_CheckLinkedNameForExisting_Precedent_::UserAccessToken__AlreadyExpired => {}
                }
                let precedents: Vec<Channel_CheckLinkedNameForExisting_Precedent_> = vec![
                    Channel_CheckLinkedNameForExisting_Precedent_::UserAccessToken__AlreadyExpired,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel__check_linked_name_for_existing(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel__create() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_Create_Outcoming_, Channel_Create_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel__create__deserialize_allocate,
                    channel__create__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel__create() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = Channel_Create_Outcoming_ {
                    channel_token_signed: ChannelTokenSigned_ {
                        channel__id: 0,
                        channel_token__obfuscation_value: 0,
                        channel_token__expires_at: 0,
                        channel_token__is_user_subscribed: false,
                        channel_token__is_user_the_owner: false,
                        signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                    },
                };
                let unified_report = UnifiedReport::<Channel_Create_Outcoming_, Channel_Create_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    channel__create__deserialize_allocate,
                    channel__create__deserialize_deallocate,
                );
            }
            fn _precedent__channel__create(precedent: Channel_Create_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Channel_Create_Outcoming_, Channel_Create_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel__create__deserialize_allocate,
                    channel__create__deserialize_deallocate,
                );
            }
            pub fn precedent__channel__create() -> Result<(), Box<dyn StdError + 'static>> {
                match Channel_Create_Precedent_::UserAccessToken__AlreadyExpired {
                    Channel_Create_Precedent_::UserAccessToken__AlreadyExpired => {}
                    Channel_Create_Precedent_::Channel__NameAlreadyExist => {}
                    Channel_Create_Precedent_::Channel__LinkedNameAlreadyExist => {}
                    Channel_Create_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<Channel_Create_Precedent_> = vec![
                    Channel_Create_Precedent_::UserAccessToken__AlreadyExpired,
                    Channel_Create_Precedent_::Channel__NameAlreadyExist,
                    Channel_Create_Precedent_::Channel__LinkedNameAlreadyExist,
                    Channel_Create_Precedent_::ParallelExecution,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel__create(precedent)?;
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
                match ChannelSubscription_Create_Precedent_::UserAccessToken__AlreadyExpired {
                    ChannelSubscription_Create_Precedent_::UserAccessToken__AlreadyExpired => {}
                    ChannelSubscription_Create_Precedent_::ChannelToken__AlreadyExpired => {}
                    ChannelSubscription_Create_Precedent_::Channel__NotFound => {}
                    ChannelSubscription_Create_Precedent_::Channel__UserIsOwner => {}
                    ChannelSubscription_Create_Precedent_::ChannelSubscription__AlreadyExist => {}
                }
                let precedents: Vec<ChannelSubscription_Create_Precedent_> = vec![
                    ChannelSubscription_Create_Precedent_::UserAccessToken__AlreadyExpired,
                    ChannelSubscription_Create_Precedent_::ChannelToken__AlreadyExpired,
                    ChannelSubscription_Create_Precedent_::Channel__NotFound,
                    ChannelSubscription_Create_Precedent_::Channel__UserIsOwner,
                    ChannelSubscription_Create_Precedent_::ChannelSubscription__AlreadyExist,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel_subscription__create(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel_subscription__delete() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelSubscription_Delete_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel_subscription__delete__deserialize_allocate,
                    channel_subscription__delete__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel_subscription__delete() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            fn _precedent__channel_subscription__delete(precedent: ChannelSubscription_Delete_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelSubscription_Delete_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel_subscription__delete__deserialize_allocate,
                    channel_subscription__delete__deserialize_deallocate,
                );
            }
            pub fn precedent__channel_subscription__delete() -> Result<(), Box<dyn StdError + 'static>> {
                match ChannelSubscription_Delete_Precedent_::UserAccessToken__AlreadyExpired {
                    ChannelSubscription_Delete_Precedent_::UserAccessToken__AlreadyExpired => {}
                    ChannelSubscription_Delete_Precedent_::ChannelToken__AlreadyExpired => {}
                    ChannelSubscription_Delete_Precedent_::Channel__UserIsOwner => {}
                    ChannelSubscription_Delete_Precedent_::ChannelSubscription__NotFound => {}
                    ChannelSubscription_Delete_Precedent_::Channel__NotFound => {}
                }
                let precedents: Vec<ChannelSubscription_Delete_Precedent_> = vec![
                    ChannelSubscription_Delete_Precedent_::UserAccessToken__AlreadyExpired,
                    ChannelSubscription_Delete_Precedent_::ChannelToken__AlreadyExpired,
                    ChannelSubscription_Delete_Precedent_::Channel__UserIsOwner,
                    ChannelSubscription_Delete_Precedent_::ChannelSubscription__NotFound,
                    ChannelSubscription_Delete_Precedent_::Channel__NotFound,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel_subscription__delete(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel_publication1__get_many() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<ChannelPublication1_GetMany_Outcoming_, ChannelPublication1_GetMany_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel_publication1__get_many__deserialize_allocate,
                    channel_publication1__get_many__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel_publication1__get_many() -> Result<(), Box<dyn StdError + 'static>> {
                let mut data_registry: Vec<ChannelPublication1_GetMany_Data_> = vec![];
                '_a: for _ in 1..=5 {
                    let data = ChannelPublication1_GetMany_Data_ {
                        channel_publication1__text: Option::Some(NOT_EMPTY_STRING_LITERAL.to_string()),
                        channel_publication1__images_pathes: vec![
                            NOT_EMPTY_STRING_LITERAL.to_string(),
                            NOT_EMPTY_STRING_LITERAL.to_string(),
                            NOT_EMPTY_STRING_LITERAL.to_string(),
                        ],
                        channel_publication1__commentaries_quantity: 0,
                        channel_publication1__marks_quantity: 0,
                        channel_publication1__view_quantity: 0,
                        channel_publication1__created_at: 0,
                        channel_publication1_mark__created_at: Option::Some(0),
                        channel_publication1_token_signed: ChannelPublication1TokenSigned_ {
                            channel__id: 0,
                            channel_publication1__id: 0,
                            channel_publication1_token__obfuscation_value: 0,
                            channel_publication1_token__expires_at: 0,
                            signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                        },
                    };
                    data_registry.push(data);
                }
                let outcoming = ChannelPublication1_GetMany_Outcoming_ {
                    data_registry,
                };
                let unified_report = UnifiedReport::<ChannelPublication1_GetMany_Outcoming_, ChannelPublication1_GetMany_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    channel_publication1__get_many__deserialize_allocate,
                    channel_publication1__get_many__deserialize_deallocate,
                );
            }
            fn _precedent__channel_publication1__get_many(precedent: ChannelPublication1_GetMany_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<ChannelPublication1_GetMany_Outcoming_, ChannelPublication1_GetMany_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel_publication1__get_many__deserialize_allocate,
                    channel_publication1__get_many__deserialize_deallocate,
                );
            }
            pub fn precedent__channel_publication1__get_many() -> Result<(), Box<dyn StdError + 'static>> {
                match ChannelPublication1_GetMany_Precedent_::UserAccessToken__AlreadyExpired {
                    ChannelPublication1_GetMany_Precedent_::UserAccessToken__AlreadyExpired => {}
                    ChannelPublication1_GetMany_Precedent_::ChannelToken__AlreadyExpired => {}
                    ChannelPublication1_GetMany_Precedent_::ChannelToken__UserIsNotOwner => {}
                    ChannelPublication1_GetMany_Precedent_::Channel__NotFound => {}
                    ChannelPublication1_GetMany_Precedent_::Channel__IsClose => {}
                }
                let precedents: Vec<ChannelPublication1_GetMany_Precedent_> = vec![
                    ChannelPublication1_GetMany_Precedent_::UserAccessToken__AlreadyExpired,
                    ChannelPublication1_GetMany_Precedent_::ChannelToken__AlreadyExpired,
                    ChannelPublication1_GetMany_Precedent_::ChannelToken__UserIsNotOwner,
                    ChannelPublication1_GetMany_Precedent_::Channel__NotFound,
                    ChannelPublication1_GetMany_Precedent_::Channel__IsClose,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel_publication1__get_many(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel_publication1__create() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<ChannelPublication1_Create_Outcoming_, ChannelPublication1_Create_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel_publication1__create__deserialize_allocate,
                    channel_publication1__create__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel_publication1__create() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = ChannelPublication1_Create_Outcoming_ {
                    channel_publication1__created_at: 0,
                    channel_publication1_token_signed: ChannelPublication1TokenSigned_ {
                        channel__id: 0,
                        channel_publication1__id: 0,
                        channel_publication1_token__obfuscation_value: 0,
                        channel_publication1_token__expires_at: 0,
                        signature: NOT_EMPTY_ARRAY_LITERAL.to_vec(),
                    },
                };
                let unified_report = UnifiedReport::<ChannelPublication1_Create_Outcoming_, ChannelPublication1_Create_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    channel_publication1__create__deserialize_allocate,
                    channel_publication1__create__deserialize_deallocate,
                );
            }
            fn _precedent__channel_publication1__create(precedent: ChannelPublication1_Create_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<ChannelPublication1_Create_Outcoming_, ChannelPublication1_Create_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel_publication1__create__deserialize_allocate,
                    channel_publication1__create__deserialize_deallocate,
                );
            }
            pub fn precedent__channel_publication1__create() -> Result<(), Box<dyn StdError + 'static>> {
                match ChannelPublication1_Create_Precedent_::UserAccessToken__AlreadyExpired {
                    ChannelPublication1_Create_Precedent_::UserAccessToken__AlreadyExpired => {}
                    ChannelPublication1_Create_Precedent_::ChannelToken__AlreadyExpired => {}
                    ChannelPublication1_Create_Precedent_::User__IsNotChannelOwner => {}
                    ChannelPublication1_Create_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<ChannelPublication1_Create_Precedent_> = vec![
                    ChannelPublication1_Create_Precedent_::UserAccessToken__AlreadyExpired,
                    ChannelPublication1_Create_Precedent_::ChannelToken__AlreadyExpired,
                    ChannelPublication1_Create_Precedent_::User__IsNotChannelOwner,
                    ChannelPublication1_Create_Precedent_::ParallelExecution,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel_publication1__create(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel_publication1__delete() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelPublication1_Delete_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel_publication1__delete__deserialize_allocate,
                    channel_publication1__delete__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel_publication1__delete() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            fn _precedent__channel_publication1__delete(precedent: ChannelPublication1_Delete_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelPublication1_Delete_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel_publication1__delete__deserialize_allocate,
                    channel_publication1__delete__deserialize_deallocate,
                );
            }
            pub fn precedent__channel_publication1__delete() -> Result<(), Box<dyn StdError + 'static>> {
                match ChannelPublication1_Delete_Precedent_::UserAccessToken__AlreadyExpired {
                    ChannelPublication1_Delete_Precedent_::UserAccessToken__AlreadyExpired => {}
                    ChannelPublication1_Delete_Precedent_::ChannelToken__AlreadyExpired => {}
                    ChannelPublication1_Delete_Precedent_::ChannelPublication1Token__AlreadyExpired => {}
                    ChannelPublication1_Delete_Precedent_::User__IsNotChannelOwner => {}
                    ChannelPublication1_Delete_Precedent_::ChannelPublication1__NotFound => {}
                }
                let precedents: Vec<ChannelPublication1_Delete_Precedent_> = vec![
                    ChannelPublication1_Delete_Precedent_::UserAccessToken__AlreadyExpired,
                    ChannelPublication1_Delete_Precedent_::ChannelToken__AlreadyExpired,
                    ChannelPublication1_Delete_Precedent_::ChannelPublication1Token__AlreadyExpired,
                    ChannelPublication1_Delete_Precedent_::User__IsNotChannelOwner,
                    ChannelPublication1_Delete_Precedent_::ChannelPublication1__NotFound,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel_publication1__delete(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel_publication1_mark__create() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelPublication1Mark_Create_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel_publication1_mark__create__deserialize_allocate,
                    channel_publication1_mark__create__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel_publication1_mark__create() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            fn _precedent__channel_publication1_mark__create(precedent: ChannelPublication1Mark_Create_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelPublication1Mark_Create_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel_publication1_mark__create__deserialize_allocate,
                    channel_publication1_mark__create__deserialize_deallocate,
                );
            }
            pub fn precedent__channel_publication1_mark__create() -> Result<(), Box<dyn StdError + 'static>> {
                match ChannelPublication1Mark_Create_Precedent_::UserAccessToken__AlreadyExpired {
                    ChannelPublication1Mark_Create_Precedent_::UserAccessToken__AlreadyExpired => {}
                    ChannelPublication1Mark_Create_Precedent_::ChannelPublication1Token__AlreadyExpired => {}
                    ChannelPublication1Mark_Create_Precedent_::ChannelPublication1Mark__AlreadyExist => {}
                    ChannelPublication1Mark_Create_Precedent_::ChannelPublication1__NotFound => {}
                }
                let precedents: Vec<ChannelPublication1Mark_Create_Precedent_> = vec![
                    ChannelPublication1Mark_Create_Precedent_::UserAccessToken__AlreadyExpired,
                    ChannelPublication1Mark_Create_Precedent_::ChannelPublication1Token__AlreadyExpired,
                    ChannelPublication1Mark_Create_Precedent_::ChannelPublication1Mark__AlreadyExist,
                    ChannelPublication1Mark_Create_Precedent_::ChannelPublication1__NotFound,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel_publication1_mark__create(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel_publication1_mark__delete() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelPublication1Mark_Delete_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel_publication1_mark__delete__deserialize_allocate,
                    channel_publication1_mark__delete__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel_publication1_mark__delete() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            fn _precedent__channel_publication1_mark__delete(precedent: ChannelPublication1Mark_Delete_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelPublication1Mark_Delete_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel_publication1_mark__delete__deserialize_allocate,
                    channel_publication1_mark__delete__deserialize_deallocate,
                );
            }
            pub fn precedent__channel_publication1_mark__delete() -> Result<(), Box<dyn StdError + 'static>> {
                match ChannelPublication1Mark_Delete_Precedent_::UserAccessToken__AlreadyExpired {
                    ChannelPublication1Mark_Delete_Precedent_::UserAccessToken__AlreadyExpired => {}
                    ChannelPublication1Mark_Delete_Precedent_::ChannelPublication1Token__AlreadyExpired => {}
                    ChannelPublication1Mark_Delete_Precedent_::ChannelPublication1Mark__NotFound => {}
                    ChannelPublication1Mark_Delete_Precedent_::ChannelPublication1__NotFound => {}
                }
                let precedents: Vec<ChannelPublication1Mark_Delete_Precedent_> = vec![
                    ChannelPublication1Mark_Delete_Precedent_::UserAccessToken__AlreadyExpired,
                    ChannelPublication1Mark_Delete_Precedent_::ChannelPublication1Token__AlreadyExpired,
                    ChannelPublication1Mark_Delete_Precedent_::ChannelPublication1Mark__NotFound,
                    ChannelPublication1Mark_Delete_Precedent_::ChannelPublication1__NotFound,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel_publication1_mark__delete(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel_publication1_view__create() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelPublication1View_Create_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel_publication1_view__create__deserialize_allocate,
                    channel_publication1_view__create__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel_publication1_view__create() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            fn _precedent__channel_publication1_view__create(precedent: ChannelPublication1View_Create_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelPublication1View_Create_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel_publication1_view__create__deserialize_allocate,
                    channel_publication1_view__create__deserialize_deallocate,
                );
            }
            pub fn precedent__channel_publication1_view__create() -> Result<(), Box<dyn StdError + 'static>> {
                match ChannelPublication1View_Create_Precedent_::UserAccessToken__AlreadyExpired {
                    ChannelPublication1View_Create_Precedent_::UserAccessToken__AlreadyExpired => {}
                    ChannelPublication1View_Create_Precedent_::ChannelPublication1Token__AlreadyExpired => {}
                }
                let precedents: Vec<ChannelPublication1View_Create_Precedent_> = vec![
                    ChannelPublication1View_Create_Precedent_::UserAccessToken__AlreadyExpired,
                    ChannelPublication1View_Create_Precedent_::ChannelPublication1Token__AlreadyExpired,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel_publication1_view__create(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel_publication1_commentary__create() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<ChannelPublication1Commentary_Create_Outcoming_, ChannelPublication1Commentary_Create_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel_publication1_commentary__create__deserialize_allocate,
                    channel_publication1_commentary__create__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel_publication1_commentary__create() -> Result<(), Box<dyn StdError + 'static>> {
                let outcoming = ChannelPublication1Commentary_Create_Outcoming_ {
                    channel_publication1_commentary__id: 0,
                    channel_publication1_commentary__created_at: 0,
                };
                let unified_report = UnifiedReport::<ChannelPublication1Commentary_Create_Outcoming_, ChannelPublication1Commentary_Create_Precedent_>::target_filled(outcoming);
                return run_by_template(
                    &unified_report,
                    channel_publication1_commentary__create__deserialize_allocate,
                    channel_publication1_commentary__create__deserialize_deallocate,
                );
            }
            fn _precedent__channel_publication1_commentary__create(precedent: ChannelPublication1Commentary_Create_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<ChannelPublication1Commentary_Create_Outcoming_, ChannelPublication1Commentary_Create_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel_publication1_commentary__create__deserialize_allocate,
                    channel_publication1_commentary__create__deserialize_deallocate,
                );
            }
            pub fn precedent__channel_publication1_commentary__create() -> Result<(), Box<dyn StdError + 'static>> {
                match ChannelPublication1Commentary_Create_Precedent_::UserAccessToken__AlreadyExpired {
                    ChannelPublication1Commentary_Create_Precedent_::UserAccessToken__AlreadyExpired => {}
                    ChannelPublication1Commentary_Create_Precedent_::ChannelPublication1Token__AlreadyExpired => {}
                    ChannelPublication1Commentary_Create_Precedent_::ParallelExecution => {}
                }
                let precedents: Vec<ChannelPublication1Commentary_Create_Precedent_> = vec![
                    ChannelPublication1Commentary_Create_Precedent_::UserAccessToken__AlreadyExpired,
                    ChannelPublication1Commentary_Create_Precedent_::ChannelPublication1Token__AlreadyExpired,
                    ChannelPublication1Commentary_Create_Precedent_::ParallelExecution,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel_publication1_commentary__create(precedent)?;
                }
                return Result::Ok(());
            }
            pub fn target_empty__channel_publication1_commentary__delete() -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelPublication1Commentary_Delete_Precedent_>::target_empty();
                return run_by_template(
                    &unified_report,
                    channel_publication1_commentary__delete__deserialize_allocate,
                    channel_publication1_commentary__delete__deserialize_deallocate,
                );
            }
            pub fn target_filled__channel_publication1_commentary__delete() -> Result<(), Box<dyn StdError + 'static>> {
                return Result::Ok(());
            }
            fn _precedent__channel_publication1_commentary__delete(precedent: ChannelPublication1Commentary_Delete_Precedent_) -> Result<(), Box<dyn StdError + 'static>> {
                let unified_report = UnifiedReport::<Void, ChannelPublication1Commentary_Delete_Precedent_>::precedent(precedent);
                return run_by_template(
                    &unified_report,
                    channel_publication1_commentary__delete__deserialize_allocate,
                    channel_publication1_commentary__delete__deserialize_deallocate,
                );
            }
            pub fn precedent__channel_publication1_commentary__delete() -> Result<(), Box<dyn StdError + 'static>> {
                match ChannelPublication1Commentary_Delete_Precedent_::UserAccessToken__AlreadyExpired {
                    ChannelPublication1Commentary_Delete_Precedent_::UserAccessToken__AlreadyExpired => {}
                    ChannelPublication1Commentary_Delete_Precedent_::ChannelPublication1Token__AlreadyExpired => {}
                    ChannelPublication1Commentary_Delete_Precedent_::ChannelPublication1Commentary__NotFound => {}
                }
                let precedents: Vec<ChannelPublication1Commentary_Delete_Precedent_> = vec![
                    ChannelPublication1Commentary_Delete_Precedent_::UserAccessToken__AlreadyExpired,
                    ChannelPublication1Commentary_Delete_Precedent_::ChannelPublication1Token__AlreadyExpired,
                    ChannelPublication1Commentary_Delete_Precedent_::ChannelPublication1Commentary__NotFound,
                ];
                '_a: for precedent in precedents {
                    _precedent__channel_publication1_commentary__delete(precedent)?;
                }
                return Result::Ok(());
            }
        }
    }
}
