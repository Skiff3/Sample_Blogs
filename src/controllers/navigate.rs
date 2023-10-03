use crate::controllers::posts_crud_controller::get_vec_len_of_count;
use crate::model::models::{HomeTemplate};
use crate::{global_number_of_items_per_page, global_number_of_items_per_page_64, IndexTemplate};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use do_paginate::Pages;
use std::collections::{BTreeMap, HashMap};
use crate::controllers::base_controller::{get_all_categories, total_posts, posts_per_page};

pub async fn page(Path(page_number): Path<i32>) -> impl IntoResponse {
    let mut category_in_template: Vec<String> = vec![];
    let mut page_numbers_in_navigation: Vec<i32> = vec![];
    let mut post_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    category_in_template.clear();
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id, category.category_name.clone());
            category_in_template.push(category.clone().category_name);
        })
    });
    let pages: Pages = Pages::new(
        get_vec_len_of_count(total_posts().await)
            .try_into()
            .unwrap(),
        global_number_of_items_per_page() as usize,
    );
    let page = pages.to_page_number(page_number as usize);
    let mut no_of_pages = page.unwrap_or_default();
    let posts = posts_per_page(no_of_pages.begin as i32).await.unwrap();
    let number_of_posts_vector = total_posts().await;
    let global_no_of_posts_per_page = number_of_posts_vector;
    let number_of_pages: i64 = if get_vec_len_of_count(global_no_of_posts_per_page)
        % global_number_of_items_per_page_64()
        == 0
    {
        get_vec_len_of_count(total_posts().await) / global_number_of_items_per_page_64()
    } else {
        get_vec_len_of_count(total_posts().await) / global_number_of_items_per_page_64() + 1
    };

    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|index| page_numbers_in_navigation.push(index as i32));
    posts.iter().for_each(|post| {
        post_id_with_title.insert(post.post_id, post.post_title.clone());
    });
    let post_title_in_template = posts.iter().map(|post| post.post_title.clone()).collect();
    let post_ids_in_template = posts.iter().map(|post1| post1.post_id.clone()).collect();
    let template = IndexTemplate {
        post_id_title: post_id_with_title,
        category_id_title: category_id_with_title,
        index_id: &post_ids_in_template,
        index_title: String::from("Posts"),
        page_number: &page_number,
        selected_category: &"Category A".to_string(),
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

pub async fn pages(Path(page_number): Path<i32>) -> impl IntoResponse {
    let mut category_in_template: Vec<String> = vec![];
    let mut post_id_in_template: Vec<i32> = vec![];
    let mut page_numbers_in_navigation: Vec<i32> = vec![];
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    category_in_template.clear();
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id, category.category_name.clone());
            category_in_template.push(category.clone().category_name);
        })
    });
    let mut post_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let page_number_integer: i32 = page_number;
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();
    let posts = posts_per_page(offset_start).await.unwrap();
    let number_of_pages: i64 = if get_vec_len_of_count(total_posts().await)
        % global_number_of_items_per_page_64()
        == 0
    {
        get_vec_len_of_count(total_posts().await) / global_number_of_items_per_page_64()
    } else {
        get_vec_len_of_count(total_posts().await) / global_number_of_items_per_page_64() + 1
    };
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|index| page_numbers_in_navigation.push(index as i32));
    post_id_in_template.clear();
    posts.iter().for_each(|post| {
        post_id_with_title.insert(post.post_id, post.post_title.clone());
    });
    let post_title_in_template = posts.iter().map(|post| post.post_title.clone()).collect();
    post_id_in_template = posts.iter().map(|post1| post1.post_id.clone()).collect();
    let template = HomeTemplate {
        post_id_title: post_id_with_title,
        category_id_title: category_id_with_title,
        index_id: &post_id_in_template,
        index_title: String::from("Posts"),
        page_number: &page_number,
        index_links: &post_title_in_template,
        index_sec: &category_in_template,
        page_nav_links: &page_numbers_in_navigation,
        current_url_page: ".".to_string(),
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}
