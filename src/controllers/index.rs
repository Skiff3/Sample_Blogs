use crate::global_number_of_items_per_page_64;
use crate::model::models::{get_all_categories, get_count_of_posts, IndexTemplate, Post};
use askama::Template;
use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode, response::Html};
use std::sync::Arc;
use sqlx::Error;

pub async fn index(State(state): State<Arc<Result<Vec<Post>,Error>>>) -> impl IntoResponse {
    let mut psec: Vec<String> = Vec::new();
    psec.clear();

    let category_list = get_all_categories().await?;
    let list_iter = category_list.iter();
    for i in list_iter {
        psec.push(i.clone().category_name);
    }
    let s = state.clone();
    //let number_of_pages: i64;
    let mut plinks: Vec<String> = Vec::new();
    let mut pids: Vec<i32> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    let number_of_posts_vector = get_count_of_posts().await;
    let m = number_of_posts_vector;
    let number_of_pages: i64 = if m[0].count % global_number_of_items_per_page_64() == 0 {
        (m[0].count) / global_number_of_items_per_page_64()
    } else {
        (m[0].count) / global_number_of_items_per_page_64() + 1
    };
    println!(
        "the number of pages are {}, count of posts {}",
        number_of_pages, m[0].count
    );
    for i in 1..number_of_pages + 1 {
        pnav.push(i.to_string())
    }
    let list_iter = s.map(|posts| {
        //plinks = posts.iter()
            //.map(|post| {post.post_title.clone()}).collect();
        let v: Vec<_> = posts.iter()
            .map(|post| {post.post_title.clone()}).collect();
        let v2: Vec<_> = posts.iter()
            .map(|post| {post.post_id.clone()}).collect();

        (v,v2)
    });

    let (plinks, pids) = list_iter.unwrap_or_default();
    // let t = list_iter.unwrap_or_default();
    // let plinks = t.0;
    // let pids = t.1;

    let template = IndexTemplate {
        index_id: &pids,
        index_title: String::from("Blogs"),
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}
