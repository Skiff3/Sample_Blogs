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
    let mut plinks: Vec<String> = vec![];
    let mut pids: Vec<i32> = vec![];
    let final_category = &category[0..category.len()];
    let mut psec: Vec<String> = vec![];
    let mut pnav: Vec<String> = vec![];
    psec.clear();
    let psec = vec![
        "Category A".to_string(),
        "Category B".to_string(),
        "Category C".to_string(),
        "No Category".to_string(),
    ];

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

    shared_state2.as_ref().iter().for_each(|posts| {
        posts
            .iter()
            .for_each(|post| plinks.push(post.post_title.clone()));

        posts
            .iter()
            .for_each(|post| pids.push(post.post_id.clone()));
    });

    let template = BlogTemplate {
        index_id: &pids,
        index_title: String::from("Posts"),
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
        current_url_page: ".".to_string(),
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn blog_pagination(
    Path((category, page_number)): Path<(String, String)>,
) -> impl IntoResponse {
    let mut plinks: Vec<String> = vec![];
    let mut pids: Vec<i32> = vec![];
    let final_category = &category[0..category.len()];
    let mut psec: Vec<String> = vec![];
    let mut pnav: Vec<String> = vec![];
    psec.clear();
    let psec = vec![
        "Category A".to_string(),
        "Category B".to_string(),
        "Category C".to_string(),
        "No Category".to_string(),
    ];

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

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}
