pub mod handler;

pub use handler::*;
use serde::{Deserialize, Serialize};
use super::{list, save, find, remove};
use mongodb::{ bson::oid::ObjectId, bson::*};
use chrono::Local;
use crate::common::{IntoDocument};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tag {
    pub _id: Option<ObjectId>,
    pub name: Option<String>,
    pub create_time: Option<String>,
    pub user_phone: Option<i64>,
}

impl Tag {
    pub const TABLE_NAME: &'static str = "blog_tag";
    pub fn new() -> Self {
        Tag {
            _id: None,
            name: None,
            user_phone: None,
            create_time: Some(Local::now().timestamp_millis().to_string()),
        }
    }
}

impl IntoDocument for Tag {}

