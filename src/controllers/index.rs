use crate::controllers::posts_crud_controller::get_vec_len_of_count;
use crate::global_number_of_items_per_page_64;
use crate::model::models::{get_all_categories, get_count_of_posts, IndexTemplate, Post};
use askama::Template;
use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode, response::Html};
use sqlx::Error;
use std::sync::Arc;

pub async fn index(State(state): State<Arc<Result<Vec<Post>, Error>>>) -> impl IntoResponse {
    let mut psec: Vec<String> = vec![];
    psec.clear();
    let mut m2: i64 = 0;
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            psec.push(category.clone().category_name);
        })
    });
    let s = state.clone();
    let mut pnav: Vec<String> = vec![];
    let number_of_posts_vector = get_count_of_posts().await;
    m2 = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = if m2 % global_number_of_items_per_page_64() == 0 {
        (m2) / global_number_of_items_per_page_64()
    } else {
        (m2) / global_number_of_items_per_page_64() + 1
    };
    (0..number_of_pages)
        .into_iter()
        .for_each(|i| pnav.push(i.to_string()));
    println!("number of pages {}", m2.clone());
    let temp = s.as_ref().as_ref();
    let list_iter = temp.map(|posts| {
        let v: Vec<_> = posts.iter().map(|post| post.post_title.clone()).collect();
        let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();
        (v, v2)
    });
    let (plinks, pids) = list_iter.unwrap_or_default();
    let template = IndexTemplate {
        index_id: &pids,
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
