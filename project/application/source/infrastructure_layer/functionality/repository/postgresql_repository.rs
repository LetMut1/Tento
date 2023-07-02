use std::marker::PhantomData;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;

pub struct PostgresqlRepository<E> {
    _entity: PhantomData<E>,
}

pub struct By1<'a> {
    pub  application_user_nickname: &'a ApplicationUser_Nickname,
}

pub struct By2<'a> {
    pub application_user_email: &'a ApplicationUser_Email,
}

pub struct By3 {
    pub application_user_id: ApplicationUser_Id,
}