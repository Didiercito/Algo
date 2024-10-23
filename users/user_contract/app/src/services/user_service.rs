mod UserState;

use gstd::exec::sleep_for;
use sails_rs::{
    gstd::{
        service,
        msg
    },
    prelude::*
};

use crate::states::user_service::{
    STATE,
    CustomStruct,
    IoCustomStruct,
    CustomInput
};

use crate::UserState::UserState;


#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Events {
    registerUser {
        user: UserState,
    }
}


#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum Errors {
    FirstError,
    SecondError,
    ThirdErrors,
    FourtErrors,
}


#[derive(Default)]
pub struct Service;

#[service]
impl Service {
    pub fn new() -> Self {
        Self
    }
}


pub async fn register_user(&mut self, username: String) -> Result<Events, Errors> {
    let state: &mut CustomStruct =
        unsafe { STATE.as_mut().expect("El contrato no está inicializado") };

    let user_id = msg::source(); 

    if state.users.contains_key(&user_id) {
        return Err(Errors::FirstError); 
    }

    let new_user = UserState {
        user_id,
        username: username.clone(),
    };

    state.users.insert(user_id, new_user);

    sleep_for(10).await;

    Ok(Events::registerUser {
        user: UserState {
            user_id,
            username,
        },
    })
}
