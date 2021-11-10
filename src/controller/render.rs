use axum::response::{IntoResponse, Html};
use crate::template::{HtmlTemplate, ArticleListRenderItem, BlogTemplate, TagRender, FriendshipLink, ArticleTemplate, LifeTemplate, LifePageTemplate, AboutTemplate, ToolTemplate};
use crate::model::article::{search_article_db_count, search_article_db, Article, get_article, ArticleTag};
use axum::extract::{ Query};
use crate::model::tag::list_all_tag;
use std::collections::HashMap;
use crate::model::copyright::get_copyright_for_phone;
use crate::{GLOBAL_CONF};
use pulldown_cmark::{Options, Parser, html};
use rand::prelude::SliceRandom;
use crate::common::article_to_article_list_render_item;
use askama::Template;
use std::string::String;
use axum::extract;


pub async fn blog(Query(qs): Query<HashMap<String, String>>) -> impl IntoResponse {
    let empty = String::from("");
    let page_value = qs.get("page").unwrap_or(&empty).clone();
    let tag_value = qs.get("tag").unwrap_or(&empty).clone();
    let keyword_value = qs.get("keyword").unwrap_or(&empty).clone();
    let mut page = 0;
    let mut tag = None;
    let mut searchkey = None;
    let page_size = 10;
    let page_range = 5;
    match page_value.parse::<u64>() {
        Ok(v) => {
            if v > 0 {
                page = v - 1;
            } else {
                page = 0;
            }
        }
        Err(_e) => {}
    }
    if !tag_value.eq(&empty) {
        tag = Some(tag_value);
    }
    if !keyword_value.eq(&empty) {
        searchkey = Some(keyword_value);
    }

    blog_list(page_size, page, page_range, searchkey, tag).await
}
pub async fn blog_list(page_size: i64, page: u64, page_range: u64, search_key: Option<String>, tag_name: Option<String>) -> impl IntoResponse {
    let mut projection_tag = Vec::new();
    projection_tag.push("Life".to_string());
    let total = match search_article_db_count(search_key.clone(), tag_name.clone(),Some(projection_tag.clone())).await {
        Ok(count) => {
            count
        }
        Err(_e) => {
            0
        }
    };
    let mut test_list: Vec<ArticleListRenderItem> = Vec::new();
    match search_article_db(search_key.clone(), page, page_size, tag_name.clone(),Some( projection_tag)).await {
        Ok(r) => {
            for x in r {
               let mut has_life = false;
               let tag_list:Vec<ArticleTag> =  x.tags.clone().unwrap_or(Vec::new());
                for tag in tag_list {
                    if tag.name.clone().unwrap_or("".to_string()).eq(&"Life".to_string()) ||  tag.name.clone().unwrap_or("".to_string()).eq(&"About".to_string())  ||  tag.name.unwrap_or("".to_string()).eq(&"Tool".to_string())
                    {
                        has_life = true;
                    }
                }
               if !has_life {
                   test_list.push(article_to_article_list_render_item(x,false));
               }
            }
        }
        Err(_e) => {}
    };
    let all_page = ((total as f64) / (page_size as f64)).ceil() as u64;
    let turn = all_page / page_range;
    let start_page = turn;
    let mut page_list: Vec<u64> = Vec::new();
    let mut end_page = start_page;
    if all_page < (start_page + page_range) {
        end_page += all_page;
    } else {
        end_page += page_range;
    }
    for i in start_page..end_page {
        page_list.push(i + 1);
    }
    let mut next = "".to_string();
    let mut previous = "".to_string();
    if page + 1 < all_page {
        next = (page + 2).to_string();
    }
    if page > 0 {
        previous = page.to_string();
    }
    if page_list.is_empty() {
        page_list.push(1 as u64);
    }
    let new_page_number = page + 1;
    let mut alltags: Vec<TagRender> = Vec::new();
    let mut tag_string = "".to_string();
    match tag_name.clone() {
        Some(d) => {
            tag_string = d;
        }
        None => {}
    };
    match list_all_tag().await {
        Ok(r) => {
            for item in r {
                let mut select = false;
                let mut name = "".to_string();
                match item.name {
                    Some(d) => {
                        name = d;
                    }
                    None => {}
                }
                if tag_string.eq(&name) {
                    select = true;
                }
                if !name.eq("Life") && !name.eq("About")  &&  !name.eq("Tool") {
                    alltags.push(TagRender {
                        name,
                        select,
                    })
                }

            }
        }
        Err(_e) => {}
    }
    let mut tag_string = "".to_string();
    let mut search_key_value = "".to_string();
    match tag_name {
        Some(t) => {
            tag_string = t;
        }
        None => {}
    }
    match search_key {
        Some(s) => {
            search_key_value = s;
        }
        None => {}
    }

    let mut copyright = "".to_string();
    let mut seo = "".to_string();
    let mut link: Vec<FriendshipLink> = Vec::new();
    let phone = GLOBAL_CONF.email.as_ref().unwrap().clone();
    match get_copyright_for_phone(&phone).await {
        Ok(r) => {
            seo = r.seo.unwrap();
            copyright = r.content.unwrap();
            match r.link {
                Some(d) => {
                    for i in d {
                        link.push(FriendshipLink {
                            name: i.name.unwrap(),
                            url: i.link.unwrap(),
                        });
                    }
                }
                None => {}
            }
        }
        Err(_e) => {}
    }

    let template = BlogTemplate {
        title: "Blog".to_string(),
        page: new_page_number.to_string(),
        article_lists: test_list,
        page_list,
        next,
        previous,
        tags: alltags,
        tag: tag_string,
        search_key: search_key_value,
        copyright,
        seo,
        link,
    };
    HtmlTemplate {
        template: template
    }
}
pub async fn article_detail(extract::Path(id): extract::Path<String>) -> impl IntoResponse {
    let article_id = id;
    let mut copyright = "".to_string();
    let mut seo = "".to_string();
    let mut title = "404".to_string();
    let mut content = "404".to_string();
    // let mut tags: Vec<ArticleTag> = Vec::new();
    let mut art: Option<Article> = None;
    match get_article(&article_id).await {
        Ok(r) => {
            art = Some(r.clone());
            seo = r.describe.unwrap() + &*seo;
            title = r.title.unwrap();
            content = r.content.unwrap();
            // match r.tags {
            //     Some(t) => {
            //         tags = t;
            //     }
            //     None => {}
            // }
        }
        Err(_e) => {}
    }
    let phone = GLOBAL_CONF.email.as_ref().unwrap().clone();
    match get_copyright_for_phone(&phone).await {
        Ok(r) => {
            seo = seo + &r.seo.unwrap();
            copyright = r.content.unwrap();
        }
        Err(_e) => {}
    }
    let mut recommend_article: Vec<ArticleListRenderItem> = Vec::new();
    match art {
        Some(a) => {
            recommend_article = generate_relevant_recommendations(a).await;
        }
        None => {}
    }
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&content, options);
    // 写入字符串缓冲区
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let template = ArticleTemplate {
        title,
        content: html_output,
        copyright,
        seo,
        recommend_article,
    };

    HtmlTemplate {
        template: template
    }
}
pub async fn about() -> impl IntoResponse {
    let mut article_id = String::from("");
    let mut seo = "".to_string();
    let mut title = "404".to_string();
    let mut content = "404".to_string();
    match search_article_db(None, 0, 1, Some("About".to_string()),None).await {
        Ok(r) => {
            for x in r {
                article_id = x._id.unwrap().to_string();
            }
        }
        Err(_e) => {}
    }
    match get_article(&article_id).await {
        Ok(r) => {
            seo = r.describe.unwrap() + &*seo;
            title = r.title.unwrap();
            content = r.content.unwrap();
        }
        Err(_e) => {}
    }
    let phone = GLOBAL_CONF.email.as_ref().unwrap().clone();
    match get_copyright_for_phone(&phone).await {
        Ok(r) => {
            seo = seo + &r.seo.unwrap();
        }
        Err(_e) => {}
    }
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&content, options);
    // 写入字符串缓冲区
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    let template = AboutTemplate {
        title,
        content: html_output,
        seo,
    };

    HtmlTemplate {
        template: template
    }
}
pub async fn tool() -> impl IntoResponse {
    let mut article_id = String::from("");
    let mut seo = "".to_string();
    let mut title = "404".to_string();
    let mut content = "404".to_string();
    match search_article_db(None, 0, 1, Some("Tool".to_string()),None).await {
        Ok(r) => {
            for x in r {
                article_id = x._id.unwrap().to_string();
            }
        }
        Err(_e) => {}
    }
    match get_article(&article_id).await {
        Ok(r) => {
            seo = r.describe.unwrap() + &*seo;
            title = r.title.unwrap();
            content = r.content.unwrap();
        }
        Err(_e) => {}
    }
    let phone = GLOBAL_CONF.email.as_ref().unwrap().clone();
    match get_copyright_for_phone(&phone).await {
        Ok(r) => {
            seo = seo + &r.seo.unwrap();
        }
        Err(_e) => {}
    }
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    let parser = Parser::new_ext(&content, options);
    // 写入字符串缓冲区
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    let template = ToolTemplate {
        title,
        content: html_output,
        seo,
    };

    HtmlTemplate {
        template: template
    }
}



