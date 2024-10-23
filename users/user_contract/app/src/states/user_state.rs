use sails_rs::{
    prelude::*,
    collections::HashMap,
};


pub static mut USER_REGISTRATION_STATE: Option<UserRegistrationState> = None;


#[derive(Clone, Default)]
pub struct UserRegistrationState {
    pub all_users: HashMap<ActorId, String>, 
}


impl UserRegistrationState {

    pub fn new() -> Self {
        Self {
            all_users: HashMap::new()
        }
    }

    
    pub fn init_state() {
        unsafe {
            USER_REGISTRATION_STATE = Some(Self::new());
        };
    }

    
    pub fn state_mut() -> &'static mut UserRegistrationState {
        let state = unsafe { USER_REGISTRATION_STATE.as_mut() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }

    
    pub fn state_ref() -> &'static UserRegistrationState {
        let state = unsafe { USER_REGISTRATION_STATE.as_ref() };
        debug_assert!(state.is_some(), "The state is not initialized");
        unsafe { state.unwrap_unchecked() }
    }
}


#[derive(Encode, Decode, TypeInfo)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct IoUserRegistrationState {
    pub all_users: Vec<(ActorId, String)>,
}


impl From<UserRegistrationState> for IoUserRegistrationState {
    fn from(value: UserRegistrationState) -> Self {
        let all_users = value
            .all_users
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect();

        Self {
            all_users,
        }
    }
}