use std::collections::HashMap;
use crate::controllers::posts_crud_controller::get_vec_len_of_count;
use crate::global_number_of_items_per_page_64;
use crate::model::models::{get_all_categories, get_connection, get_count_of_posts, IndexTemplate};
use askama::Template;
use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Html};

use axum_macros::debug_handler;

#[debug_handler]
pub async fn index() -> impl IntoResponse {
    let mut psec: Vec<String> = vec![];
    let mut m2: i64 = 0;
    let mut post_id_with_title: HashMap<i32, String> = HashMap::new();
    let mut category_id_with_title: HashMap<i32, String> = HashMap::new();
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            psec.push(category.clone().category_name);
        })
    });
    let posts = get_connection().await.unwrap();
    let mut pnav: Vec<String> = vec![];
    let number_of_posts_vector = get_count_of_posts().await;
    m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (m2 + 2) / global_number_of_items_per_page_64();
    println!("count {} and number {}", m2, number_of_pages);
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|i| pnav.push(i.to_string()));
    posts.iter().for_each(|post| {post_id_with_title.insert(post.post_id,post.post_title.clone());});
    //let list_iter = s.iter().map()
    let plinks = posts.iter().map(|post| post.post_title.clone()).collect();
    let pids = posts.iter().map(|post1| post1.post_id.clone()).collect();
    //let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();
    //(plinks, pids) = list_iter.unwrap_or_default();
    println!("hashmap {:?}",post_id_with_title);

    let template = IndexTemplate {
        post_id_title:post_id_with_title,
        category_id_title:category_id_with_title,
        index_id: &pids,
        index_title: String::from("Posts"),
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}
