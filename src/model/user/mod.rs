pub mod handler;
pub use handler::*;
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use super::{ save,find,update};
use chrono::Local;
use crate::common::IntoDocument;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    #[serde(rename(serialize = "u_id"))]
    _id: Option<ObjectId>,
   pub name: Option<String>,
   pub phone: Option<i64>,
    password: Option<String>,
   pub avatar: Option<String>,
   pub create_time: Option<String>,
}

impl  User {
    pub const TABLE_NAME: &'static str = "user_info";
    pub fn change_password(&mut self, new_password:String) {
        self.password = Option::from(new_password);
    }

    pub fn new() -> Self {
        User {
            _id: None,
            name: None,
            phone: None,
            password: None,
            avatar: None,
            create_time: Some(Local::now().timestamp_millis().to_string()),
        }
    }

}

impl IntoDocument for User {}