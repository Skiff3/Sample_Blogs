use std::collections::HashMap;
use crate::controllers::posts_crud_controller::get_vec_len_of_count;
use crate::model::models::{get_all_categories, get_count_of_posts, get_posts_per_page, HomeTemplate};
use crate::{global_number_of_items_per_page, global_number_of_items_per_page_64, IndexTemplate};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn page(Path(page_number): Path<i32>) -> impl IntoResponse {
    println!("{}", page_number);
    let mut plinks: Vec<String> = vec![];
    let mut psec: Vec<String> = vec![];
    let mut pnav: Vec<i32> = vec![];
    let mut post_id_with_title: HashMap<i32, String> = HashMap::new();
    let mut category_id_with_title: HashMap<i32, String> = HashMap::new();
    psec.clear();
    let category_list = get_all_categories().await;
    let mut psec: Vec<String> = vec![];
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            psec.push(category.clone().category_name);
        })
    });

    let _page_number_inter: i64 = page_number as i64;
    let page_number_integer: i32 = page_number as i32;
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();
    let posts = get_posts_per_page(offset_start).await.unwrap();
    let number_of_posts_vector = get_count_of_posts().await;
    let m = number_of_posts_vector;
    let number_of_pages: i64 = if get_vec_len_of_count(m) % global_number_of_items_per_page_64()
        == 0
    {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64()
    } else {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64() + 1
    };

    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|i| pnav.push(i as i32));

    plinks.clear();
    posts.iter().for_each(|post| {post_id_with_title.insert(post.post_id,post.post_title.clone());});
    //let list_iter = s.iter().map()
    let plinks = posts.iter().map(|post| post.post_title.clone()).collect();
    let pids = posts.iter().map(|post1| post1.post_id.clone()).collect();
    //let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();
    //(plinks, pids) = list_iter.unwrap_or_default();
    println!("hashmap {:?}",post_id_with_title);
    let mut temp: i32 = page_number;

    let template = IndexTemplate {
        post_id_title: post_id_with_title,
        category_id_title:category_id_with_title,
        index_id: &pids,
        index_title: String::from("Posts"),
        page_number: &page_number,
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

pub async fn pages(Path(page_number): Path<i32>) -> impl IntoResponse {
    let mut plinks: Vec<String> = vec![];
    let mut psec: Vec<String> = vec![];
    let mut pid: Vec<i32> = vec![];
    let mut pnav: Vec<i32> = vec![];
    let mut category_id_with_title: HashMap<i32, String> = HashMap::new();
    psec.clear();
    let category_list = get_all_categories().await;
    let mut psec: Vec<String> = vec![];
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            psec.push(category.clone().category_name);
        })
    });
    let mut post_id_with_title: HashMap<i32, String> = HashMap::new();
    let page_number_integer: i32 = page_number;
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();
    let posts = get_posts_per_page(offset_start).await.unwrap();
    let number_of_pages: i64 = if get_vec_len_of_count(get_count_of_posts().await)
        % global_number_of_items_per_page_64()
        == 0
    {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64()
    } else {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64() + 1
    };
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|i| pnav.push(i as i32));
    plinks.clear();// plinks.clear();
    // let temp = s.as_ref();
    // let list_iter = temp.clone().map(|posts| {
    //     let v: Vec<_> = posts.iter().map(|post| post.post_title.clone()).collect();
    //     let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();
    //     (v, v2)
    // });
    //(plinks, pid) = list_iter.unwrap_or_default();
    posts.iter().for_each(|post| {post_id_with_title.insert(post.post_id,post.post_title.clone());});
    //let list_iter = s.iter().map()
    let plinks = posts.iter().map(|post| post.post_title.clone()).collect();
    pid = posts.iter().map(|post1| post1.post_id.clone()).collect();
    //let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();
    //(plinks, pids) = list_iter.unwrap_or_default();
    println!("hashmap {:?}",post_id_with_title);

    let template = HomeTemplate {
        post_id_title:post_id_with_title ,
        category_id_title: category_id_with_title,
        index_id: &pid,
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
