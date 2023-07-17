use crate::model::models::{count_of_get_filtered_from_database_by_category, get_all_categories, get_count_of_posts, get_filtered_from_database_by_category, HomeFilterTemplate, HomeTemplate};
use crate::{BlogTemplate, global_number_of_items_per_page_64};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::sync::Arc;

use crate::controllers::posts_crud_controller::{get_vec_len, get_vec_len_of_count};

pub async fn admin_blogs(Path(category): Path<String>) -> impl IntoResponse {
    let mut psec: Vec<String> = vec![];
    let mut plinks: Vec<String> = vec![];
    let mut pids: Vec<i32> = vec![];
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            psec.push(category.clone().category_name);
        })
    });
    let mut pnav: Vec<String> = vec![];
    let string_a: String = category.clone();
    let string_b: &str = "/pages";
    let current_url = string_a + string_b;
    let posts2 = get_filtered_from_database_by_category(category.clone()).await;
    //let ted = count_of_get_filtered_from_database_by_category(category.clone()).await;
    let number_of_posts_vector = count_of_get_filtered_from_database_by_category(category).await;
    let shared_state2 = Arc::new(posts2);
    let m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (m2 + 2) / global_number_of_items_per_page_64();
    (1..number_of_pages+1)
        .into_iter()
        .for_each(|i| pnav.push(i.to_string()));
    let temp = shared_state2.as_ref().as_ref();
    let list_iter = temp.map(|posts| {
        let v: Vec<_> = posts.iter().map(|post| post.post_title.clone()).collect();
        let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();
        (v,v2)
    });
    (plinks,pids) = list_iter.unwrap_or_default();

    let template = BlogTemplate {
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

pub async fn blogs(Path(category): Path<String>) -> impl IntoResponse {
    println!("category {}",category);
    let mut pids: Vec<i32> = vec![];
    let mut plinks: Vec<String> = vec![];
    let mut psec: Vec<String> = vec![];
    psec.clear();
    let category_list = get_all_categories().await;
    let mut psec: Vec<String> = vec![];
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            psec.push(category.clone().category_name);
        })
    });
    let mut pnav: Vec<String> = vec![];
    let string_a: String = category.clone();
    let string_b: &str = "/pages";
    let current_url = string_a + string_b;
    let posts2 = get_filtered_from_database_by_category(category.clone()).await;
    let category_for_ref = category.clone();
    let shared_state2 = Arc::new(posts2);
    let number_of_posts_vector = count_of_get_filtered_from_database_by_category(category_for_ref).await;
    let m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (m2 + 2) / global_number_of_items_per_page_64();
    println!("count {} and number {} in filter post",m2,number_of_pages);
    (1..number_of_pages+1)
        .into_iter()
        .for_each(|i| pnav.push(i.to_string()));
    shared_state2
        .clone()
        .iter()
        .for_each(|posts| posts.iter().for_each(|post| {}));
    let temp = shared_state2.as_ref().as_ref();
    let list_iter = temp.map(|posts| {
        let v: Vec<_> = posts.iter().map(|post| post.post_title.clone()).collect();
        let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();

        (v,v2)
    });
    (plinks,pids) = list_iter.unwrap_or_default();

    let template = HomeFilterTemplate {
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

fn get_category_id(category_name: String) -> String{
    if category_name.eq("Category A") {
        return "1".to_string();
    }
    else if category_name.eq("Category B"){
        return "2".to_string();
    }
    else if category_name.eq("Category C"){
        return "3".to_string();
    }
    else{
        return "4".to_string();
    }
}
