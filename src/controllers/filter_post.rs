use crate::model::models::{
    get_all_categories, get_filtered_from_database_by_category, HomeTemplate,
};
use crate::BlogTemplate;
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::collections::BTreeMap;
use std::sync::Arc;

pub async fn admin_blogs(Path(category): Path<String>) -> impl IntoResponse {
    let mut psec: Vec<String> = Vec::new();
    psec.clear();
    psec.push("Category A".to_string());
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());
    // let mut number_of_pages:i32;
    let mut plinks: Vec<String> = Vec::new();
    let mut pnav: Vec<i32> = Vec::new();
    let mut pids: Vec<i32> = Vec::new();
    let string_a: String = category.clone();
    let string_b: &str = "/pages";
    let current_url = string_a + string_b;

    let posts2 = get_filtered_from_database_by_category(category)
        .await
        .unwrap();

    let shared_state2 = Arc::new(posts2);

    //number_of_pages = shared_state2.len();
    let number_of_pages = if shared_state2.len() % 3 == 0 {
        (shared_state2.len() / 3) as i32
    } else {
        ((shared_state2.len() / 3) + 1) as i32
    };

    for i in 1..number_of_pages + 1 {
        pnav.push(i)
    }

    if shared_state2.len() >= 3 {
        for i in 0..3 {
            plinks.push(shared_state2[i].post_title.clone());
            pids.push(shared_state2[i].post_id);
        }
    } else {
        for i in 0..shared_state2.len() {
            plinks.push(shared_state2[i].post_title.clone());
            pids.push(shared_state2[i].post_id);
        }
    }

    let template = BlogTemplate {
        post_id_title: Default::default(),
        category_id_title: Default::default(),
        index_id: &vec![],
        index_title: String::from("Blogs"),
        page_number: &0,
        category_name: &"".to_string(),
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
    let mut psec: Vec<String> = Vec::new();
    psec.clear();
    psec.push("Category A".to_string());
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());
    // let mut number_of_pages:i32;
    let mut plinks: Vec<String> = Vec::new();
    let mut pnav: Vec<i32> = Vec::new();
    let mut pids: Vec<i32> = Vec::new();
    //let pid: Vec<i32> = vec![];
    let _string_a: String = category.clone();
    let _string_b: &str = "/pages";
    let mut post_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let category_list = get_all_categories().await;
    //let current_url = string_a + string_b;

    let posts2 = get_filtered_from_database_by_category(category)
        .await
        .unwrap();

    posts2.iter().for_each(|post| {
        post_id_with_title.insert(post.post_id, post.post_title.clone());
    });

    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id, category.category_name.clone());
            psec.push(category.clone().category_name);
        })
    });

    let shared_state2 = Arc::new(posts2);
    let number_of_pages = if shared_state2.len() % 3 == 0 {
        shared_state2.len() as i32
    } else {
        ((shared_state2.len() / 3) + 1) as i32
    };
    for i in 1..number_of_pages + 1 {
        pnav.push(i)
    }

    if shared_state2.len() >= 3 {
        for i in 0..3 {
            plinks.push(shared_state2[i].post_title.clone());
            pids.push(shared_state2[i].post_id);
        }
    } else {
        for i in 0..shared_state2.len() {
            plinks.push(shared_state2[i].post_title.clone());
            pids.push(shared_state2[i].post_id);
        }
    }

    let template = HomeTemplate {
        post_id_title: post_id_with_title,
        category_id_title: category_id_with_title,
        index_id: &pids,
        index_title: String::from("Posts"),
        page_number: &1,
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
        current_url_page: ".".to_string(),
    };

    template.render().map(Html).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}
