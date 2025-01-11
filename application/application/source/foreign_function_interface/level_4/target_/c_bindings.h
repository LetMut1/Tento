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

typedef struct CVector_c_uchar {
  unsigned char *pointer;
  size_t length;
} CVector_c_uchar;

typedef struct CResult_CVector_c_uchar {
  struct CVector_c_uchar data;
  bool is_data;
} CResult_CVector_c_uchar;

typedef struct CString {
  char *pointer;
} CString;

typedef struct UserAuthorization_RegisterByFirstStep_Incoming {
  struct CString user__email;
  struct CString user_device__id;
} UserAuthorization_RegisterByFirstStep_Incoming;

typedef struct UserAuthorization_RegisterByFirstStep_Outcoming {
  bool verification_message_sent;
  long user_registration_token__can_be_resent_from;
  short user_registration_token__wrong_enter_tries_quantity;
  short user_registration_token__wrong_enter_tries_quantity_limit;
} UserAuthorization_RegisterByFirstStep_Outcoming;

typedef struct CData_UserAuthorization_RegisterByFirstStep_Outcoming {
  struct UserAuthorization_RegisterByFirstStep_Outcoming filled;
  bool is_filled;
} CData_UserAuthorization_RegisterByFirstStep_Outcoming;

typedef struct UserAuthorization_RegisterByFirstStep_Precedent {
  bool user__email_already_exist;
} UserAuthorization_RegisterByFirstStep_Precedent;

typedef struct CUnifiedReport_UserAuthorization_RegisterByFirstStep_Outcoming__UserAuthorization_RegisterByFirstStep_Precedent {
  struct CData_UserAuthorization_RegisterByFirstStep_Outcoming target;
  struct UserAuthorization_RegisterByFirstStep_Precedent precedent;
  bool is_target;
} CUnifiedReport_UserAuthorization_RegisterByFirstStep_Outcoming__UserAuthorization_RegisterByFirstStep_Precedent;

typedef struct CResult_CUnifiedReport_UserAuthorization_RegisterByFirstStep_Outcoming__UserAuthorization_RegisterByFirstStep_Precedent {
  struct CUnifiedReport_UserAuthorization_RegisterByFirstStep_Outcoming__UserAuthorization_RegisterByFirstStep_Precedent data;
  bool is_data;
} CResult_CUnifiedReport_UserAuthorization_RegisterByFirstStep_Outcoming__UserAuthorization_RegisterByFirstStep_Precedent;

typedef struct CResult_CUnifiedReport_UserAuthorization_RegisterByFirstStep_Outcoming__UserAuthorization_RegisterByFirstStep_Precedent UserAuthorization_RegisterByFirstStep_CResult;

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

bool is_c__1_nigger(struct C *c);

bool is_c__1_nigger___full_struct(struct C c);

struct CResult_CVector_c_uchar *user_authorization__register_by_first_step__serialize_allocate(struct UserAuthorization_RegisterByFirstStep_Incoming *incoming);

void user_authorization__register_by_first_step__serialize_deallocate(struct CResult_CVector_c_uchar *c_result);

UserAuthorization_RegisterByFirstStep_CResult *user_authorization__register_by_first_step__deserialize_allocate(struct CVector_c_uchar *c_vector_of_bytes);

void user_authorization__register_by_first_step__deserialize_deallocate(UserAuthorization_RegisterByFirstStep_CResult *c_result);
