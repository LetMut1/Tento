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