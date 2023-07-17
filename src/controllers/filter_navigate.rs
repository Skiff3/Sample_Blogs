use crate::model::models::{count_of_get_filtered_from_database_by_category, get_all_categories, get_count_of_posts, get_filtered_from_database, HomeFilterTemplate, HomeTemplate};
use crate::{global_number_of_items_per_page, BlogTemplate, global_number_of_items_per_page_64};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::sync::Arc;
use cookie::time::macros::date;
use rand::thread_rng;

use crate::controllers::posts_crud_controller::{get_vec_len, get_vec_len_of_count};

pub async fn admin_blog_pagination(
    Path((category, page_number)): Path<(String, String)>,
) -> impl IntoResponse {
    let mut plinks: Vec<String> = vec![];
    let mut pids: Vec<i32> = vec![];
    let mut len = 0;
    let final_category = &category[0..category.len()];
    let mut psec: Vec<String> = vec![];
    let mut pnav: Vec<String> = vec![];
    psec.clear();
    let category_list = get_all_categories().await;
    let mut psec: Vec<String> = vec![];
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            psec.push(category.clone().category_name);
        })
    });

    let page_number_integer: i32 = page_number.parse().unwrap();
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();

    let posts2 = get_filtered_from_database(final_category.clone().to_string(), offset_start).await;
    let temp = posts2.as_ref();
    temp.into_iter().for_each(|p|{
        len = p.len();
        println!("p.len {}{:?}",p.len(),p);
    });
    let number_of_posts_vector = count_of_get_filtered_from_database_by_category(final_category.clone().to_string()).await;
    println!("len {}",len);
    let shared_state2 = Arc::new(posts2);
    //let count_of_posts_result = get_vec_len_of_count(number_of_posts_vector);
    let m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (m2 + 2) / global_number_of_items_per_page_64();
    println!("number in filter navigate {}",number_of_pages);
    (1..number_of_pages + 1)
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
    let category_list = get_all_categories().await;
    let mut psec: Vec<String> = vec![];
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            psec.push(category.clone().category_name);
        })
    });

    let page_number_integer: i32 = page_number.parse().unwrap();
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();

    let posts2 = get_filtered_from_database(final_category.clone().to_string(), offset_start).await;

    let shared_state2 = Arc::new(posts2);
    let number_of_posts_vector = count_of_get_filtered_from_database_by_category(final_category.to_string()).await;
    let m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (m2 + 2) / global_number_of_items_per_page_64();

    (1..number_of_pages+1)
        .into_iter()
        .for_each(|i| pnav.push(i.to_string()));
    let temp = shared_state2.as_ref().as_ref();
    let list_iter = temp.map(|posts| {
        let v: Vec<_> = posts.iter().map(|post| post.post_title.clone()).collect();
        let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();

        (v, v2)
    });

    (plinks, pids) = list_iter.unwrap_or_default();

    let template = HomeFilterTemplate {
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
