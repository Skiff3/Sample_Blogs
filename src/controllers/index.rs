use crate::controllers::posts_crud_controller::get_vec_len_of_count;
use crate::global_number_of_items_per_page_64;
use crate::model::models::{IndexTemplate};
use askama::Template;
use axum::response::IntoResponse;
use axum::{http::StatusCode, response::Html};
use std::collections::{BTreeMap, HashMap};
use axum_macros::debug_handler;
use std::string::String;
use crate::controllers::base_controller::{get_all_categories, get_connection, get_count_of_posts};

#[debug_handler]
pub async fn index() -> impl IntoResponse {
    let mut category_in_template: Vec<String> = vec![];
    let mut count_of_posts: i64 = 0;
    let mut post_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id, category.category_name.clone());
            category_in_template.push(category.clone().category_name);
        })
    });
    let posts = get_connection().await.unwrap();
    let mut page_numbers_in_navigation: Vec<i32> = vec![];
    let number_of_posts_vector = get_count_of_posts().await;
    count_of_posts = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (count_of_posts + 2) / global_number_of_items_per_page_64();
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|i| page_numbers_in_navigation.push(i as i32));
    posts.iter().for_each(|post| {
        post_id_with_title.insert(post.post_id, post.post_title.clone());
    });
    let post_title_in_template = posts.iter().map(|post| post.post_title.clone()).collect();
    let post_id_in_template = posts.iter().map(|post1| post1.post_id.clone()).collect();
    let template = IndexTemplate {
        post_id_title: post_id_with_title,
        category_id_title: category_id_with_title,
        index_id: &post_id_in_template,
        index_title: String::from("Posts"),
        page_number: &1,
        selected_category: &"Not Selected".to_string(),
        index_links: &post_title_in_template,
        index_sec: &category_in_template,
        page_nav_links: &page_numbers_in_navigation,
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}
