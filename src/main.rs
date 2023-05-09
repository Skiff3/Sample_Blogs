mod model;
mod controllers;
use crate::controllers::filter_navigate::blog_pagination;
use crate::controllers::navigate::page;
use crate::model::models::{BlogTemplate,IndexTemplate,Post,Blog,PostTemplate,get_connection};
use crate::controllers::post::{show_post};
use crate::controllers::index::index;
use crate::controllers::filter_post::blogs;
use tower_http::services::ServeDir;
use std::sync::Arc;
use askama::Template;
use core::fmt::Write;
use axum::http::Request;
use axum::Router;
use axum::extract::OriginalUri;
use axum::routing::get;


#[tokio::main]
async fn main() {
    let mut posts = get_connection().await;
    let shared_state = Arc::new(posts);

    let blog_routes = Router::new()
        .route("/posts/category/:category",get(blogs))
        .route("/posts/category/:category/pages/:page_number", get(blog_pagination));

    let app = Router::new()
        .route("/", get(index))
        .route("/post/:post_id", get(show_post))
        .route("/page/:page_number", get(page))
        .with_state(shared_state)
        .merge(blog_routes)
        .nest_service("/assets", ServeDir::new("assets"));

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}
