#![no_std]

// necesary crates
use sails_rs::prelude::*;

// Importar los módulos de servicios y estados
pub mod services;
pub mod states;

use services::user_service::UserRegistrationService;


// User registration program struct to build the program
pub struct UserRegistrationProgram;

// User registration program, it hosts one or more services
#[program]
impl UserRegistrationProgram {
    // Application constructor (it is an associated function)
    // It can be called once per application lifetime.
    pub fn new() -> Self {
        // Init the state
        UserRegistrationService::seed();

        Self
    }

    // Method to expose the user registration service
    #[route("UserRegistration")]
    pub fn user_registration_svc(&self) -> UserRegistrationService {
        UserRegistrationService::new()
    }
}