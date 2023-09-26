use std::collections::{BTreeMap, HashMap};
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
    let mut plinks: Vec<String> = vec![];
    let mut psec: Vec<String> = vec![];
    let mut pnav: Vec<i32> = vec![];
    let mut post_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    psec.clear();
    let category_list = get_all_categories().await;
    let mut psec: Vec<String> = vec![];
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            psec.push(category.clone().category_name);
        })
    });
    let mut length_posts = get_vec_len_of_count(get_count_of_posts().await) as usize;
    let pages: Pages = Pages::new(
        get_vec_len_of_count(get_count_of_posts().await)
            .try_into()
            .unwrap(),
        global_number_of_items_per_page() as usize
    );
    let page = pages.to_page_number(page_number as usize);
    let mut no_of_pages = page.unwrap_or_default();
    println!("no of pages {}",no_of_pages.begin);
    let mut html_from_crate = pages.generate_html(length_posts);
    println!("count and html {},{}",page.begin,html_from_crate);

    let _page_number_inter: i64 = page_number as i64; // html from length posts
    let page_number_integer: i32 = page_number as i32;
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();
    let posts = get_posts_per_page(page.begin as i32).await.unwrap();
    let number_of_posts_vector = get_count_of_posts().await;
    let m = number_of_posts_vector;// global number of items per page
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
    let plinks = posts.iter().map(|post| post.post_title.clone()).collect();
    let pids = posts.iter().map(|post1| post1.post_id.clone()).collect();
    let template = IndexTemplate {
        post_id_title: post_id_with_title,
        category_id_title:category_id_with_title,
        index_id: &pids,
        index_title: String::from("Posts"),
        page_number: &page_number,
        selected_category: &"Category A".to_string(),
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
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    psec.clear();
    let category_list = get_all_categories().await;
    let mut psec: Vec<String> = vec![];
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id,category.category_name.clone());
            psec.push(category.clone().category_name);
        })
    });
    let mut post_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
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
    plinks.clear();
    posts.iter().for_each(|post| {post_id_with_title.insert(post.post_id,post.post_title.clone());});
    let plinks = posts.iter().map(|post| post.post_title.clone()).collect();
    pid = posts.iter().map(|post1| post1.post_id.clone()).collect();
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
