use std::collections::HashMap;
use crate::model::models::{
    count_of_get_filtered_from_database_by_category, get_all_categories,
    get_filtered_from_database_by_category, HomeFilterTemplate,
};
use crate::{global_number_of_items_per_page_64, BlogTemplate};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::sync::Arc;

use crate::controllers::posts_crud_controller::get_vec_len_of_count;

pub async fn admin_blogs(Path(category): Path<i32>) -> impl IntoResponse {
    let mut psec: Vec<String> = vec![];
    let mut plinks: Vec<String> = vec![];
    let mut pids: Vec<i32> = vec![];
    let mut post_id_with_title: HashMap<i32, String> = HashMap::new();
    let mut category_id_with_title: HashMap<i32, String> = HashMap::new();
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            psec.push(category.clone().category_name);
        })
    });
    let mut pnav: Vec<String> = vec![];
    let string_a: String = category.clone().to_string();
    let string_b: &str = "/pages";
    let current_url = string_a + string_b;
    let posts2 = get_filtered_from_database_by_category(category.clone()).await.unwrap();
    let number_of_posts_vector = count_of_get_filtered_from_database_by_category(category).await;
  //  let shared_state2 = Arc::new(posts2);
    let m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (m2 + 2) / global_number_of_items_per_page_64();
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|i| pnav.push(i.to_string()));
   // let temp = shared_state2.as_ref().as_ref();
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
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
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
    println!("category {}", category);
    let mut pids: Vec<i32> = vec![];
    let mut plinks: Vec<String> = vec![];
    let mut psec: Vec<String> = vec![];
    psec.clear();
    let category_list = get_all_categories().await;
    let mut psec: Vec<String> = vec![];
    let mut category_id_with_title: HashMap<i32, String> = HashMap::new();
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            psec.push(category.clone().category_name);
        })
    });
    let mut pnav: Vec<String> = vec![];
    let string_a: String = category.clone().to_string();
    let string_b: &str = "/pages";
    let current_url = string_a + string_b;
    let posts2 = get_filtered_from_database_by_category(category.clone()).await.unwrap();
    let category_for_ref = category.clone();
   // let shared_state2 = Arc::new(posts2);
    let mut post_id_with_title: HashMap<i32, String> = HashMap::new();
    let number_of_posts_vector =
        count_of_get_filtered_from_database_by_category(category_for_ref).await;
    let m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (m2 + 2) / global_number_of_items_per_page_64();
    println!("count {} and number {} in filter post", m2, number_of_pages);
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|i| pnav.push(i.to_string()));
    // shared_state2
    //     .clone()
    //     .iter()
    //     .for_each(|posts| posts.iter().for_each(|_post| {}));
    //let temp = shared_state2.as_ref().as_ref();
    posts2.iter().for_each(|post| {post_id_with_title.insert(post.post_id,post.post_title.clone());});
    //let list_iter = s.iter().map()
    let plinks = posts2.iter().map(|post| post.post_title.clone()).collect();
    pids = posts2.iter().map(|post1| post1.post_id.clone()).collect();
    //let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();
    //(plinks, pids) = list_iter.unwrap_or_default();
    println!("hashmap {:?}",post_id_with_title);

    let template = HomeFilterTemplate {
        post_id_title: post_id_with_title,
        category_id_title: category_id_with_title,
        index_id: &pids,
        index_title: String::from("Posts"),
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
        current_url_page: current_url,
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

fn get_category_id(category_name: String) -> String {
    if category_name.eq("Category A") {
        return "1".to_string();
    } else if category_name.eq("Category B") {
        return "2".to_string();
    } else if category_name.eq("Category C") {
        return "3".to_string();
    } else {
        return "4".to_string();
    }
}
