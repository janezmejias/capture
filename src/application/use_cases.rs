use validator::ValidationError;
use crate::domain::entities::User;

pub fn find_user(id: u32) -> Result<User, ValidationError> {
    User::new(id, "janez", 34)
}