pub async fn generate_relevant_recommendations(art: Article) -> Vec<ArticleListRenderItem> {
    let mut recommend_article: Vec<ArticleListRenderItem> = Vec::new();
    let max_rec_item = 5;
    let tags = art.tags.unwrap_or(Vec::new());
    if !tags.is_empty() {
        let taglength = tags.len();
        let _get_item_number = max_rec_item / taglength;
        for x in tags {
            let tagname = x.name;
            let mut all_page = 0;
            match search_article_db_count(None, tagname.clone(),None).await {
                Ok(d) => {
                    all_page = d
                }
                Err(_e) => {}
            }
            let mut nums: Vec<i32> = (0..((all_page / (max_rec_item as u64)) as i32)).collect();
            nums.shuffle(&mut rand::thread_rng());
            if nums.len() == 0 {
                nums.push(0);
            }
            let r = search_article_db(None, nums[0] as u64,
                                      max_rec_item as i64, tagname,None).await.unwrap_or(Vec::new());
            let  maxget_article_number = r.len();
            let mut rng_nums: Vec<i32> = (0..(maxget_article_number as i32)).collect();
            rng_nums.shuffle(&mut rand::thread_rng());
            for item in rng_nums {
                let index = item as usize;
                let article = r[index].clone();
                if !article._id.unwrap().to_string().eq(&art._id.unwrap().to_string()) {
                    recommend_article.push(article_to_article_list_render_item(article,false))
                }
            }

            // if recommend_article.len() >= max_rec_item {
            //     break;
            // }
        }
    }
    let mut remove_len = 0;
    if recommend_article.len() > 5 {
        remove_len = recommend_article.len() - 5;
    }
    if remove_len > 0 {
        let mut remoindex = random_remove_indexs(recommend_article.len() , remove_len );
        remoindex.reverse();
        for x in remoindex {
            recommend_article.remove(x as usize);
        }
    }
    recommend_article
}
pub fn random_remove_indexs(max: usize, items: usize) -> Vec<usize> {
    let mut results: Vec<usize> = Vec::new();
    let mut nums: Vec<usize> = (0..max).collect();
    nums.shuffle(&mut rand::thread_rng());
    for i in 0..items {
        let value = nums[i];
        results.push(value);
    }
    results.sort();


    results
}
pub async fn life() -> impl IntoResponse {
    let page = 0;
    let page_size = 10;
    let articles =  life_list(page_size,page).await;
    let mut copyright = "".to_string();
    let mut seo = "".to_string();
    let mut count = 0;
    let phone = GLOBAL_CONF.email.as_ref().unwrap().clone();
    match get_copyright_for_phone(&phone).await {
        Ok(r) => {
            seo = r.seo.unwrap();
            copyright = r.content.unwrap();
        }
        Err(_e) => {}
    }
    match search_article_db_count(None, Some("Life".to_string()),None).await {
        Ok(d) => {
            count = d
        }
        Err(_e) => {}
    }

    let template = LifeTemplate {
        title: "Life".to_string(),
        article: articles,
        copyright,
        seo,
        count
    };
    HtmlTemplate {
        template
    }
}
pub async fn life_page(extract::Path(page): extract::Path<u64>) -> Html<String> {
    let page_size = 10;
    let articles =  life_list(page_size,page).await;
    let render =  LifePageTemplate{
        article:articles
    };
    let rhtml = render.render().unwrap_or("".to_string());
     Html(rhtml)
}
pub async fn life_list(page_size: i64, page: u64,) -> Vec<ArticleListRenderItem> {
    let mut results: Vec<ArticleListRenderItem> = Vec::new();
    match search_article_db(None, page, page_size, Some("Life".to_string()),None).await {
        Ok(r) => {
            let mut  i = 0;
            for x in r {
                i+=1;
                let mut odd = true;
                if i%2==0 {
                    odd = false;
                }
               results.push(article_to_article_list_render_item(x,odd));
            }
        }
        Err(_e) => {}
    };
    results
}