use crate::middleware::auth::BlogAuth;
use crate::common::{Resp, WebError, OK, OKMSG};
use crate::model::copyright::{Copyright, get_copyright_for_phone, save_copyright, Link, update_copyright};
use crate::{GLOBAL_CONF};
use mongodb::{bson::oid::ObjectId};
use std::str::FromStr;
use axum::Json;
use serde::{Deserialize, Serialize};



pub async fn get_copyright(
    _auth: BlogAuth,
) -> Result<Resp<Copyright>, WebError> {
    let phone =  GLOBAL_CONF.email.as_ref().unwrap().clone();

    match get_copyright_for_phone(&phone).await {
        Ok(r) => {
            Ok(Resp {
                code: OK,
                data: Some(r),
                msg: Some(OKMSG.to_string()),
            })
        }
        Err(_e) => {
            let mut copy = Copyright::new();
            copy.content = Some("".to_string());
            copy.user_phone = Some(phone);
            match save_copyright(&copy).await {
                Ok(d) => {
                    copy._id = Some(ObjectId::from_str(&d).unwrap());
                    Ok(Resp {
                        code: OK,
                        data: Some(copy),
                        msg: Some(OKMSG.to_string()),
                    })
                }
                Err(e) => {
                    Err(WebError::CommonError {
                        field: e.to_string()
                    })
                }
            }
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CopyrightContentInfo {
    pub content: String,
    pub seo: String,
    pub link: Vec<Link>,
}
pub async fn save_copyright_content(_auth: BlogAuth,Json(content_info): Json<CopyrightContentInfo>)->Result<Resp<Copyright>, WebError>{
    let phone =  GLOBAL_CONF.email.as_ref().unwrap().clone();
    match get_copyright_for_phone(&phone).await {
        Ok(mut r) => {

            r.content = Some(content_info.content);
            r.link = Some(content_info.link);
            r.seo = Some(content_info.seo);

            match update_copyright(&r).await {
                Ok(_d) => {
                    Ok(Resp {
                        code: OK,
                        data: Some(r),
                        msg: Some(OKMSG.to_string()),
                    })
                }
                Err(e) => {
                    Err(WebError::CommonError {
                        field: e.to_string()
                    })
                }
            }
        }
        Err(e) => {
            Err(WebError::CommonError {
                field:e.to_string()
            })
        }
    }
}