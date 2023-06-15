use crate::model::models::{get_filtered_from_database, HomeTemplate};
use crate::{global_number_of_items_per_page, BlogTemplate};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::sync::Arc;

use crate::controllers::posts_crud_controller::get_vec_len;

pub async fn admin_blog_pagination(
    Path((category, page_number)): Path<(String, String)>,
) -> impl IntoResponse {
    let mut plinks: Vec<String> = Vec::new();
    let mut pids: Vec<i32> = Vec::new(); // number_of_pages.
    let final_category = &category[0..category.len()];
    let mut psec: Vec<String> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    //let mut check_category:String = category;
    psec.clear(); // psec.clear()
    psec.push("Category A".to_string());
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("Category D".to_string());

    let page_number_integer: i32 = page_number.parse().unwrap();
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();
    println!("page starts from {}", offset_start);

    let posts2 = get_filtered_from_database(final_category.to_string(), offset_start).await;

    let shared_state2 = Arc::new(posts2);
    let number_of_pages = 0;
    (1..number_of_pages)
        .into_iter()
        .for_each(|i| pnav.push(i.to_string()));
    let _tmp2 = shared_state2.as_ref();
    let list_iter = shared_state2.iter().map(|posts| {
        let mut _v: Vec<_> = vec![];
        _v = posts
            .iter()
            .map(|post| plinks.push(post.post_title.clone()))
            .collect();
        let mut _v1: Vec<_> = vec![];
        _v1 = posts
            .iter()
            .map(|post| pids.push(post.post_id.clone()))
            .collect();
        (_v, _v1)
    });

    let list_iter = shared_state2.as_ref().iter().map(|posts| {
        let _v: Vec<_> = posts
            .iter()
            .map(|post| plinks.push(post.post_title.clone()))
            .collect();
        let _v2: Vec<_> = posts
            .iter()
            .map(|post| pids.push(post.post_id.clone()))
            .collect();
    });

    let template = BlogTemplate {
        index_id: &pids,
        index_title: String::from("Posts"),
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
        current_url_page: ".".to_string(), // change the format
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

pub async fn blog_pagination(
    Path((category, page_number)): Path<(String, String)>,
) -> impl IntoResponse {
    let mut plinks: Vec<String> = Vec::new();
    let mut pids: Vec<i32> = Vec::new();
    let final_category = &category[0..category.len()];
    let mut psec: Vec<String> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    psec.clear(); // psec.clear()
    psec.push("Category A".to_string());
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("Category D".to_string());

    let page_number_integer: i32 = page_number.parse().unwrap();
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();

    let posts2 = get_filtered_from_database(final_category.to_string(), offset_start).await;

    let shared_state2 = Arc::new(posts2);
    let number_of_pages = get_vec_len(shared_state2.clone());

    (1..number_of_pages)
        .into_iter()
        .for_each(|i| pnav.push(i.to_string()));
    let temp = shared_state2.as_ref().as_ref();
    let list_iter = temp.map(|posts| {
        let v: Vec<_> = posts.iter().map(|post| post.post_title.clone()).collect();
        let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();

        (v, v2)
    });

    (plinks, pids) = list_iter.unwrap_or_default();

    let template = HomeTemplate {
        index_id: &pids,
        index_title: String::from("Posts"),
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
        current_url_page: ".".to_string(),
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
