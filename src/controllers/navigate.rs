use crate::model::models::{get_count_of_posts, get_posts_per_page, HomeTemplate};
use crate::{global_number_of_items_per_page, global_number_of_items_per_page_64, IndexTemplate};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use crate::controllers::posts_crud_controller::get_vec_len_of_count;

pub async fn page(Path(page_number): Path<String>) -> impl IntoResponse {
    println!("{}", page_number);
    let mut plinks: Vec<String> = Vec::new();
    let mut psec: Vec<String> = Vec::new();
    let _pid: Vec<i32> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    psec.clear();
    psec.push("Category A".to_string());
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());

    let page_number_integer: i32 = page_number.parse().unwrap();
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page(); // offset value.
    println!("page starts from {}", offset_start);
    let s = get_posts_per_page(offset_start).await;
    let number_of_posts_vector = get_count_of_posts().await;
    let m = number_of_posts_vector;
    let number_of_pages: i64 = if get_vec_len_of_count(m) % global_number_of_items_per_page_64() == 0 {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64()
    } else {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64() + 1 //
    };

    for i in 1..number_of_pages + 1 {
        pnav.push(i.to_string())
    }

    plinks.clear();// return result
    let list_iter = s.map(|posts| {
        //plinks = posts.iter()
        //.map(|post| {post.post_title.clone()}).collect();
        let v: Vec<_> = posts.iter()
            .map(|post| {post.post_title.clone()}).collect();
        let v2: Vec<_> = posts.iter()
            .map(|post| {post.post_id.clone()}).collect();

        (v,v2)
    });

    let (plinks,pid) = list_iter.unwrap_or_default();

    let template = IndexTemplate {
        index_id: &pid,
        index_title: String::from("Blogs"),
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}

pub async fn pages(Path(page_number): Path<String>) -> impl IntoResponse {
    println!("{}", page_number);
    let mut plinks: Vec<String> = Vec::new();
    let mut psec: Vec<String> = Vec::new();
    let mut pid: Vec<i32> = Vec::new();
    let mut pnav: Vec<String> = Vec::new(); // let mut pnav: Vec<String> = Vec::new();
    psec.clear();

    psec.push("Category A".to_string()); // psec.push("Category A")
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());

    let page_number_integer: i32 = page_number.parse().unwrap();
    let offset_start: i32 = (page_number_integer - 1) * global_number_of_items_per_page(); // offset value.
    println!("page starts from {}", offset_start);
    let s = get_posts_per_page(offset_start).await;
    let number_of_posts_vector = get_count_of_posts().await;
    let _m = number_of_posts_vector;
    let number_of_pages: i64 = if get_vec_len_of_count(get_count_of_posts().await) % global_number_of_items_per_page_64() == 0 {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64()
    } else {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64() + 1
    };
    for i in 1..number_of_pages + 1 {
        pnav.push(i.to_string())
    }

    plinks.clear();
    let temp = s.as_ref();
    let list_iter = temp.clone().map(|posts| {
        //plinks = posts.iter()
        //.map(|post| {post.post_title.clone()}).collect();
        let v: Vec<_> = posts.iter()
            .map(|post| {post.post_title.clone()}).collect();
        let v2: Vec<_> = posts.iter()
            .map(|post| {post.post_id.clone()}).collect();

        (v,v2)
    });

    (plinks,pid) = list_iter.unwrap_or_default();

    let template = HomeTemplate {
        index_id: &pid,
        index_title: String::from("Blogs"),
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
        current_url_page: ".".to_string(),
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(), //
    }
}
