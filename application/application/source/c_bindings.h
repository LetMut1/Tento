#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct A {
  int a;
};

struct B {
  int a;
};

struct C {
  int a;
  bool b;
};

struct StructWithString1 {
  bool is_exist;
  char *string;
};

struct Error2 {
  bool is_exist;
};

struct StructWithString2 {
  StructWithString1 struct_with_string;
  Error2 error;
};

struct Opaque {
  bool public_;
  bool _private;
};

template<typename T>
struct StructWithGeneric {
  T a;
  bool b;
};

struct Nested1 {
  bool a;
  bool b;
  bool c;
  char *string;
};

struct Main1 {
  Nested1 nested1;
};

struct Nested2 {
  bool a;
};

struct Main2 {
  Nested1 nested1;
  Nested2 nested2;
};

template<typename T>
struct C_Vector {
  T *pointer;
  size_t length;
};

template<typename T>
struct C_Result {
  T data;
  bool is_data;
};

struct C_String {
  char *pointer;
};

struct ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming {
  C_String application_user_device_id;
  C_String application_user_email_or_application_user_nickname;
  C_String application_user_password;
};

struct ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming {
  long application_user_id;
  bool verification_message_sent;
  long application_user_authorization_token_can_be_resent_from;
  short application_user_authorization_token_wrong_enter_tries_quantity;
  short application_user_authorization_token_wrong_enter_tries_quantity_limit;
};

struct ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent {
  bool application_user__wrong_email_or_nickname_or_password;
};

template<typename T>
struct C_Data {
  T filled;
  bool is_filled;
};

template<typename D, typename P>
struct C_UnifiedReport {
  C_Data<D> target;
  P precedent;
  bool is_target;
};

using ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming, ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent>>;

struct ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming {
  C_String application_user_access_token_encrypted;
  C_String application_user_access_refresh_token_encrypted;
};

struct ApplicationUserAuthorizationToken_WrongValue {
  bool is_exist;
  short application_user_authorization_token_wrong_enter_tries_quantity;
};

struct ApplicationUser__Authorization___AuthorizeByLastStep___Precedent {
  bool application_user_authorization_token__not_found;
  bool application_user_authorization_token__already_expired;
  ApplicationUserAuthorizationToken_WrongValue application_user_authorization_token__wrong_value;
  bool application_user__not_found;
};

using ApplicationUser__Authorization___AuthorizeByLastStep___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___AuthorizeByLastStep___Outcoming, ApplicationUser__Authorization___AuthorizeByLastStep___Precedent>>;

struct ApplicationUser__Authorization___CheckEmailForExisting___Outcoming {
  bool result;
};

struct C_Void {
  bool _inner;
};

using ApplicationUser__Authorization___CheckEmailForExisting___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___CheckEmailForExisting___Outcoming, C_Void>>;

struct ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming {
  bool result;
};

using ApplicationUser__Authorization___CheckNicknameForExisting___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___CheckNicknameForExisting___Outcoming, C_Void>>;

struct ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent {
  bool application_user_access_token__already_expired;
  bool application_user_access_token__in_application_user_access_token_black_list;
};

using ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___DeauthorizeFromAllDevices___Precedent>>;

struct ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent {
  bool application_user_access_token__already_expired;
  bool application_user_access_token__in_application_user_access_token_black_list;
};

using ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___DeauthorizeFromOneDevice___Precedent>>;

struct ApplicationUser__Authorization___RefreshAccessToken___Outcoming {
  C_String application_user_access_token_encrypted;
  C_String application_user_access_refresh_token_encrypted;
};

struct ApplicationUser__Authorization___RefreshAccessToken___Precedent {
  bool application_user_access_refresh_token__not_found;
  bool application_user_access_refresh_token__already_expired;
};

using ApplicationUser__Authorization___RefreshAccessToken___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___RefreshAccessToken___Outcoming, ApplicationUser__Authorization___RefreshAccessToken___Precedent>>;

struct ApplicationUser__Authorization___RegisterByFirstStep___Outcoming {
  bool verification_message_sent;
  long application_user_registration_token_can_be_resent_from;
  short application_user_registration_token_wrong_enter_tries_quantity;
  short application_user_registration_token_wrong_enter_tries_quantity_limit;
};

struct ApplicationUser__Authorization___RegisterByFirstStep___Precedent {
  bool application_user__email_already_exist;
};

using ApplicationUser__Authorization___RegisterByFirstStep___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___RegisterByFirstStep___Outcoming, ApplicationUser__Authorization___RegisterByFirstStep___Precedent>>;

struct ApplicationUserRegistrationToken_WrongValue {
  bool is_exist;
  short application_user_registration_token_wrong_enter_tries_quantity;
};

