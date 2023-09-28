use std::collections::{BTreeMap, HashMap};
use crate::model::models::{count_of_get_filtered_from_database_by_category, get_all_categories, get_category_name_by_id, get_count_of_posts, get_filtered_from_database, HomeFilterTemplate};
use crate::{global_number_of_items_per_page, global_number_of_items_per_page_64, BlogTemplate};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::sync::Arc;
use crate::controllers::posts_crud_controller::get_vec_len_of_count;


pub async fn admin_blog_pagination(
    Path((category_in_url, page_number)): Path<(i32, i32)>,
) -> impl IntoResponse {
    let mut post_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let category_id = category_in_url;
    let mut category_in_template: Vec<String> = vec![];
    let mut page_navigation_numbers: Vec<i32> = vec![];
    let tmp = get_category_name_by_id(category_in_url).await;
    let category_name_iter = tmp.iter();
    let mut category_name = "".to_string();
    for index in category_name_iter {
        category_name = index.category_name.clone();
    }
    category_in_template.clear();
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            category_in_template.push(category.clone().category_name);
        })
    });

    let page_number_integer: i32 = page_number;
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();

    let posts = get_filtered_from_database(category_id.clone(), offset_start).await.unwrap();
    let number_of_posts_vector =
        count_of_get_filtered_from_database_by_category(category_id.clone()).await;
    let count_of_posts = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (count_of_posts + 2) / global_number_of_items_per_page_64();
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|index| page_navigation_numbers.push(index as i32));
    //posts.iter().for_each(|post| {post_id_with_title.insert(post.post_id, post.post_title.clone());});
    let post_id_with_title = posts.iter().map(|post| (post.post_id, post.post_title.clone())).collect::<BTreeMap<_, _>>();
    let post_name_in_template = posts.iter().map(|post| post.post_title.clone()).collect();
    let post_id_in_template = posts.iter().map(|post| post.post_id.clone()).collect();
    let template = BlogTemplate {
        post_id_title:post_id_with_title,
        category_id_title: category_id_with_title,
        index_id: &post_id_in_template,
        index_title: String::from("Posts"),
        page_number: &page_number,
        category_name:&category_name,
        index_links: &post_name_in_template,
        index_sec: &category_in_template,
        page_nav_links: &page_navigation_numbers,
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
    Path((category_in_url, page_number)): Path<(i32, i32)>,
) -> impl IntoResponse {
    let mut post_id_in_template: Vec<i32> = vec![];
    let mut post_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let category = category_in_url;
    let mut category_in_template: Vec<String> = vec![];
    let mut page_navigation_numbers: Vec<i32> = vec![];
    let tmp = get_category_name_by_id(category_in_url).await;
    let category_name_iter = tmp.iter();
    let mut category_name = "".to_string();
    for index in category_name_iter {
        category_name = index.category_name.clone();
    }
    category_in_template.clear();
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            category_in_template.push(category.clone().category_name);
        })
    });

    let page_number_integer: i32 = page_number;
    let _offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();
    let posts = get_filtered_from_database(category.clone(), 0).await.unwrap();

    let number_of_posts_vector =
        count_of_get_filtered_from_database_by_category(category).await;
    let count_of_posts = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = count_of_posts as i64;
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|index| page_navigation_numbers.push(index as i32));
    posts.clone().iter().for_each(|post| {post_id_with_title.insert(post.post_id, post.post_title.clone());});
    let post_name_in_template = posts.iter().map(|post| post.post_title.clone()).collect();
    post_id_in_template = posts.iter().map(|post1| post1.post_id.clone()).collect();
    let template = HomeFilterTemplate {
        post_id_title: post_id_with_title,
        category_id_title: category_id_with_title,
        index_id: &post_id_in_template,
        index_title: String::from("Posts"),
        page_number: &page_number,
        category_name:&category_name,
        index_links: &post_name_in_template,
        index_sec: &category_in_template,
        page_nav_links: &page_navigation_numbers,
        current_url_page: ".".to_string(),
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}
