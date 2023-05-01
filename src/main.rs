//mod page_navigation;
// mod filter_posts;
// mod connections;
// mod filter_blogs_pagination;
mod model;
mod controllers;

use crate::controllers::filter_navigate::blog_pagination;
use crate::controllers::pg_connect::connect_to_pg;
use crate::controllers::navigate::page;
use crate::model::models::{BlogTemplate,IndexTemplate,Post,Blog,PostTemplate};
use crate::controllers::filter_post::blogs;
use axum::{
    http::StatusCode, routing::{get, Router},
    response::{Html, IntoResponse},
    extract::{State, Path},
};
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;
use sqlx::types::time::Date;
use tower_http::services::ServeDir;
use std::sync::Arc;
use askama::Template;
use paginator::{Paginator, PageItem};
use core::fmt::Write;




pub struct Pages {
    pub page_number: i32,
}


async fn post(Path(query_title): Path<String>, State(state): State<Arc<Vec<Post>>>) -> impl IntoResponse {
    println!("{}",query_title);
    let mut template = PostTemplate{post_title: "none", post_description: "", post_body: "none"};
    for i in 0..state.len() {
        if query_title == state[i].post_title {
            template = PostTemplate{post_title: &state[i].post_title,
                post_description: "",
                post_body: &state[i].post_body,
            };
            break;
        } else {
            continue
        }
    }


    if &template.post_title == &"none" {
        return (StatusCode::NOT_FOUND, "404 not found").into_response();
    }
    println!("{}",template);

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "try again later").into_response()
    }
}

async fn index(State(state): State<Arc<Vec<Post>>>) -> impl IntoResponse{
    let mut psec: Vec<String> = Vec::new();
    psec.clear();
    psec.push("Category A".to_string());
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());
    let number_of_pages = 5;
    let s = state.clone();
    let mut plinks: Vec<String> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    for i in 1 .. number_of_pages+1 {
        pnav.push(i.to_string())
    }

    for i in 0 .. 3 {
        plinks.push(s[i].post_title.clone());
        println!("{}",s.len())
    }

    let template = IndexTemplate{index_title: String::from("Sakib"), index_links: &plinks, index_sec: &psec, page_nav_links: &pnav};

    match template.render() {
            Ok(html) => Html(html).into_response(),
         Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error {}", err),
            ).into_response(),
    }
}

#[tokio::main]
async fn main() {

    let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
                .await// await
                .expect("couldn't connect to the database");

    let mut posts = sqlx::query_as::<_, Post>("select post_title, post_body, post_description from posts")
        .fetch_all(&pool)
        .await
        .unwrap();

    let shared_state = Arc::new(posts);
    //let shared_state2 = Arc::new(blogs);

    let app = Router::new()
        .route("/", get(index))
        .route("/post/:query_title", get(post))
        .route("/page/:query_title", get(page))
        .with_state(shared_state)
        .route("/blogs/:query_title",get(blogs))
        .route("/blogs/page/:query_title", get(blog_pagination))
        //.route("/category/:category/page/:page", get(blog_pagination))
        .nest_service("/assets", ServeDir::new("assets"));//nest service


    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}