struct ApplicationUser__Authorization___RegisterBySecondStep___Precedent {
  bool application_user_registration_token__not_found;
  bool application_user_registration_token__already_expired;
  bool application_user_registration_token__already_approved;
  ApplicationUserRegistrationToken_WrongValue application_user_registration_token__wrong_value;
};

using ApplicationUser__Authorization___RegisterBySecondStep___C_Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___RegisterBySecondStep___Precedent>>;

struct ApplicationUser__Authorization___RegisterByLastStep___Outcoming {
  C_String application_user_access_token_encrypted;
  C_String application_user_access_refresh_token_encrypted;
};

struct ApplicationUser__Authorization___RegisterByLastStep___Precedent {
  bool application_user__nickname_already_exist;
  bool application_user__email_already_exist;
  bool application_user_registration_token__not_found;
  bool application_user_registration_token__already_expired;
  bool application_user_registration_token__is_not_approved;
  bool application_user_registration_token__wrong_value;
};

using ApplicationUser__Authorization___RegisterByLastStep___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___RegisterByLastStep___Outcoming, ApplicationUser__Authorization___RegisterByLastStep___Precedent>>;

struct ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming {
  long application_user_id;
  bool verification_message_sent;
  long application_user_reset_password_token_can_be_resent_from;
  short application_user_reset_password_token_wrong_enter_tries_quantity;
  short application_user_reset_password_token_wrong_enter_tries_quantity_limit;
};

struct ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent {
  bool application_user__not_found;
};

using ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___ResetPasswordByFirstStep___Outcoming, ApplicationUser__Authorization___ResetPasswordByFirstStep___Precedent>>;

struct ApplicationUserResetPasswordToken_WrongValue {
  bool is_exist;
  short application_user_reset_password_token_wrong_enter_tries_quantity;
};

struct ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent {
  bool application_user_reset_password_token__not_found;
  bool application_user_reset_password_token__already_expired;
  bool application_user_reset_password_token__already_approved;
  ApplicationUserResetPasswordToken_WrongValue application_user_reset_password_token__wrong_value;
};

using ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___ResetPasswordBySecondStep___Precedent>>;

struct ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent {
  bool application_user__not_found;
  bool application_user_reset_password_token__not_found;
  bool application_user_reset_password_token__already_expired;
  bool application_user_reset_password_token__is_not_approved;
  bool application_user_reset_password_token__wrong_value;
};

using ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result = C_Result<C_UnifiedReport<C_Void, ApplicationUser__Authorization___ResetPasswordByLastStep___Precedent>>;

struct ApplicationUser__Authorization___SendEmailForRegister___Outcoming {
  long application_user_registration_token_can_be_resent_from;
};

struct ApplicationUser__Authorization___SendEmailForRegister___Precedent {
  bool application_user_registration_token__not_found;
  bool application_user_registration_token__already_expired;
  bool application_user_registration_token__already_approved;
  bool application_user_registration_token__time_to_resend_has_not_come;
};

using ApplicationUser__Authorization___SendEmailForRegister___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForRegister___Outcoming, ApplicationUser__Authorization___SendEmailForRegister___Precedent>>;

struct ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming {
  long application_user_authorization_token_can_be_resent_from;
};

struct ApplicationUser__Authorization___SendEmailForAuthorize___Precedent {
  bool application_user__not_found;
  bool application_user_authorization_token__not_found;
  bool application_user_authorization_token__already_expired;
  bool application_user_authorization_token__time_to_resend_has_not_come;
};

using ApplicationUser__Authorization___SendEmailForAuthorize___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForAuthorize___Outcoming, ApplicationUser__Authorization___SendEmailForAuthorize___Precedent>>;

struct ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming {
  long application_user_resep_password_token_can_be_resent_from;
};

struct ApplicationUser__Authorization___SendEmailForResetPassword___Precedent {
  bool application_user__not_found;
  bool application_user_reset_password_token__not_found;
  bool application_user_reset_password_token__already_expired;
  bool application_user_reset_password_token__already_approved;
  bool application_user_reset_password_token__time_to_resend_has_not_come;
};

using ApplicationUser__Authorization___SendEmailForResetPassword___C_Result = C_Result<C_UnifiedReport<ApplicationUser__Authorization___SendEmailForResetPassword___Outcoming, ApplicationUser__Authorization___SendEmailForResetPassword___Precedent>>;

template<typename T>
struct C_Option {
  T data;
  bool is_data;
};

struct Channel1 {
  long channel_id;
  C_String channel_name;
  C_String channel_linked_name;
  short channel_access_modifier;
  short channel_visability_modifier;
  C_Option<C_String> channel_cover_image_path;
  C_Option<C_String> channel_background_image_path;
};

struct Common1 {
  Channel1 channel;
  bool is_application_user_subscribed;
};

