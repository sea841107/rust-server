use crate::utils::file_system;

use mysql::*;
use mysql::prelude::*;

#[derive(Debug, serde::Deserialize)]
struct Config {
    mysql: Option<Mysql>,
}

#[derive(Debug, serde::Deserialize)]
struct Mysql {
    user: String,
    password: String,
    host: String,
    port: u16,
    database_name: String,
}

static mut MYSQL_CONFIG: Config = Config { mysql: None };

pub fn init() {
    unsafe {
        MYSQL_CONFIG = file_system::read_toml("Config.toml").unwrap();
    }
}

// SELECT
pub fn query<T, Q, P>(sql: Q, params: P) -> Result<Vec<T>, mysql::Error>
where
    T: FromRow,
    Q: AsRef<str>,
    P: Into<Params>,
{
    let mut conn = get_connection()?;

    let stmt = conn.prep(&sql)?;
    let results = conn.exec(stmt, params);

    match results {
        Ok(res) => Ok(res),
        Err(error) => {
            println!("Query failed: Error:[{}]", error);
            Err(error)
        }
    }
}

// INSERT, UPDATE, DELETE
pub fn query_drop<Q, P>(sql: Q, params: P) -> Result<(), mysql::Error>
where
    Q: AsRef<str>,
    P: Into<Params>,
{
    let mut conn = get_connection()?;

    let stmt = conn.prep(sql)?;
    let results = conn.exec_drop(stmt, params);

    match results {
        Ok(res) => Ok(res),
        Err(error) => {
            println!("Query failed: Error:[{}]", error);
            Err(error)
        }
    }
}

fn get_connection() -> Result<PooledConn> {
    let mysql = match unsafe { &MYSQL_CONFIG.mysql } {
        Some(mysql) => mysql,
        None => {
            println!("MySQL configuration not found");
            return Err(mysql::Error::from(std::io::Error::new(
                std::io::ErrorKind::Other,
                "MySQL configuration not found",
            )));
        }
    };
    let opts = OptsBuilder::new()
        .user(Some(&mysql.user))
        .pass(Some(&mysql.password))
        .ip_or_hostname(Some(&mysql.host))
        .tcp_port(mysql.port)
        .db_name(Some(&mysql.database_name));

    let pool = Pool::new(opts)?;
    pool.get_conn()
}