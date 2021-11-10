use super::{User};
use anyhow::Result;
use mongodb::bson::doc;
use crate::common::IntoDocument;

// base 获取某个 User
pub async fn get_user(phone: &i64,password:&str) -> Result<User> {
    super::find(User::TABLE_NAME,Some(doc!{
          "phone": phone,
          "password": password,
    })).await
}

pub async fn get_user_for_phonne(phone: &i64) -> Result<User> {
    super::find(User::TABLE_NAME,Some(doc!{
          "phone": phone,
    })).await
}

pub async fn save_user(user:&User) -> Result<String>{
    super::save(User::TABLE_NAME,user).await
}

pub async fn update_user_info(user:&User) -> Result<u64> {
    super::update(User::TABLE_NAME,&user._id.unwrap().to_string(),user.to_document().unwrap()).await
}

// mod test {
//     use super::*;
//     #[tokio::test]
//    async fn test_find_user() {
//         let phone = 15173135646;
//         let mut  v =  get_user(&phone,"").await;
//         println!("结果 33333{:?}", v);
//     }
//
//     #[tokio::test]
//     async fn test_insert_user() {
//         let mut u  = User::new();
//         u.name= Some("测试".to_string());
//         u.phone = Some(15173135646);
//         u.password = Some("".to_string());
//         let mut  v =  save_user(&u).await;
//         println!("保存结果{:?}", v);
//     }
//
// }

