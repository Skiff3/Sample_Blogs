use crate::model::models::{get_count_of_posts, get_posts_per_page};
use crate::{
    global_number_of_items_per_page, global_number_of_items_per_page_64, IndexTemplate, Post,
};
use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::sync::Arc;

pub async fn page(
    Path(page_number): Path<String>,
    State(_state): State<Arc<Vec<Post>>>,
) -> impl IntoResponse {
    println!("{}", page_number);
    let mut plinks: Vec<String> = Vec::new();
    let mut psec: Vec<String> = Vec::new();
    let mut pid: Vec<i32> = Vec::new();
    let mut pnav: Vec<String> = Vec::new(); // let mut pnav: Vec<String> = Vec::new();
    psec.clear();
    //let mut psec: Vec<String> = Vec::new();
    psec.push("Category A".to_string()); // psec.push("Category A")
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());

    // let page_number_integer:i32;
    // let number_of_pages: i64;
    let page_number_integer : i32 = page_number.parse().unwrap();
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page(); // offset value.
    println!("page starts from {}", offset_start);
    let s = get_posts_per_page(offset_start).await;
    let number_of_posts_vector = get_count_of_posts().await;
    let m = number_of_posts_vector;
    let number_of_pages: i64 = if m[0].count % global_number_of_items_per_page_64() == 0 {
        (m[0].count) / global_number_of_items_per_page_64()
    } else {
        (m[0].count) / global_number_of_items_per_page_64() + 1
    };
    println!(
        "the number of pages are {} count of posts {}",
        number_of_pages, m[0].count
    );
    for i in 1..number_of_pages + 1 {
        pnav.push(i.to_string())
    }

    plinks.clear();
    let list_iter = s.iter();
    for i in list_iter{
        plinks.push(i.post_title.clone());
        pid.push(i.post_id);
    } //

    let template = IndexTemplate {
        index_id: &pid,
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
            .into_response(), //
    }
}
