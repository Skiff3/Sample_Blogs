use crate::controllers::posts_crud_controller::get_vec_len_of_count;
use crate::model::models::{get_count_of_posts, get_posts_per_page, HomeTemplate};
use crate::{global_number_of_items_per_page, global_number_of_items_per_page_64, IndexTemplate};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn page(Path(page_number): Path<String>) -> impl IntoResponse {
    println!("{}", page_number);
    let mut plinks: Vec<String> = vec![];
    let mut psec: Vec<String> = vec![];
    let mut pnav: Vec<String> = vec![];
    psec.clear();
    let psec = vec![
        "Category A".to_string(),
        "Category B".to_string(),
        "Category C".to_string(),
        "No Category".to_string(),
    ];
    let page_number_integer: i32 = page_number.parse().unwrap();
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();
    println!("page starts from {}", offset_start);
    let s = get_posts_per_page(offset_start).await;
    let number_of_posts_vector = get_count_of_posts().await;
    let m = number_of_posts_vector;
    let number_of_pages: i64 = if get_vec_len_of_count(m) % global_number_of_items_per_page_64()
        == 0
    {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64()
    } else {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64() + 1
    };

    (1..number_of_pages)
        .into_iter()
        .for_each(|i| pnav.push(i.to_string()));

    plinks.clear();
    let list_iter = s.map(|posts| {
        let v: Vec<_> = posts.iter().map(|post| post.post_title.clone()).collect();
        let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();
        (v, v2)
    });

    let (plinks, pid) = list_iter.unwrap_or_default();

    let template = IndexTemplate {
        index_id: &pid,
        index_title: String::from("Blogs"),
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

pub async fn pages(Path(page_number): Path<String>) -> impl IntoResponse {
    let mut plinks: Vec<String> = vec![];
    let mut psec: Vec<String> = vec![];
    let mut pid: Vec<i32> = vec![];
    let mut pnav: Vec<String> = vec![];
    psec.clear();
    let psec = vec![
        "Category A".to_string(),
        "Category B".to_string(),
        "Category C".to_string(),
        "No Category".to_string(),
    ];
    let page_number_integer: i32 = page_number.parse().unwrap();
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page();
    let s = get_posts_per_page(offset_start).await;
    let number_of_pages: i64 = if get_vec_len_of_count(get_count_of_posts().await)
        % global_number_of_items_per_page_64()
        == 0
    {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64()
    } else {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64() + 1
    };
    (1..number_of_pages)
        .into_iter()
        .for_each(|i| pnav.push(i.to_string()));
    plinks.clear();
    let temp = s.as_ref();
    let list_iter = temp.clone().map(|posts| {
        let v: Vec<_> = posts.iter().map(|post| post.post_title.clone()).collect();
        let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();
        (v, v2)
    });
    (plinks, pid) = list_iter.unwrap_or_default();

    let template = HomeTemplate {
        index_id: &pid,
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
