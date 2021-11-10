use std::convert::Infallible;
use axum::handler::{get, Handler,post};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{Router, service};
use axum::routing::BoxRoute;
use crate::controller::user;
use crate::controller::tag;
use crate::controller::article;
use crate::controller::copyright;
use crate::controller::render;
use tower_http::{services::ServeDir};
use crate::middleware::cors::CorsLayer;
use crate::{ GLOBAL_CONF};
pub fn route_info() -> Router<BoxRoute> {

    Router::new()
            .nest("/user", user(), )
            .nest("/tag", tag(), )
            .nest("/article", article(), )
            .nest("/setting", setting(), )
            .nest("/", render_view(), )
            .nest(
                "/static",
                service::get(ServeDir::new(GLOBAL_CONF.server.web_path.as_ref().unwrap() )).handle_error(|error: std::io::Error| {
                    Ok::<_, Infallible>((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled internal error: {}", error),
                    ))
                }),
            )
        .or(handler_404.into_service())
        .layer(CorsLayer{})
        .boxed()

}




pub fn render_view() -> Router<BoxRoute> {
    Router::new().
        route("/", get(render::blog))
        .route("/blog", get(render::blog))
        .route("/life", get(render::life))
        .route("/life_page/:page", get(render::life_page))
        .route("/about", get(render::about))
        .route("/tool", get(render::tool))
        .route("/article/:id", get(render::article_detail))
        .boxed()
}

pub fn user() -> Router<BoxRoute> {
    Router::new().
        route("/login", post(user::login)).
        route("/updateUser", post(user::update_user)).
        route("/cosKey", post(user::cos_key)).
        route("/test", post(user::test))
        .boxed()
}

pub fn setting() -> Router<BoxRoute> {
    Router::new()
        .route("/get", post(copyright::get_copyright))
        .route("/save", post(copyright::save_copyright_content))
        .boxed()
}
pub fn article() -> Router<BoxRoute> {
    Router::new()
        .route("/setArticle", post(article::set_article))
        .route("/searchArticle", post(article::search_article))
        .route("/findArticle", post(article::find_article))
        .route("/delArticle", post(article::del_article))
        .boxed()
}
pub fn tag() -> Router<BoxRoute> {
    Router::new()
        .route("/getAllTags", get(tag::list_tags))
        .route("/add",post(tag::add_tag))
        .route("/remove", post(tag::del_tag))
        .route("/search", post(tag::search_tag))
        .boxed()
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

