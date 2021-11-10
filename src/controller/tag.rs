use axum::extract::Json;
use crate::common::{Resp, OK, WebError, OKMSG};
use serde::{Serialize, Deserialize};
use crate::middleware::auth::BlogAuth;
use crate::model::tag::{Tag, list_all_tag, contains_tag_in_db, save_tag, del_tag_in_db, search_tag_in_db};
use chrono::Local;
use mongodb::{ bson::oid::ObjectId};
use std::str::FromStr;

pub async fn list_tags(
    _auth: BlogAuth,
) -> Result<Resp<Vec<Tag>>, WebError> {
    match list_all_tag().await {
        Ok(r) => {
            Ok(Resp {
                code: OK,
                data: Some(r),
                msg: Some(OKMSG.to_string()),
            })
        }
        Err(_e) => {
            // e.into()
            Err(WebError::NoRecordFound)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagInfo {
    pub name: String,
}
pub async fn add_tag(auth: BlogAuth,Json(new_tag_info): Json<TagInfo>, ) -> Result<Resp<Tag>, WebError> {
     match  contains_tag_in_db(&new_tag_info.name).await {
         Ok(_)=>{
             Err(WebError::DataDuplication)
         },
         Err(_e)=>{
            let mut t =  Tag::new();
             t.name = Some(new_tag_info.name.to_string());
             t.create_time = Some(Local::now().timestamp_millis().to_string());
             t.user_phone = auth.user.phone;
             match  save_tag(&t).await {
                 Ok(d)=>{

                     t._id  = Some(ObjectId::from_str(&d).unwrap());

                     Ok(Resp {
                         code: OK,
                         data: Some(t),
                         msg: Some(OKMSG.to_string()),
                     })
                 },
                 Err(_e)=>{
                     Ok(Resp {
                         code: -1,
                         data: None,
                         msg: Some("保存失败".to_string()),
                     })
                 }
             }
         }
     }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DelTagInfo {
    pub id: String,
}
pub async fn del_tag(_auth: BlogAuth,Json(tag_info): Json<DelTagInfo>)-> Result<Resp<u64>,WebError>{
    match del_tag_in_db(&tag_info.id).await {
        Ok(r)=>{
            Ok(Resp {
                code: OK,
                data: Some(r),
                msg: Some(OKMSG.to_string()),
            })
        },
        Err(e)=>{
            Err(WebError::CommonError{
                field: e.to_string()
            })
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchTagInfo {
    pub name: String,
}
pub async fn search_tag(_auth: BlogAuth,Json(search_info): Json<SearchTagInfo>) -> Result<Resp<Vec<Tag>>,WebError>{
   match search_tag_in_db(&search_info.name).await {
       Ok(r)=>{
           Ok(Resp {
               code: OK,
               data: Some(r),
               msg: Some(OKMSG.to_string()),
           })
       },
       Err(e)=>{
           Err(WebError::CommonError{
               field: e.to_string()
           })
       }
   }

}