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

struct Error {
  bool is_exist;
};

struct StructWithString2 {
  StructWithString1 struct_with_string;
  Error error;
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

} // extern "C"
