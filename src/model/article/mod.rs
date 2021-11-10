pub mod handler;

pub use handler::*;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use super::{get, list, save,update,count,remove};
use chrono::Local;
use crate::common::IntoDocument;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleTag {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Article {
    pub _id: Option<ObjectId>,
    pub id:Option<String>,
    pub title: Option<String>,
    pub describe: Option<String>,
    pub cover_pic: Option<String>,
    pub tags: Option<Vec<ArticleTag>>,
    pub content: Option<String>,
    pub author: Option<i64>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}


impl Article {
    pub const TABLE_NAME: &'static str = "blog_article";
    pub fn new() -> Self {
        Article {
            _id: None,
            title: None,
            describe: None,
            cover_pic: None,
            tags: None,
            content: None,
            author: None,
            id:None,
            update_time: Some(Local::now().timestamp_millis().to_string()),
            create_time: Some(Local::now().timestamp_millis().to_string()),
        }
    }
}

impl IntoDocument for Article {}


