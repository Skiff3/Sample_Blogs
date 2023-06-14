use crate::model::models::{get_filtered_from_database_by_category, HomeTemplate};
use crate::BlogTemplate;
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::sync::Arc;


use crate::controllers::posts_crud_controller::get_vec_len;

pub async fn admin_blogs(Path(category): Path<String>) -> impl IntoResponse {
    println!("category {} page number", category);
    let mut psec: Vec<String> = Vec::new();
    psec.clear();
    psec.push("Category A".to_string());
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());
    // let mut number_of_pages:i32;
    let _plinks: Vec<String> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    let _pids: Vec<i32> = Vec::new();
    let string_a: String = category.clone();
    let string_b: &str = "/pages";
    let current_url = string_a + string_b;
    println!("current url {}", current_url);
    let posts2 = get_filtered_from_database_by_category(category).await;

    let shared_state2 = Arc::new(posts2);

    //number_of_pages = shared_state2.len();
    let number_of_pages = if get_vec_len(shared_state2.clone()) % 3 == 0 {
        (get_vec_len(shared_state2.clone()) / 3) as i32
    } else {
        ((get_vec_len(shared_state2.clone()) / 3) + 1) as i32
    };

    for i in 1..number_of_pages + 1 {
        pnav.push(i.to_string())
    }
    let temp = shared_state2.as_ref().as_ref();
    let list_iter = temp.map(|posts| {
        let v: Vec<_> = posts.iter()
            .map(|post| {post.post_title.clone()}).collect();
        let v2: Vec<_> = posts.iter()
            .map(|post| {post.post_id.clone()}).collect();

        (v,v2)
    });
    let (plinks, _pids) = list_iter.unwrap_or_default();


    let template = BlogTemplate {
        index_id: &vec![],
        index_title: String::from("Blogs"),
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
        current_url_page: current_url,
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

pub async fn blogs(Path(category): Path<String>) -> impl IntoResponse {
    println!("category {} page number", category);
    let mut psec: Vec<String> = Vec::new();
    psec.clear();
    psec.push("Category A".to_string());
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());
    // let mut number_of_pages:i32;
    let _plinks: Vec<String> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    let _pids: Vec<i32> = Vec::new();
    let string_a: String = category.clone();
    let string_b: &str = "/pages";
    let current_url = string_a + string_b;
    println!("current url {}", current_url);
    let posts2 = get_filtered_from_database_by_category(category).await;

    let shared_state2 = Arc::new(posts2);
    let number_of_pages = if get_vec_len(shared_state2.clone()) % 3 == 0 {
        (get_vec_len(shared_state2.clone()) / 3) as i32
    } else {
        ((get_vec_len(shared_state2.clone()) / 3) + 1) as i32
    };
    for i in 1..number_of_pages + 1 {
        pnav.push(i.to_string())
    }

    let temp = shared_state2.as_ref().as_ref();
    let list_iter = temp.map(|posts| {

        let v: Vec<_> = posts.iter()
            .map(|post| {post.post_title.clone()}).collect();
        let v2: Vec<_> = posts.iter()
            .map(|post| {post.post_id.clone()}).collect();

        (v,v2)
    });
    let (plinks, _pids) = list_iter.unwrap_or_default();

    let template = HomeTemplate {
        index_id: &vec![],
        index_title: String::from("Blogs"),
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
        current_url_page: current_url,
    };


    template.render().map(
        |html|Html(html)
    )
        .map_err(|err|(StatusCode::INTERNAL_SERVER_ERROR,format!("Failed to render {}",err)))
}
