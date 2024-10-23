use sails_rs::{
    prelude::*,
    gstd::msg,
};


use crate::states::user_state::{UserRegistrationState, IoUserRegistrationState};



#[derive(Default)]
pub struct UserRegistrationService;


impl UserRegistrationService {
    
    pub fn seed() {
        UserRegistrationState::init_state();
    }

    
    pub fn register_user(&mut self, user_name: String) {
        
        UserRegistrationState::state_mut()
            .all_users
            .insert(msg::source().into(), user_name);
    }
}


#[service]
impl UserRegistrationService {
    
    pub fn new() -> Self {
        Self
    }

    
   pub fn register(&mut self, user_name: String) -> bool {
    // Lógica de registro del usuario
    if user_name.is_empty() {
        return false; // Devuelve false si el nombre de usuario está vacío
    }

    // Supongamos que el registro es exitoso
    true // Devuelve true si el registro es exitoso
}

    
    pub fn get_users(&self) -> IoUserRegistrationState {
        UserRegistrationState::state_ref().to_owned().into()
    }
}