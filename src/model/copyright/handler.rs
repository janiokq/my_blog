use super::{Copyright};
use anyhow::Result;
use mongodb::bson::doc;
use crate::common::IntoDocument;

pub async fn get_copyright_for_phone(phone: &str) -> Result<Copyright> {
    super::find(Copyright::TABLE_NAME, Some(doc! {
          "user_phone": phone,
    })).await
}

pub async fn save_copyright(copyright: &Copyright) -> Result<String> {
    super::save(Copyright::TABLE_NAME, copyright).await
}

pub async fn update_copyright(copyright: &Copyright) -> Result<u64> {
    super::update(Copyright::TABLE_NAME, &copyright._id.unwrap().to_string(),copyright.to_document().unwrap()).await
}