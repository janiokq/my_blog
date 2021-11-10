use crate::model::article::Article;
use anyhow::Result;
use crate::common::IntoDocument;
use mongodb::{options::FindOptions,bson::*,bson::Document};
use std::option::Option;
use mongodb::options::CountOptions;


pub async fn get_article(id:&str)->Result<Article>{
    super::get(Article::TABLE_NAME,id).await
}

pub async fn del_article_in_db(id:&str)->Result<u64>{
    super::remove(Article::TABLE_NAME,id).await
}

pub async fn update_article(article: &Article,id:&str) -> Result<u64> {
    super::update(Article::TABLE_NAME, id, article.to_document().unwrap()).await
}

pub async fn save_article(article: &Article) -> Result<String> {
    super::save(Article::TABLE_NAME, article).await
}

pub async fn search_article_db_count(keywords:Option<String>,tag:Option<String>,projection_tag:Option<Vec<String>>)-> Result<u64> {
    let  option = CountOptions::default();
    let mut filter:Document = doc! {};

    match  projection_tag {
        Some(k)=>{
            for x in k {
                filter.insert("tags",doc!{"$elemMatch":doc!{"name":doc!{"$ne": x.as_str()}}} );
            }
        },
        None=>{
        }
    };

    match  keywords {
        Some(k)=>{
            filter.insert("title",
                          doc!{"$regex": k.as_str(), "$options": "i"},
            );
        },
        None=>{
        }
    };
    match  tag {
        Some(k)=>{
            filter.insert("tags",doc!{"$elemMatch":doc!{"name":doc!{"$eq": k.as_str()}}} );
        },
        None=>{
        }
    };


    super::count(Article::TABLE_NAME, Some(filter), option).await
}

pub async fn search_article_db(keywords:Option<String>,page_number:u64,page_size:i64,tag:Option<String>,projection_tag:Option<Vec<String>>)-> Result<Vec<Article>>{
    let mut option = FindOptions::default();

    let  projection:Document = doc! {
        "content":0
    };


    option.projection = Some(projection);
    option.sort  = Some(doc!{"create_time":-1});
    option.limit= Some(page_size);
    option.skip = Some(page_number*(page_size as u64));
    let mut filter:Document = doc! {};

    match  projection_tag {
        Some(k)=>{
            for x in k {
                filter.insert("tags",doc!{"$elemMatch":doc!{"name":doc!{"$ne": x.as_str()}}} );
            }
        },
        None=>{
        }
    };
    match  keywords {
        Some(k)=>{
            filter.insert("title",
                          doc!{"$regex": k.as_str(), "$options": "i"},
            );
        },
        None=>{
        }
    };

    match  tag {
        Some(k)=>{
            filter.insert("tags",doc!{"$elemMatch":doc!{"name":doc!{"$eq": k.as_str()}}} );
        },
        None=>{
        }
    };


    super::list(Article::TABLE_NAME, Some(filter), option).await
}
