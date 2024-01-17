use crate::api::common::*;
use crate::model::user;
use regex::Regex;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/user")
            .service(web::resource("/login").route(web::post().to(login)))
            .service(web::resource("/logout").route(web::post().to(logout)))
            .service(web::resource("/register").route(web::post().to(register)))
    );
}

#[derive(Debug, Deserialize)]
struct LoginRequest {
   account: String,
   password: String,
}

#[derive(Debug, Deserialize)]
struct LogoutRequest {
   account: String,
}

#[derive(Debug, Deserialize)]
struct RegisterRequest {
   account: String,
   password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
}

/** 用戶登入 */
async fn login(request: web::Json<LoginRequest>) -> impl Responder {
    let request = request.into_inner();
    let mut response = create_response();

    // 檢查帳號長度
    if request.account.len() < constant::USER_ACCOUNT_MIN_LENGTH || request.account.len() > constant::USER_ACCOUNT_MAX_LENGTH {
        response.code = error_code::USER_ACCOUNT_INVALID;
        response.msg = String::from(error_msg::USER_ACCOUNT_INVALID);
        return HttpResponse::BadRequest().json(response);
    }

    // 暫時用不到 因為密碼會先經過加密
    // 檢查密碼格式
    // if !check_password_regex(&request.password) {
    //     response.code = error_code::USER_LOGIN_PASSWORD_INVALID;
    //     response.msg = String::from(error_msg::USER_LOGIN_PASSWORD_INVALID);
    //     return HttpResponse::BadRequest().json(response);
    // }

    // 取得用戶資訊
    let md5_password = md5::compute(request.password);
    let user = user::get_user_by_password(request.account, format!("{:?}", md5_password));
    if user.len() == 0 {
        response.code = error_code::USER_LOGIN_ACCOUNT_OR_PASSWORD_WRONG;
        response.msg = String::from(error_msg::USER_LOGIN_ACCOUNT_OR_PASSWORD_WRONG);
        return HttpResponse::BadRequest().json(response);
    }

    response.data = Some(LoginResponse {
        token: String::from("token"), // todo 產token
    });
    HttpResponse::Ok().json(response)
}

/** 用戶登出 */
async fn logout(request: web::Json<LogoutRequest>) -> impl Responder {
    let request = request.into_inner();
    let mut response = create_response();
    if request.account.is_empty() {
        response.code = error_code::USER_ACCOUNT_INVALID;
        response.msg = String::from(error_msg::USER_ACCOUNT_INVALID);
        return HttpResponse::Ok().json(response);
    }

    response.data = Some(EmptyResponse{});
    HttpResponse::Ok().json(response)
}

/** 用戶註冊 */
async fn register(request: web::Json<RegisterRequest>) -> impl Responder {
    let request = request.into_inner();
    let mut response = create_response();

    // 檢查帳號長度
    if request.account.len() < constant::USER_ACCOUNT_MIN_LENGTH || request.account.len() > constant::USER_ACCOUNT_MAX_LENGTH {
        response.code = error_code::USER_ACCOUNT_INVALID;
        response.msg = String::from(error_msg::USER_ACCOUNT_INVALID);
        return HttpResponse::BadRequest().json(response);
    }

    // 新增用戶
    let md5_password = md5::compute(request.password);
    let result = user::add_user(request.account, format!("{:?}", md5_password));
    if !result {
        response.code = error_code::USER_REGISTER_FAILED;
        response.msg = String::from(error_msg::USER_REGISTER_FAILED);
        return HttpResponse::BadRequest().json(response);
    }

    response.data = Some(EmptyResponse{});
    HttpResponse::Ok().json(response)
}

/** 檢查密碼格式(至少包含大小寫英文+數字) */
fn check_password_regex(password: &str) -> bool {
    let lowercase = Regex::new(r"[a-z]").unwrap();
    let uppercase = Regex::new(r"[A-Z]").unwrap();
    let digit = Regex::new(r"\d").unwrap();
    if !lowercase.is_match(password) || !uppercase.is_match(password) || !digit.is_match(password) {
        return false;
    }

    true
}