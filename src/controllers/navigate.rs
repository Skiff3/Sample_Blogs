use std::sync::Arc;
use askama::Template;
use axum::{
    http::StatusCode, routing::{get, Router},
    response::{Html, IntoResponse},
    extract::{State, Path},
};
use crate::{global_number_of_items_per_page, global_number_of_items_per_page_64, IndexTemplate, Post};
use crate::model::models::{get_count_of_posts, get_posts_per_page};

pub async fn page(Path(page_number): Path<String> , State(state): State<Arc<Vec<Post>>>) -> impl IntoResponse {
    println!("{}", page_number);
    let mut plinks: Vec<String> = Vec::new();
    let mut psec: Vec<String> = Vec::new();
    let mut pid: Vec<i32> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();// let mut pnav: Vec<String> = Vec::new();
    psec.clear();
    //let mut psec: Vec<String> = Vec::new();
    psec.push("Category A".to_string());// psec.push("Category A")
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());

    let mut page_number_integer = 0;
    let mut number_of_pages: i64 = 0;
    page_number_integer = page_number.parse().unwrap();
    let mut offset_start: i32 = 0;
    offset_start = (page_number_integer-1) * global_number_of_items_per_page();// offset value.
    println!("page starts from {}", offset_start);
    let s = get_posts_per_page(offset_start).await;
    let mut number_of_posts_vector = get_count_of_posts().await;
    let m = number_of_posts_vector.clone();
    if m[0].count % global_number_of_items_per_page_64() ==0 {
        number_of_pages = ((m[0].count)/global_number_of_items_per_page_64());
    }
    else {// dashboard, routing paths
        number_of_pages = ((m[0].count)/global_number_of_items_per_page_64())+1;
    }
    println!("the number of pages are {} count of posts {}",number_of_pages,m[0].count);
    for i in 1 .. number_of_pages+1 {
        pnav.push(i.to_string())
    }

        plinks.clear();
        for i in 0 .. s.len() {
            plinks.push(s[i].post_title.clone());
            pid.push(s[i].post_id.clone());
        } //

    let template = IndexTemplate{ index_id: &pid, index_title: String::from("Blogs"), index_links: &plinks, index_sec: &psec, page_nav_links: &pnav};

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        ).into_response(),//
    }
}