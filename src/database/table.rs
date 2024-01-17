use mysql::prelude::*;

#[derive(FromRow)]
pub struct User {
    id: i32,
    account: String,
    password: String,
}