mod api {
    pub mod user;
    pub mod common;
    pub mod constant;
    pub mod error_code;
    pub mod error_msg;
}

mod database {
    pub mod mysql;
    pub mod table;
}

mod utils {
    pub mod file_system;
}

mod model {
    pub mod user;
    pub mod common;
}

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use database::mysql;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    mysql::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::default() // 默认配置，允许所有请求
                .allowed_origin("http://localhost:3000")
                .allowed_methods(vec!["GET", "POST"])
                .allow_any_header()
                .max_age(3600)) // 預檢請求緩存時間
            .configure(api::user::config)
    })
    .bind("127.0.0.1:1234")?
    .run()
    .await
}
