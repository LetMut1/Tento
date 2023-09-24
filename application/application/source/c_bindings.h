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
struct Data {
  T filled;
  bool is_filled;
};

template<typename D, typename P>
struct UnifiedReport {
  Data<D> target;
  P precedent;
  bool is_target;
};

template<typename T>
struct Result {
  T data;
  bool is_data;
};

using ApplicationUser__Authorization___AuthorizeByFirstStep___Result = Result<UnifiedReport<ApplicationUser__Authorization___AuthorizeByFirstStep___Outcoming, ApplicationUser__Authorization___AuthorizeByFirstStep___Precedent>>;

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

ApplicationUser__Authorization___AuthorizeByFirstStep___Result *application_user___authorization____authorize_by_first_step____deserialize(unsigned char *pointer_to_first_element_of_registry,
                                                                                                                                           size_t registry_length);

void application_user___authorization____authorize_by_first_step____deallocate(ApplicationUser__Authorization___AuthorizeByFirstStep___Result *result);

} // extern "C"
