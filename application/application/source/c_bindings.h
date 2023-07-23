#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct A {
  int x;
};

struct B {
  int x;
};

extern "C" {

bool is_equal_to_1(int x);

bool is_x_equal_to_x(A *a, B *b);

} // extern "C"
