pub mod user;
pub mod article;
pub mod tag;
pub mod copyright;
use anyhow::Result;
use crate::common::{table, IntoDocument,WebError,BaseError};
use std::str::FromStr;
use futures_util::TryStreamExt;
use mongodb::{options::FindOptions,bson,bson::oid::ObjectId,bson::*,bson::Document};
use mongodb::options::CountOptions;

// 获取对应的数据
pub async fn get<T>(table_name: &str, id: &str) -> Result<T>
where
    T: IntoDocument  + ?Sized,
{
    let filter = Some(doc!{"_id": ObjectId::from_str(id)?} );
    let document = table(table_name).find_one(filter, None).await?;
    match document {
        Some(doc)=> {
            Ok(bson::from_document( doc )?)
        },
        None=> {
            Err(BaseError::NoRecordFound.into())
        }
    }
}
// 查找对应的数据
pub async fn find<T>(table_name: &str, filter: Option<Document>) -> Result<T>
    where
        T: IntoDocument + ?Sized,
{
    let document = table(table_name).find_one(filter, None).await?;
    match document {
        Some(d)=>{
            Ok(bson::from_document(d)?)
        },
        None=>{
            Err(WebError::NoRecordFound.into())
        }
    }
}

pub async fn count(table_name: &str, filter: Option<Document>, find_options: CountOptions) -> Result<u64> {

    let  count =   table(table_name).count_documents(filter, Some(find_options)).await?;
    Ok(count)
}

// 获取对应的数据vec
pub async fn list<T>(table_name: &str, filter: Option<Document>, find_options: FindOptions) -> Result<Vec<T>>
    where
        T: IntoDocument + ?Sized,
{

    let mut cursor = table(table_name).find(filter, Some(find_options)).await?;
    let mut result: Vec<T> = vec![];
    while let Some(data) = cursor.try_next().await? {
        match bson::from_document(data).unwrap() {
            Some(d)=>  {
                result.push(d)
            } ,
            None => {
                // tracing::error!("conversion bson to struct failure ",);
            },
        }
    }
    Ok(result)
}
// 保存对应的数据
pub async fn save<T>(table_name: &str, model: &T) -> Result<String>
    where  T: IntoDocument + ?Sized,
{
    let mut d =  bson::to_bson(&model).map(|x|x.as_document().unwrap().to_owned()).unwrap();
    d.remove("_id");
    match table(table_name).insert_one(d, None).await {
        Ok(rs) => {
            let new_id = rs.inserted_id.as_object_id().expect("cant find object_id");
            // unwrap如果错误，则panic,而\?则比较聪明，如果unwrap失败则直接return err,不会panic,否则返回unwrap之后的值。
            Ok(new_id.to_string())
        }
        Err(e) => Err(e.into()),
    }
}


// 删除对应的数据
pub async fn remove(table_name: &str, id: &str) -> Result<u64>
{
    let filter = doc!{"_id": ObjectId::from_str(id)?};
    let res =  table(table_name).delete_one(filter, None).await?;
    Ok(res.deleted_count)
}
// return : 返回更改的个数
// 更新对应的数据
pub async fn update(table_name: &str, id: &str, document: Document) -> Result<u64> {
    let filter = doc! {"_id": ObjectId::from_str(id)?};
    let update = doc! {"$set":document};
    let res =  table(table_name).update_one(filter, update, None).await?;
    Ok(res.modified_count)
}


// mod test {
//     use super::*;
//     #[test]
//     fn test_print() {
//         println!("{:?}", "model");
//     }
// }

