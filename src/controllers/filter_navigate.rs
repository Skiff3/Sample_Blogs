use std::collections::HashMap;
use crate::model::models::{
    count_of_get_filtered_from_database_by_category, get_all_categories,
    get_filtered_from_database, HomeFilterTemplate,
};
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
    Path((category, page_number)): Path<(i32, i32)>,
) -> impl IntoResponse {
    let mut plinks: Vec<String> = vec![];
    let mut pids: Vec<i32> = vec![];
    let mut len = 0;
    let mut post_id_with_title: HashMap<i32, String> = HashMap::new();
    let mut category_id_with_title: HashMap<i32, String> = HashMap::new();
    let final_category = category;
    let mut psec: Vec<String> = vec![];
    let mut pnav: Vec<i32> = vec![];
    psec.clear();
    let category_list = get_all_categories().await;
    let mut psec: Vec<String> = vec![];
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            psec.push(category.clone().category_name);
        })
    });

    let page_number_integer: i32 = page_number;
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();

    let posts2 = get_filtered_from_database(final_category.clone(), offset_start).await.unwrap();
    let number_of_posts_vector =
        count_of_get_filtered_from_database_by_category(final_category.clone()).await;
    println!("len {}", len);
    let m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (m2 + 2) / global_number_of_items_per_page_64();
    println!("number in filter navigate {}", number_of_pages);
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|i| pnav.push(i as i32));
    posts2.iter().for_each(|post| {post_id_with_title.insert(post.post_id,post.post_title.clone());});
    //let list_iter = s.iter().map()
    let plinks = posts2.iter().map(|post| post.post_title.clone()).collect();
    let pids = posts2.iter().map(|post1| post1.post_id.clone()).collect();
    //let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();
    //(plinks, pids) = list_iter.unwrap_or_default();
    println!("hashmap {:?}",post_id_with_title);

    let template = BlogTemplate {
        post_id_title:post_id_with_title,
        category_id_title: category_id_with_title,
        index_id: &pids,
        index_title: String::from("Posts"),
        page_number: &page_number,
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
    Path((category, page_number)): Path<(i32, i32)>,
) -> impl IntoResponse {
    let mut plinks: Vec<String> = vec![];
    let mut pids: Vec<i32> = vec![];
    let mut post_id_with_title: HashMap<i32, String> = HashMap::new();
    let mut category_id_with_title: HashMap<i32, String> = HashMap::new();
    let final_category = category;
    let mut psec: Vec<String> = vec![];
    let mut pnav: Vec<i32> = vec![];
    psec.clear();
    let category_list = get_all_categories().await;
    let mut psec: Vec<String> = vec![];
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            psec.push(category.clone().category_name);
        })
    });

    let page_number_integer: i32 = page_number;
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();
    let posts2 = get_filtered_from_database(final_category.clone(), offset_start).await.unwrap();

    let number_of_posts_vector =
        count_of_get_filtered_from_database_by_category(final_category).await;
    let m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (m2 + 2) / global_number_of_items_per_page_64();

    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|i| pnav.push(i as i32));
    posts2.clone().iter().for_each(|post| {post_id_with_title.insert(post.post_id,post.post_title.clone());});
    let plinks = posts2.iter().map(|post| post.post_title.clone()).collect();
    pids = posts2.iter().map(|post1| post1.post_id.clone()).collect();
    println!("hashmap {:?}",post_id_with_title);

    let template = HomeFilterTemplate {
        post_id_title: post_id_with_title,
        category_id_title: category_id_with_title,
        index_id: &pids,
        index_title: String::from("Posts"),
        page_number: &page_number,
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
