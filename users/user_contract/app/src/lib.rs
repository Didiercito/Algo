#![no_std]

use sails_rs::prelude::*;

pub mod states;
pub mod services;

use services::user_service::Service as UserService;

pub struct UserRegistrationProgram;

#[program]
impl UserRegistrationProgram {
    pub fn new() -> Self {
        UserService::seed(); // Asumiendo que tienes un método `seed` en tu servicio de usuarios.

        Self
    }

    #[route("UserRegistration")]
    pub fn user_registration_svc(&self) -> UserService {
        UserService::new()
    }
}
