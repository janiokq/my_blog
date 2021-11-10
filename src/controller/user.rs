use crate::{ GLOBAL_CONF};
use crate::model::user::*;
use axum::extract::Json;
use crate::common::{Resp, OK, WebError, OKMSG, Claims, generate_token};
use serde::{Serialize, Deserialize};
use crate::middleware::auth::BlogAuth;


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUser {
    pub phone: i64,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUserResult {
    pub user: User,
    pub token: String,
}

pub async fn login(
    Json(user): Json<LoginUser>,
) -> Result<Resp<LoginUserResult>, WebError> {
    let user = get_user(&user.phone, &*user.password).await?;
    let phone = user.phone.unwrap();
    let name = user.name.as_ref().unwrap().to_string();
    let token = generate_token(&Claims {
        phone,
        username: name,
        exp: GLOBAL_CONF.jwt.claim_exp,
    })?;

    Ok(Resp {
        code: OK,
        data: Some(LoginUserResult {
            user,
            token,
        }),
        msg: Some(OKMSG.to_string()),
    })
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUser {
    pub name: String,
    pub password: String,
    pub avatar: String,
    pub phone: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateUserResult {
    pub user: User,
}

pub async fn update_user(
    _auth: BlogAuth,
    Json(update_user): Json<UpdateUser>,
) -> Result<Resp<UpdateUserResult>, WebError> {
    let mut user = get_user_for_phonne(&update_user.phone).await?;
    user.name = Option::from(update_user.name);
    if !update_user.password.eq("") {
        user.change_password(update_user.password);
    }

    user.avatar = Option::from(update_user.avatar);
    let res = update_user_info(&user).await?;

    Ok(Resp {
        code: OK,
        data: Some(UpdateUserResult {
            user,
        }),
        msg: Some(res.to_string()),
    })
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CosKey {
    pub secret_id: String,
    pub secret_key: String,
}

pub async fn cos_key(
    _auth: BlogAuth,
) -> Result<Resp<CosKey>, WebError> {
    Ok(Resp {
        code: OK,
        data: Some(CosKey {
            secret_id:"".to_string(),
            secret_key:"".to_string()
        }),
        msg: Some(OKMSG.to_string()),
    })
}


pub async fn test(
    // Extension(auth): Extension<MyAuth>,
    _auth: BlogAuth,
    Json(user): Json<LoginUser>,
) -> Result<Resp<LoginUserResult>, WebError> {
    Ok(Resp {
        code: OK,
        data: None,
        msg: Some(user.phone.to_string()),
    })
}