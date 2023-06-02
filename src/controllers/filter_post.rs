use std::error::Error;
use std::sync::Arc;
use askama::Template;
use axum::{
    http::StatusCode, routing::{get, Router},
    response::{Html, IntoResponse},
    extract::{State, Path},
};
use crate::model::models::{get_filtered_from_database, get_filtered_from_database_by_category};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use sqlx::postgres::PgPoolOptions;
use crate::{Blog, BlogTemplate};
use crate::controllers::filter_navigate::blog_pagination;

pub async fn blogs(Path(category): Path<String>) -> impl IntoResponse {
    println!("category {} page number", category);
    let mut psec: Vec<String> = Vec::new();
    psec.clear();
    psec.push("Category A".to_string());
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());
    let mut number_of_pages = 0;
    let mut plinks: Vec<String> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    let mut pids: Vec<i32> = Vec::new();
    let mut string_a: String = category.clone().to_owned();
    let mut string_b: &str = "/pages";
    let mut current_url = string_a + string_b;
    println!("current url {}",current_url);
    let mut posts2 = get_filtered_from_database_by_category(category).await;

    for post in &mut posts2 {
        post.post_title = post.post_title.replace("-", " ");
    }

    let shared_state2 = Arc::new(posts2);
    println!("len {}",shared_state2.len());

    //number_of_pages = shared_state2.len();
    if shared_state2.len()%3==0 {
        number_of_pages = shared_state2.len()/3;
    }
    // else if shared_state2.len() == 1{
    //     number_of_pages = 1;
    // }
    else{

        number_of_pages = (shared_state2.len()/3)+1;
    }
    println!("total{} number of pages {}",shared_state2.len(),number_of_pages);
    for i in 1 .. number_of_pages+1 {
        pnav.push(i.to_string())
    }

    if shared_state2.len() >= 3 {
        for i in 0 .. 3 {
            plinks.push(shared_state2[i].post_title.clone());
            pids.push(shared_state2[i].post_id.clone());
        }
    }
    else{
        for i in 0 .. shared_state2.len() {
            plinks.push(shared_state2[i].post_title.clone());
            pids.push(shared_state2[i].post_id.clone());
        }
    }

    let template = BlogTemplate{ index_id: &vec![], index_title: String::from("Blogs"), index_links: &plinks, index_sec: &psec, page_nav_links: &pnav,current_url_page: current_url};

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        ).into_response(),
    }
}
