use std::convert::Infallible;
use axum::handler::Handler;
use axum::routing::{
    get, post,get_service
};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Router};
use crate::controller::user;
use crate::controller::tag;
use crate::controller::article;
use crate::controller::copyright;
use crate::controller::render;
use tower_http::{services::ServeDir};
use crate::middleware::cors::CorsLayer;
use crate::{ GLOBAL_CONF};
pub fn route_info() -> Router {

    Router::new()
            .nest("/user", user(), )
            .nest("/tag", tag(), )
            .nest("/article", article(), )
            .nest("/setting", setting(), )
            .nest("/", render_view(), )
            .nest(
                "/static",
                get_service(ServeDir::new(GLOBAL_CONF.server.web_path.as_ref().unwrap() )).handle_error(|error: std::io::Error| async move{
                    Ok::<_, Infallible>((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    ))
                }),
            )
        .fallback(handler_404.into_service())
        .layer(CorsLayer{})

}




pub fn render_view() -> Router {
    Router::new().
        route("/", get(render::blog))
        .route("/blog", get(render::blog))
        .route("/life", get(render::life))
        .route("/life_page/:page", get(render::life_page))
        .route("/about", get(render::about))
        .route("/tool", get(render::tool))
        .route("/article/:id", get(render::article_detail))
        
}

pub fn user() -> Router {
    Router::new().
        route("/login", post(user::login)).
        route("/updateUser", post(user::update_user)).
        route("/cosKey", post(user::cos_key)).
        route("/test", post(user::test))
}

pub fn setting() -> Router {
    Router::new()
        .route("/get", post(copyright::get_copyright))
        .route("/save", post(copyright::save_copyright_content))
}
pub fn article() -> Router {
    Router::new()
        .route("/setArticle", post(article::set_article))
        .route("/searchArticle", post(article::search_article))
        .route("/findArticle", post(article::find_article))
        .route("/delArticle", post(article::del_article))
}
pub fn tag() -> Router {
    Router::new()
        .route("/getAllTags", get(tag::list_tags))
        .route("/add",post(tag::add_tag))
        .route("/remove", post(tag::del_tag))
        .route("/search", post(tag::search_tag))
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

