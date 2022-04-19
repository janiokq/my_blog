use crate::middleware::auth::BlogAuth;
use axum::extract::Json;
use crate::model::article::{Article, save_article, update_article, search_article_db, search_article_db_count, get_article, del_article_in_db};
use crate::common::{Resp, WebError, OK, OKMSG};
use anyhow::Result;
use serde::{Serialize, Deserialize};

pub async fn set_article(auth: BlogAuth, Json(new_article_info): Json<Article>) -> Result<Resp<Article>, WebError> {
    let mut new_art = Article::new();
    new_art.content = new_article_info.content;
    new_art.author = auth.user.phone;
    new_art.cover_pic = new_article_info.cover_pic;
    new_art.title = new_article_info.title;
    new_art.describe = new_article_info.describe;
    new_art.tags = new_article_info.tags;
    match new_article_info.create_time {
        Some(time)=>{
            new_art.create_time  = Some(time);
        },
        None=>{
        }
    } 
    // newArt.id = newArticleInfo.id;
    match new_article_info.id {
        Some(_) => {
            //更新
            match update_article(&new_art, new_article_info.id.unwrap().as_str()).await {
                Ok(_r) => {
                    Ok(Resp {
                        code: OK,
                        data: Some(new_art),
                        msg: Some(OKMSG.to_string()),
                    })
                }
                Err(e) => {
                    Err(WebError::CommonError {
                        field: e.to_string()
                    })
                }
            }
        }
        None => {
            match save_article(&new_art).await {
                Ok(r) => {
                    new_art.id = Some(r);
                    Ok(Resp {
                        code: OK,
                        data: Some(new_art),
                        msg: Some(OKMSG.to_string()),
                    })
                }
                Err(e) => {
                    Err(WebError::CommonError {
                        field: e.to_string()
                    })
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchArticleInfo {
    pub keyword: Option<String>,
    // pub id: Option<String>,
    pub page: u64,
    pub page_size: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchArticleResponse {
    pub data: Option<Vec<Article>>,
    pub page: Option<u64>,
    pub total: Option<u64>,
}
pub async fn search_article(_auth: BlogAuth, Json(search_info): Json<SearchArticleInfo>) -> Result<Resp<SearchArticleResponse>, WebError> {
    let keyword = match search_info.keyword {
        Some(d) => {
            Some(d)
        }
        _ => {
            None
        }
    };

    let mut response_data  =  SearchArticleResponse{
        data:None,
        page:Some(search_info.page.clone()),
        total:None,
    };

    response_data.total = match search_article_db_count(keyword.clone(),None,None).await {
        Ok(count)=>{
            Some(count)
        },
        Err(_e)=>{
            None
        }
    };

    match search_article_db(keyword, search_info.page, search_info.page_size,None,None).await {
        Ok(r) => {
            response_data.data = Some(r);
            Ok(Resp {
                code: OK,
                data: Some(response_data),
                msg: Some(OKMSG.to_string()),
            })
        }
        Err(e) => {
            Err(WebError::CommonError {
                field: e.to_string()
            })
        }
    }
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FindArticleInfo {
    pub id: String,
}
pub async fn find_article(_auth: BlogAuth,Json(find_info): Json<FindArticleInfo>)-> Result<Resp<Article>, WebError>{
    match  get_article(&find_info.id).await {
        Ok(r)=>{
            Ok(Resp {
                code: OK,
                data: Some(r),
                msg: Some(OKMSG.to_string()),
            })
        },
        Err(e)=>{
            Err(WebError::CommonError {
                field:e.to_string()
            })
        }
    }
}

pub async fn del_article(_auth: BlogAuth,Json(find_info): Json<FindArticleInfo>)-> Result<Resp<u64>, WebError>{
    match  del_article_in_db(&find_info.id).await {
        Ok(r)=>{
            Ok(Resp {
                code: OK,
                data: Some(r),
                msg: Some(OKMSG.to_string()),
            })
        },
        Err(e)=>{
            Err(WebError::CommonError {
                field:e.to_string()
            })
        }
    }
}