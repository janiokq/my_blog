use askama::Template;
use axum::{response::IntoResponse, response};
use std::{convert::Infallible};
use axum::body::{Bytes, Full};
use axum::http::{Response, StatusCode};

pub struct ArticleRenderTag {
    pub id: String,
    pub name: String,
}
pub struct ArticleListRenderItem {
    pub id: String,
    pub title: String,
    pub describe: String,
    pub cover_pic: String,
    pub tags: Vec<ArticleRenderTag>,
    pub create_time: String,
    pub update_time: String,
    pub odd_number: bool,
}
pub struct TagRender {
    pub name: String,
    pub select: bool,
}
pub struct FriendshipLink {
    pub name: String,
    pub url: String,
}

#[derive(Template)]
#[template(path = "blog.html")]
pub struct BlogTemplate {
    pub title: String,
    pub page: String,
    pub page_list: Vec<u64>,
    pub next: String,
    pub previous: String,
    pub article_lists: Vec<ArticleListRenderItem>,
    pub tags: Vec<TagRender>,
    pub search_key: String,
    pub tag: String,
    pub copyright: String,
    pub seo: String,
    pub link: Vec<FriendshipLink>,
}



#[derive(Template)]
#[template(path = "article.html")]
pub struct ArticleTemplate {
    pub title: String,
    pub content: String,
    pub copyright: String,
    pub seo: String,
    pub recommend_article: Vec<ArticleListRenderItem>,
}


#[derive(Template)]
#[template(path = "about.html")]
pub struct AboutTemplate {
    pub title: String,
    pub content: String,
    pub seo: String,
}

#[derive(Template)]
#[template(path = "tool.html")]
pub struct ToolTemplate {
    pub title: String,
    pub content: String,
    pub seo: String,
}


#[derive(Template)]
#[template(path = "Life.html")]
pub struct LifeTemplate {
    pub title: String,
    pub copyright: String,
    pub seo: String,
    pub count: u64,
    pub article: Vec<ArticleListRenderItem>,
}

#[derive(Template)]
#[template(path = "LifePage.html")]
pub struct LifePageTemplate {
    pub article: Vec<ArticleListRenderItem>,
}


#[derive(Template)]
#[template(path = "404.html")]
pub struct Empty404Template {
}



pub struct HtmlTemplate<T> {
    pub template: T,
}
impl<T> IntoResponse for HtmlTemplate<T>
    where
        T: Template,
{
    type Body = Full<Bytes>;
    type BodyError = Infallible;
    fn into_response(self) -> Response<Self::Body> {
        match self.template.render() {
            Ok(html) => response::Html(html).into_response(),
            Err(err) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Full::from(format!(
                    "Failed to render template. Error: {}",
                    err
                )))
                .unwrap(),
        }
    }
}
