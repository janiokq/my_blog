use super::{Tag};
use anyhow::Result;
use mongodb::bson::doc;
use mongodb::{options::FindOptions,bson::Document};


pub async fn contains_tag_in_db(name:&str)->Result<Tag>{
    super::find(Tag::TABLE_NAME, Some(doc! {
          "name": name,
    })).await
}

// base 获取某个 User
pub async fn get_tag(id: &str) -> Result<Tag> {
    super::find(Tag::TABLE_NAME, Some(doc! {
          "_id": id,
    })).await
}

pub async fn get_tag_for_name(name: String) -> Result<Tag> {
    super::find(Tag::TABLE_NAME, Some(doc! {
          "name": name,
    })).await
}

pub async fn save_tag(tag: &Tag) -> Result<String> {
    super::save(Tag::TABLE_NAME, tag).await
}

pub async fn del_tag_in_db(id: &str)-> Result<u64>{
    super::remove(Tag::TABLE_NAME,id).await
}

pub async fn list_all_tag() -> Result<Vec<Tag>> {
    let mut option = FindOptions::default();
    option.sort = Some(doc!{"create_time":-1});
    super::list(Tag::TABLE_NAME, None, option).await
}


pub async fn search_tag_in_db(keywords:&str) -> Result<Vec<Tag>> {
    let  option = FindOptions::default();
    let  fiter: Document = doc! {
        // "name":format!("/{}/",keywords)
        "name": {"$regex": keywords, "$options": "i"}
    };

    super::list(Tag::TABLE_NAME, Some(fiter), option).await
}

