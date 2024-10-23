use sails_rs::{
    prelude::*,
    collections::HashMap,
};

pub static mut USER_STATE: Option<UserState> = None;


#[derive(Clone, Default)]
pub struct UserState {
    pub name: String,
    pub lastnames: String,
    pub age: u128,
    pub ine: String,
    pub rfc: String,
    pub locality: String,
    pub phone: String,
    pub email: String,
    pub password: String,
    pub users: HashMap<ActorId, IoUser>, 
}

#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct IoUser {
    pub name: String,
    pub lastnames: String,
    pub age: u128,
    pub ine: String,
    pub rfc: String,
    pub locality: String,
    pub phone: String,
    pub email: String,
    pub password: String,
}


impl From<UserState> for IoUser {
    fn from(value: UserState) -> Self {
        Self {
            name: value.name,
            lastnames: value.lastnames,
            age: value.age,
            ine: value.ine,
            rfc: value.rfc,
            locality: value.locality,
            phone: value.phone,
            email: value.email,
            password: value.password,
        }
    }
}





