use crate::controllers::posts_crud_controller::get_vec_len_of_count;
use crate::model::models::{
  Blog, HomeFilterTemplate,
};
use crate::{global_number_of_items_per_page_64, BlogTemplate};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;
use crate::controllers::base_controller::{count_filtered_cat, get_all_categories, category_by_id, get_filtered_cat, count_of_postsdb, filtered_cat};

pub async fn admin_blogs(Path(category): Path<i32>) -> impl IntoResponse {
    let mut category_in_template: Vec<String> = vec![];
    let mut posts: Vec<Blog> = vec![];
    let category_name = category_by_id(category)
        .await
        .first()
        .unwrap()
        .clone()
        .category_name;
    let mut post_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id, category.category_name.clone());
            category_in_template.push(category.clone().category_name);
        })
    });// todo add flatten() here
    let mut page_numbers_in_navigation: Vec<i32> = vec![];
    let string_a: String = category.clone().to_string();
    let string_b: &str = "/pages";
    let number_of_posts_vector;
    let current_url = string_a + string_b;
    if category != 0 {
        posts = get_filtered_cat(category.clone())
            .await
            .unwrap();
    } else {
        posts = filtered_cat().await.unwrap();
    }
    if category != 0 {
        number_of_posts_vector = count_filtered_cat(category).await;
    } else {
        number_of_posts_vector = count_of_postsdb().await;
    }

    let count_of_posts = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (count_of_posts + 2) / global_number_of_items_per_page_64();
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|index| page_numbers_in_navigation.push(index as i32));
    posts.iter().for_each(|post| {
        post_id_with_title.insert(post.post_id, post.post_title.clone());
    }); // todo add flatten() here
    let post_title_in_template = posts.iter().map(|post| post.post_title.clone()).collect();
    let post_id_in_template = posts.iter().map(|post| post.post_id.clone()).collect();
    let template = BlogTemplate {
        post_id_title: post_id_with_title,
        category_id_title: category_id_with_title,
        index_id: &post_id_in_template,
        index_title: String::from("Posts"),
        page_number: &1,
        category_name: &category_name,
        index_links: &post_title_in_template,
        index_sec: &category_in_template,
        page_nav_links: &page_numbers_in_navigation,
        current_url_page: current_url,
    };
    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn blogs(Path(category): Path<i32>) -> impl IntoResponse {
    let mut post_id_in_template: Vec<i32> = vec![];
    let mut category_in_template: Vec<String> = vec![];
    let tmp = category_by_id(category).await;
    let mut category_name = tmp.first().unwrap().clone().category_name;
    category_in_template.clear();
    let category_list = get_all_categories().await;
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id, category.category_name.clone());
            category_in_template.push(category.clone().category_name);
        })
    });// todo add flatten() here
    let mut page_numbers_in_navigation: Vec<i32> = vec![];
    let string_a: String = category.clone().to_string();
    let string_b: &str = "/pages";
    let current_url = string_a + string_b;
    let posts = get_filtered_cat(category.clone())
        .await
        .unwrap();
    let category_for_ref = category.clone();
    let mut post_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let number_of_posts_vector =
        count_filtered_cat(category_for_ref).await;
    let count_of_posts = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (count_of_posts + 2) / global_number_of_items_per_page_64();
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|i| page_numbers_in_navigation.push(i as i32));
    posts.iter().for_each(|post| {
        post_id_with_title.insert(post.post_id, post.post_title.clone());
    });// add flatten() here
    let post_title_in_template = posts.iter().map(|post| post.post_title.clone()).collect();
    post_id_in_template = posts.iter().map(|post1| post1.post_id.clone()).collect();

    let template = HomeFilterTemplate {
        post_id_title: post_id_with_title,
        category_id_title: category_id_with_title,
        index_id: &post_id_in_template,
        index_title: String::from("Posts"),
        page_number: &1,
        category_name: &category_name,
        index_links: &post_title_in_template,
        index_sec: &category_in_template,
        page_nav_links: &page_numbers_in_navigation,
        current_url_page: current_url,
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}
