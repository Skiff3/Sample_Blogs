use std::sync::Arc;
use askama::Template;
use axum::{
    http::StatusCode, routing::{get, Router},
    response::{Html, IntoResponse},
    extract::{State, Path},
};
use sqlx::postgres::PgPoolOptions;
use crate::model::models::get_filtered_from_database;
use crate::{Blog, BlogTemplate, global_number_of_items_per_page, IndexTemplate, Post};
use crate::controllers::navigate::page;

pub async fn blog_pagination(Path((category, page_number)): Path<(String,String)>) -> impl IntoResponse {
    let mut plinks: Vec<String> = Vec::new();
    let mut pids: Vec<i32> = Vec::new();
    let mut number_of_pages = 0;// number_of_pages.
    let mut check_category = 0;
    let final_category = &category[0..category.len()];
    println!("category {}",final_category.clone());
    let mut psec: Vec<String> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    //let mut check_category:String = category;
    psec.clear();// psec.clear()
    psec.push("Category A".to_string());
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());// auth steps: html, database(user_db), controller() -> link >
    psec.push("No Category".to_string());

    let mut page_number_integer = 0;
    page_number_integer = page_number.parse().unwrap();
    let mut offset_start: i32 = 0;
    offset_start = ((page_number_integer-1) * global_number_of_items_per_page());// offset value.
    println!("page starts from {}", offset_start);

    let mut posts2 = get_filtered_from_database(final_category.to_string(), offset_start.clone()).await;

    for post in &mut posts2 {
        post.post_title = post.post_title.replace("-", " ");
    }

    let shared_state2 = Arc::new(posts2);
    number_of_pages = shared_state2.len();
    // if shared_state2.len()%3==0 {
    //     number_of_pages = shared_state2.len()/3;
    // }
    // else{
    //     number_of_pages = (shared_state2.len()/3)+1;
    // }
    println!("total{} number of pages {}",shared_state2.len(),number_of_pages);
    for i in 1 .. number_of_pages+1 {
        pnav.push(i.to_string())
    }
    for i in 0 .. shared_state2.len() {
        plinks.push(shared_state2[i].post_title.clone());
        pids.push(shared_state2[i].post_id.clone());
    }
        pids.clear();
        plinks.clear();
    for i in 0 .. shared_state2.len() {
            plinks.push(shared_state2[i].post_title.clone());
            pids.push(shared_state2[i].post_id.clone());
        }


    let template = BlogTemplate{ index_id: &pids, index_title: String::from("Blogs"), index_links: &plinks, index_sec: &psec, page_nav_links: &pnav,current_url_page:".".to_string()};

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        ).into_response(),
    }
}