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

} // extern "C"
