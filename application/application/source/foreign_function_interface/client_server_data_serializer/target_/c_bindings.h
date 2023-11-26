#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct A {
  int a;
} A;

typedef struct B {
  int a;
} B;

typedef struct C {
  int a;
  bool b;
} C;

typedef struct StructWithString1 {
  bool is_exist;
  char *string;
} StructWithString1;

typedef struct ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming {
  bool is_exist;
  char *string;
} ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming;

typedef struct Error2 {
  bool is_exist;
} Error2;

typedef struct StructWithString2 {
  struct StructWithString1 struct_with_string;
  struct Error2 error;
} StructWithString2;

typedef struct Opaque {
  bool public_;
  bool _private;
} Opaque;

typedef struct StructWithGeneric_c_char {
  char a;
  bool b;
} StructWithGeneric_c_char;

typedef struct StructWithGeneric_____c_char {
  char *a;
  bool b;
} StructWithGeneric_____c_char;

typedef struct Nested1 {
  bool a;
  bool b;
  bool c;
  char *string;
} Nested1;

typedef struct Main1 {
  struct Nested1 nested1;
} Main1;

typedef struct Nested2 {
  bool a;
} Nested2;

typedef struct Main2 {
  struct Nested1 nested1;
  struct Nested2 nested2;
} Main2;

typedef struct C_Vector_c_uchar {
  unsigned char *pointer;
  size_t length;
} C_Vector_c_uchar;

typedef struct C_Result_C_Vector_c_uchar {
  struct C_Vector_c_uchar data;
  bool is_data;
} C_Result_C_Vector_c_uchar;

typedef struct C_String {
  char *pointer;
} C_String;

typedef struct ApplicationUser__Authorization___CheckEmailForExisting___Incoming {
  struct C_String application_user_email;
} ApplicationUser__Authorization___CheckEmailForExisting___Incoming;

typedef struct ApplicationUser__Authorization___CheckEmailForExisting___Outcoming {
  bool result;
} ApplicationUser__Authorization___CheckEmailForExisting___Outcoming;

typedef struct C_Data_ApplicationUser__Authorization___CheckEmailForExisting___Outcoming {
  struct ApplicationUser__Authorization___CheckEmailForExisting___Outcoming filled;
  bool is_filled;
} C_Data_ApplicationUser__Authorization___CheckEmailForExisting___Outcoming;

typedef struct C_Void {
  bool _inner;
} C_Void;

typedef struct C_UnifiedReport_ApplicationUser__Authorization___CheckEmailForExisting___Outcoming__C_Void {
  struct C_Data_ApplicationUser__Authorization___CheckEmailForExisting___Outcoming target;
  struct C_Void precedent;
  bool is_target;
} C_UnifiedReport_ApplicationUser__Authorization___CheckEmailForExisting___Outcoming__C_Void;

typedef struct C_Result_C_UnifiedReport_ApplicationUser__Authorization___CheckEmailForExisting___Outcoming__C_Void {
  struct C_UnifiedReport_ApplicationUser__Authorization___CheckEmailForExisting___Outcoming__C_Void data;
  bool is_data;
} C_Result_C_UnifiedReport_ApplicationUser__Authorization___CheckEmailForExisting___Outcoming__C_Void;

typedef struct C_Result_C_UnifiedReport_ApplicationUser__Authorization___CheckEmailForExisting___Outcoming__C_Void ApplicationUser__Authorization___CheckEmailForExisting___C_Result;

int f1(int a);

int application_user___authorization____authorize_by_first_step____serialize(int a);

bool f2(bool a);

double f3(double a);

int f4(struct A a);

struct B f5(struct A a);

bool f6(struct A a, struct B b);

int f7(const int *a);

int f8(int *a);

int f9(const struct A *a);

int f10(struct A *a);

bool f11(struct A *a, struct B *b);

struct C f12(struct C *a, int b);

void f13(struct C *a, bool b);

char *string_allocate_f1(void);

void string_deallocate_f1(char *string);

struct StructWithString1 *string_allocate_f2(void);

void string_deallocate_f2(struct StructWithString1 *struct_with_string);

struct ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming *string_allocate_f22(void);

void string_deallocate_f22(struct ApplicationUser__Authorization___AuthorizeByFirstStep___Incoming *struct_with_string);

struct StructWithString2 *string_allocate_f3(void);

void string_deallocate_f3(struct StructWithString2 *struct_with_string);

unsigned char array_slice_f1(unsigned char *pointer_to_first_element_of_registry,
                             size_t registry_length);

bool opaque_f1(struct Opaque *opaque);

struct StructWithGeneric_c_char *generic_allocate_f1(void);

void generic_deallocate_f1(struct StructWithGeneric_c_char *struct_with_generic);

struct StructWithGeneric_____c_char *generic_allocate_f2(void);

void generic_deallocate_f2(struct StructWithGeneric_____c_char *struct_with_generic);

struct Main1 *main_nested_allocate_f1(void);

void main_nested_deallocate_f1(struct Main1 *main);

struct Main2 *main_nested_allocate_f2(void);

void main_nested_deallocate_f2(struct Main2 *main);

struct C_Result_C_Vector_c_uchar *application_user___authorization____check_email_for_existing____serialize(struct ApplicationUser__Authorization___CheckEmailForExisting___Incoming *incoming);

void application_user___authorization____check_email_for_existing____serialize____deallocate(struct C_Result_C_Vector_c_uchar *c_result);

ApplicationUser__Authorization___CheckEmailForExisting___C_Result *application_user___authorization____check_email_for_existing____deserialize(struct C_Vector_c_uchar *vector_of_bytes);

void application_user___authorization____check_email_for_existing____deserialize____deallocate(ApplicationUser__Authorization___CheckEmailForExisting___C_Result *c_result);
