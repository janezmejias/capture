use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};
use validator_derive::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct User {
    #[validate(range(min = 1))]
    id: u32,

    #[validate(length(min = 2, max = 50))]
    name: String,

    #[validate(range(min = 1, max = 150))]
    age: u16,
}

impl User {
    pub fn new(id: u32, name: &str, age: u16) -> Result<Self, ValidationError> {
        let entity = User {
            id,
            name: name.to_string(),
            age,
        };
        let _ =entity.validate();

        Ok(entity)
    }
}
