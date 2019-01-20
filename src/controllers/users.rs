use crate::models::user::User;

use rocket_contrib::json::{Json, JsonValue};

#[post("/", data="<user>")]
pub fn create(user: Json<User>) -> Json<User> {
    user
}
