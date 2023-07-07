use crate::domain_layer::data::entity::application_user::ApplicationUser_Email;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Id;
use crate::domain_layer::data::entity::application_user::ApplicationUser_Nickname;
use crate::domain_layer::data::entity::application_user_device::ApplicationUserDevice_Id;
use std::marker::PhantomData;

pub struct PostgresqlRepository<E> {
    _entity: PhantomData<E>,
}

pub struct By1<'a> {
    pub application_user_nickname: &'a ApplicationUser_Nickname,
}

pub struct By2<'a> {
    pub application_user_email: &'a ApplicationUser_Email,
}

pub struct By3 {
    pub application_user_id: ApplicationUser_Id,
}

pub struct By4<'a> {
    pub application_user_id: ApplicationUser_Id,
    pub application_user_device_id: &'a ApplicationUserDevice_Id,
}
