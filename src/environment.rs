use crate::errors::{EnvironmentErrorType, LoxErrors};
use crate::token::{Object, Token};

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    /// This hashmap contains variables names and values of the running lox code
    static ref VALUES: Mutex<HashMap<String, Object>> = Mutex::new(HashMap::new());
}

pub struct Environment {}

impl Environment {
    /// Useless
    pub fn new() -> Self {
        Environment {}
    }

    /**
     * Inserts a key-value pair in the global HashMap storage.
     */
    pub fn define(&self, name: String, obj: Object) {
        // Insert the value after locking the mutex
        VALUES.lock().unwrap().insert(name, obj);
    }

    /**
     * Gets a value using its key name from the storage HashMap.
     *
     * Note: Throws an error if the key does not exist.
     */
    pub fn get(&self, token: Token) -> Result<Object, LoxErrors> {
        // Lock the mutex and try to get the value
        match VALUES.lock().unwrap().get(&token.lexeme) {
            Some(v) => Ok(v.clone()),
            None => Err(LoxErrors::Environment {
                error_type: EnvironmentErrorType::UnknownVariable,
                msg: format!(
                    "{} -> No such variable '{}'.",
                    token.location(),
                    token.lexeme
                ),
            }),
        }
    }
}