struct Channel__Base___GetManyByNameInSubscriptions___Outcoming {
  C_Vector<Common1> common_registry;
};

struct Channel__Base___GetManyByNameInSubscriptions___Precedent {
  bool application_user_access_token__already_expired;
  bool application_user_access_token__in_application_user_access_token_black_list;
};

using Channel__Base___GetManyByNameInSubscriptions___C_Result = C_Result<C_UnifiedReport<Channel__Base___GetManyByNameInSubscriptions___Outcoming, Channel__Base___GetManyByNameInSubscriptions___Precedent>>;

struct Channel__Base___GetManyBySubscription___Outcoming {
  C_Vector<Common1> common_registry;
};

struct Channel__Base___GetManyBySubscription___Precedent {
  bool application_user_access_token__already_expired;
  bool application_user_access_token__in_application_user_access_token_black_list;
};

using Channel__Base___GetManyBySubscription___C_Result = C_Result<C_UnifiedReport<Channel__Base___GetManyBySubscription___Outcoming, Channel__Base___GetManyBySubscription___Precedent>>;

struct Channel__Base___GetManyPublicByName___Outcoming {
  C_Vector<Common1> common_registry;
};

struct Channel__Base___GetManyPublicByName___Precedent {
  bool application_user_access_token__already_expired;
  bool application_user_access_token__in_application_user_access_token_black_list;
};

using Channel__Base___GetManyPublicByName___C_Result = C_Result<C_UnifiedReport<Channel__Base___GetManyPublicByName___Outcoming, Channel__Base___GetManyPublicByName___Precedent>>;

struct Channel2 {
  long channel_owner;
  C_String channel_name;
  C_String channel_linked_name;
  C_Option<C_String> channel_description;
  short channel_access_modifier;
  short channel_visability_modifier;
  C_Vector<short> channel_orientation;
  C_Option<C_String> channel_cover_image_path;
  C_Option<C_String> channel_background_image_path;
  long channel_subscribers_quantity;
  long channel_marks_quantity;
  long channel_viewing_quantity;
};

struct ChannelInnerLink1 {
  long channel_inner_link_to;
};

struct ChannelOuterLink1 {
  C_String channel_outer_link_alias;
  C_String channel_outer_link_address;
};

struct Channel__Base___GetOneById___Outcoming {
  Channel2 channel;
  C_Vector<ChannelInnerLink1> channel_inner_link_registry;
  C_Vector<ChannelOuterLink1> channel_outer_link_registry;
};

struct Channel__Base___GetOneById___Precedent {
  bool application_user_access_token__already_expired;
  bool application_user_access_token__in_application_user_access_token_black_list;
  bool channel__not_found;
  bool channel__is_close;
};

using Channel__Base___GetOneById___C_Result = C_Result<C_UnifiedReport<Channel__Base___GetOneById___Outcoming, Channel__Base___GetOneById___Precedent>>;

struct ChannelSubscription__Base___Create___Precedent {
  bool application_user_access_token__already_expired;
  bool application_user_access_token__in_application_user_access_token_black_list;
  bool channel__not_found;
  bool channel__is_close;
  bool application_user__is_channel_owner;
};

using ChannelSubscription__Base___Create___C_Result = C_Result<C_UnifiedReport<C_Void, ChannelSubscription__Base___Create___Precedent>>;

