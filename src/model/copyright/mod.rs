pub mod handler;
pub use handler::*;
use super::{ save, find,update};
use serde::{Deserialize, Serialize};
use chrono::Local;
use crate::common::IntoDocument;
use mongodb::{bson::oid::ObjectId};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Link {
    pub name: Option<String>,
    pub link: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Copyright {
    pub _id: Option<ObjectId>,
    pub content: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub user_phone: Option<String>,
    pub seo: Option<String>,
    pub link: Option<Vec<Link>>,
}

impl Copyright {
    pub const TABLE_NAME: &'static str = "blog_copyright";
    pub fn new() -> Self {
        Copyright {
            _id: None,
            content: Some("".to_string()),
            user_phone: Some("".to_string()),
            seo:Some("".to_string()),
            link:Some(Vec::new()),
            update_time: Some(Local::now().timestamp_millis().to_string()),
            create_time: Some(Local::now().timestamp_millis().to_string()),
        }
    }
}

impl IntoDocument for Copyright {}

