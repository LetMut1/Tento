#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct A {
  unsigned char a;
  unsigned char b;
} A;

typedef struct B {
  unsigned char a;
  unsigned char b;
} B;

typedef struct C {
  unsigned char a;
  char *string_pointer;
} C;

char get_number(void);

bool is_goyda(void);

bool convert_to_bool(char number);

bool check_for_nigger_word(const char *pointer_to_string);

char *get_nigger_word__allocate(unsigned char quantity);

void get_nigger_word__deallocate(char *pointer_to_string);

struct A get_a(unsigned char a);

unsigned short calculate_a_sum(struct A a);

struct B *get_b__allocate(unsigned char a);

void get_b__deallocate(struct B *b);

unsigned short calculate_b_sum(struct B *b);

struct C *get_c__allocate(void);

void get_c__deallocate(struct C *c);

bool is_c__1_nigger___pointer(struct C *c);

bool is_c__1_nigger___full_struct(struct C c);
