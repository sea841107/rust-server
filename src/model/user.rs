use crate::model::common::*;

pub fn get_user_by_password(account: String, password: String) -> Vec<table::User> {
    let sql = format!("
        SELECT id, account, password FROM user
        WHERE account = :account AND password = :password
    ");

    let params = params! {
        "account" => account,
        "password" => password,
    };

    mysql::query(sql, params).unwrap()
}

pub fn add_user(account: String, password: String) -> bool {
    let sql = "
        INSERT INTO user (account, password)
        VALUES(:account, :password)
    ";

    let params = params! {
        "account" => account,
        "password" => password,
    };

    match mysql::query_drop(sql, params) {
        Ok(_) => true,
        Err(_) => false,
    }
}