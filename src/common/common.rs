use serde::{Serialize, Deserialize,};
use std::result;
use mongodb::{Collection, bson, bson::Document};
use crate::{MONGO, GLOBAL_CONF};
use serde::de::{DeserializeOwned};
use thiserror::Error;


use std::convert::Infallible;
use axum::{response::IntoResponse, response};
use axum::body::{Bytes, Full};
use axum::http::header::ToStrError;
use axum::response::Response;
use axum::http::{StatusCode};

use crate::template::{ArticleListRenderItem, ArticleRenderTag};
use crate::model::article::Article;
use chrono::{NaiveDateTime, DateTime, Utc};


pub const OK: i32 = 0;
pub const OKMSG: &str = "";
pub const NORMAL_ERROR: i32 = -10000;
pub const VALIDATE_ERROR: i32 = -10001;
pub const UPLOAD_ERROR: i32 = -10002;
pub const NEEDLOGIN_ERROR: i32 = -10003;


#[derive(Serialize, Deserialize)]
pub struct Resp<T>
    where T: Serialize
{
    pub(crate) code: i32,
    pub(crate) msg: Option<String>,
    pub(crate) data: Option<T>,
}

impl<T: Serialize> Resp<T> {
    pub fn ok(data: T) -> Self {
        Resp {
            code: OK,
            msg: None,
            data: Some(data),
        }
    }

    // pub fn to_json(&self) -> CommonResp { Ok(HttpResponse::Ok().json(self)) }

    pub fn into_json(self) -> serde_json::Value {
        serde_json::json!({
            "code":self.code,
            "msg":self.msg,
            "data":self.data,
        })
    }
}
impl Resp<()> {
    pub fn ok_msg(msg: &str) -> Self {
        Resp {
            code: OK,
            msg: Some(msg.to_string()),
            data: None,
        }
    }
    pub fn err(codenumber: i32, msg: &str) -> Self {
        Resp {
            code: codenumber,
            msg: Some(msg.to_string()),
            data: None,
        }
    }
    pub fn err_msg(msg: &str) -> Self {
        Resp {
            code: NORMAL_ERROR,
            msg: Some(msg.to_string()),
            data: None,
        }
    }
}

impl<T> IntoResponse for Resp<T>
    where
        T: Send + Sync + Serialize,
{
    // type Body = Full<Bytes>;
    // type BodyError = Infallible;
    
    fn into_response(self) -> Response {
        let mut respon = response::Json(self.into_json())
            .into_response();
        *respon.status_mut() = StatusCode::OK;
        respon
    }
}



pub type WebError = BaseError;
pub type Result<T> = result::Result<T, WebError>;
// pub type CommonResp = Result<HttpResponse>;

#[derive(Debug, Serialize, Deserialize, Error,)]
pub enum BaseError {
    #[error("Validation error on field: {field:?}")]
    ValidationError { field: String },
    #[error("An internal error : {field:?}")]
    CommonError { field: String },
    #[error("Login Auth Failed")]
    LoginAuthFailedError,
    #[error("upload failed")]
    UploadError,
    #[error("No record found")]
    NoRecordFound,
    #[error("Data duplication")]
    DataDuplication,
    #[error("{info:?}")]
    Other{info:String},

}

impl From<anyhow::Error>  for BaseError {
    fn from(inner: anyhow::Error) -> Self {
        BaseError::Other{
            info:inner.to_string()
        }
    }
}

impl From<ToStrError>  for BaseError {
    fn from(inner: ToStrError) -> Self {
        BaseError::Other{
            info:inner.to_string()
        }
    }
}


impl IntoResponse for BaseError {
    // type Body = Full<Bytes>;
    // type BodyError = Infallible;

    fn into_response(self) -> Response {
        let code = match self {
            BaseError::ValidationError { field: _, } => StatusCode::OK,
            BaseError::LoginAuthFailedError => StatusCode::OK,
            _ => StatusCode::OK,
        };

        let  responsedata =  match self {
            BaseError::ValidationError { .. } => Resp::err(VALIDATE_ERROR, &self.to_string()),
            BaseError::CommonError { .. } => Resp::err(NORMAL_ERROR, &self.to_string()),
            BaseError::LoginAuthFailedError { .. } => Resp::err(NEEDLOGIN_ERROR, &self.to_string()),
            BaseError::UploadError { .. } => Resp::err(UPLOAD_ERROR, &self.to_string()),
            _ => Resp::err(UPLOAD_ERROR, &self.to_string()),
        };

        let mut response = response::Json(responsedata.into_json())
            .into_response();
        *response.status_mut() = code;

        response
    }
}

pub trait IntoDocument
    where
        Self: Sized + Serialize + DeserializeOwned
{
    fn to_document(&self) -> Option<Document> {
        let mid = bson::to_bson(self)
            .ok()
            .map(|x| x.as_document().unwrap().to_owned());

        mid.map(|mut doc| {
            let keys = doc.keys();
            let rm: Vec<String> = keys
                .filter(|k| doc.is_null(k))
                .map(|x| x.to_owned())
                .collect();
            for x in rm {
                doc.remove(&x);
            }
            doc
        })
    }
}

pub fn table(coll_name: &str) -> Collection::<Document> {
    MONGO
        .database(&GLOBAL_CONF.mongo.db_name)
        .collection(coll_name)
}


pub fn  article_to_article_list_render_item(article: Article,odd_number:bool)-> ArticleListRenderItem{
    let mut id = "".to_string();
    match article._id {
        Some(d) => {
            id = d.to_string();
        }
        None => {}
    }
    let mut title = "".to_string();
    match article.title {
        Some(d) => {
            title = d;
        }
        None => {}
    }
    let mut describe = "".to_string();
    match article.describe {
        Some(d) => {
            describe = d;
        }
        None => {}
    }
    let mut cover_pic = "".to_string();
    match article.cover_pic {
        Some(d) => {
            cover_pic = d;
        }
        None => {}
    }
    let mut create_time = "".to_string();
    match article.create_time {
        Some(d) => {
            match d.parse::<i64>() {
                Ok(millis)=>{
                    let naive = NaiveDateTime::from_timestamp((millis/ 1000) as i64, 0);
                    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
                    create_time =  datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                },
                Err(_e)=>{
                }
            } ;
        }
        None => {}
    }
    let mut update_time = "".to_string();
    match article.update_time {
        Some(d) => {
            match d.parse::<i64>() {
                Ok(millis)=>{
                    let naive = NaiveDateTime::from_timestamp((millis/ 1000) as i64, 0);
                    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
                    update_time =  datetime.format("%y-%m-%d").to_string();
                },
                Err(_e)=>{
                }
            } ;
        }
        None => {}
    }
    let mut tags:Vec<ArticleRenderTag>  = vec![];
    match  article.tags{
        Some(d)=>{
            for tag in d {
                let mut tagid = "".to_string();
                match tag.id {
                    Some(tid)=>{
                        tagid = tid;
                    },
                    None=>{}
                }
                let mut tagname = "".to_string();
                match tag.name {
                    Some(tname)=>{
                        tagname = tname;
                    },
                    None=>{
                    }
                }
                tags.push(ArticleRenderTag{
                    id:tagid,
                    name: tagname
                });
            }
        },
        None=>{}
    }
    ArticleListRenderItem {
        id,
        title,
        describe,
        cover_pic,
        create_time,
        update_time,
        tags,
        odd_number
    }

}