extern "C" {

int f1(int a);

bool f2(bool a);

double f3(double a);

int f4(A a);

B f5(A a);

bool f6(A a, B b);

int f7(const int *a);

int f8(int *a);

int f9(const A *a);

int f10(A *a);

bool f11(A *a, B *b);

C f12(C *a, int b);

void f13(C *a, bool b);

char *string_allocate_f1();

void string_deallocate_f1(char *string);

StructWithString1 *string_allocate_f2();

void string_deallocate_f2(StructWithString1 *struct_with_string);

StructWithString2 *string_allocate_f3();

void string_deallocate_f3(StructWithString2 *struct_with_string);

unsigned char array_slice_f1(unsigned char *pointer_to_first_element_of_registry,
                             size_t registry_length);

bool opaque_f1(Opaque *opaque);

StructWithGeneric<char> *generic_allocate_f1();

void generic_deallocate_f1(StructWithGeneric<char> *struct_with_generic);

StructWithGeneric<char*> *generic_allocate_f2();

void generic_deallocate_f2(StructWithGeneric<char*> *struct_with_generic);

Main1 *main_nested_allocate_f1();

void main_nested_deallocate_f1(Main1 *main);

Main2 *main_nested_allocate_f2();

void main_nested_deallocate_f2(Main2 *main);

C_Result<C_Vector<unsigned char>> *application_user___authorization____authorize_by_first_step____serialize(ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming *incoming);

ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result *application_user___authorization____authorize_by_first_step____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____authorize_by_first_step____deallocate(ApplicationUser__Authorization___AuthorizeByFirstStep___C_Result *c_result);

ApplicationUser__Authorization___AuthorizeByLastStep___C_Result *application_user___authorization____authorize_by_last_step____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____authorize_by_last_step____deallocate(ApplicationUser__Authorization___AuthorizeByLastStep___C_Result *c_result);

ApplicationUser__Authorization___CheckEmailForExisting___C_Result *application_user___authorization____check_email_for_existing____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____check_email_for_existing____deallocate(ApplicationUser__Authorization___CheckEmailForExisting___C_Result *c_result);

ApplicationUser__Authorization___CheckNicknameForExisting___C_Result *application_user___authorization____check_nickname_for_existing____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____check_nickname_for_existing____deallocate(ApplicationUser__Authorization___CheckNicknameForExisting___C_Result *c_result);

ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result *application_user___authorization____deauthorize_from_all_devices____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____deauthorize_from_all_devices____deallocate(ApplicationUser__Authorization___DeauthorizeFromAllDevices___C_Result *c_result);

ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result *application_user___authorization____deauthorize_from_one_device____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____deauthorize_from_one_device____deallocate(ApplicationUser__Authorization___DeauthorizeFromOneDevice___C_Result *c_result);

ApplicationUser__Authorization___RefreshAccessToken___C_Result *application_user___authorization____refresh_access_token____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____refresh_access_token____deallocate(ApplicationUser__Authorization___RefreshAccessToken___C_Result *c_result);

ApplicationUser__Authorization___RegisterByFirstStep___C_Result *application_user___authorization____register_by_first_step____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____register_by_first_step____deallocate(ApplicationUser__Authorization___RegisterByFirstStep___C_Result *c_result);

ApplicationUser__Authorization___RegisterBySecondStep___C_Result *application_user___authorization____register_by_second_step____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____register_by_second_step____deallocate(ApplicationUser__Authorization___RegisterBySecondStep___C_Result *c_result);

ApplicationUser__Authorization___RegisterByLastStep___C_Result *application_user___authorization____register_by_last_step____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____register_by_last_step____deallocate(ApplicationUser__Authorization___RegisterByLastStep___C_Result *c_result);

ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result *application_user___authorization____reset_password_by_first_step____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____reset_password_by_first_step____deallocate(ApplicationUser__Authorization___ResetPasswordByFirstStep___C_Result *c_result);

ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result *application_user___authorization____reset_password_by_second_step____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____reset_password_by_second_step____deallocate(ApplicationUser__Authorization___ResetPasswordBySecondStep___C_Result *c_result);

ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result *application_user___authorization____reset_password_by_last_step____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____reset_password_by_last_step____deallocate(ApplicationUser__Authorization___ResetPasswordByLastStep___C_Result *c_result);

ApplicationUser__Authorization___SendEmailForRegister___C_Result *application_user___authorization____send_email_for_register____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____send_email_for_register____deallocate(ApplicationUser__Authorization___SendEmailForRegister___C_Result *c_result);

ApplicationUser__Authorization___SendEmailForAuthorize___C_Result *application_user___authorization____send_email_for_authorize____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____send_email_for_authorize____deallocate(ApplicationUser__Authorization___SendEmailForAuthorize___C_Result *c_result);

ApplicationUser__Authorization___SendEmailForResetPassword___C_Result *application_user___authorization____send_email_for_reset_password____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void application_user___authorization____send_email_for_reset_password____deallocate(ApplicationUser__Authorization___SendEmailForResetPassword___C_Result *c_result);

Channel__Base___GetManyByNameInSubscriptions___C_Result *channel___base____get_many_by_name_in_subscriptions____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void channel___base____get_many_by_name_in_subscriptions____deallocate(Channel__Base___GetManyByNameInSubscriptions___C_Result *c_result);

Channel__Base___GetManyBySubscription___C_Result *channel___base____get_many_by_subscription____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void channel___base____get_many_by_subscription____deallocate(Channel__Base___GetManyBySubscription___C_Result *c_result);

Channel__Base___GetManyPublicByName___C_Result *channel___base____get_many_public_by_name____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void channel___base____get_many_public_by_name____deallocate(Channel__Base___GetManyPublicByName___C_Result *c_result);

Channel__Base___GetOneById___C_Result *channel___base____get_one_by_id____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void channel___base____get_one_by_id____deallocate(Channel__Base___GetOneById___C_Result *c_result);

ChannelSubscription__Base___Create___C_Result *channel_subscription___base____create____deserialize(C_Vector<unsigned char> *vector_of_bytes);

void channel_subscription___base____create____deallocate(ChannelSubscription__Base___Create___C_Result *c_result);

} // extern "C"
