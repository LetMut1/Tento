#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef enum C {
  One,
  Two,
} C;

typedef enum D {
  Three,
  Four,
} D;

typedef struct A {
  unsigned char a;
  unsigned char b;
} A;

typedef struct B {
  unsigned char a;
  unsigned char b;
} B;

typedef enum E_Tag {
  Five,
  Six,
} E_Tag;

typedef struct Five_Body {
  char a;
  char *b_pointer_to_string;
} Five_Body;

typedef struct Six_Body {
  bool a;
} Six_Body;

typedef struct E {
  E_Tag tag;
  union {
    Five_Body five;
    Six_Body six;
  };
} E;

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

unsigned short calculate_scd_sum(struct B *b);

enum C get_c(unsigned char number);

bool is_c_one(enum C c);

enum D *get_d__allocate(unsigned char number);

void get_d__deallocate(enum D *d);

bool is_d_one(enum D *d);

struct E *get_e__allocate_1(void);

struct E *get_e__allocate_2(void);

void get_e__deallocate(struct E *e);

bool is_e_one_69_nigger(struct E *e);
