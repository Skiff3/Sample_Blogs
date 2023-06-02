use std::collections::HashMap;
use std::sync::Arc;
use askama::Template;
use crate::{AuthMemoryStore, global_number_of_items_per_page, global_number_of_items_per_page_64, User};
use crate::SessionMemoryStore;
use axum::{http::StatusCode, routing::{get, Router}, response::{Html, IntoResponse}, extract::{State, Path}, Extension};
use axum_login::{AuthLayer, AuthUser, RequireAuthorizationLayer};
use axum_login::axum_sessions::SessionLayer;
use axum_login::extractors::AuthContext;
use axum_login::secrecy::SecretVec;
use rand::Rng;
use tokio::sync::RwLock;
use crate::model::models::{BlogTemplate, IndexTemplate, Post, Blog, PostTemplate, get_connection, get_count_of_posts, get_posts_per_page, get_all_categories};


pub async fn index(State(state): State<Arc<Vec<Post>>>) -> impl IntoResponse{
    let mut psec: Vec<String> = Vec::new();
    psec.clear();

    let mut category_list = get_all_categories().await;
    for i in  0 .. category_list.len(){
        psec.push(category_list[i].clone().category_name);
    }
    let s = state.clone();
    let mut number_of_pages: i64 = 0;
    let mut plinks: Vec<String> = Vec::new();
    let mut pids: Vec<i32> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    let mut number_of_posts_vector = get_count_of_posts().await;
    let m = number_of_posts_vector.clone();
    if m[0].count % global_number_of_items_per_page_64() ==0 {
        number_of_pages = ((m[0].count)/global_number_of_items_per_page_64());
    }
    else {
        number_of_pages = ((m[0].count)/global_number_of_items_per_page_64())+1;
    }
    println!("the number of pages are {}, count of posts {}",number_of_pages,m[0].count);
    for i in 1 .. number_of_pages+1 {
        pnav.push(i.to_string())
    }

    for i in 0 .. s.len() {
        plinks.push(s[i].post_title.clone());
        pids.push(s[i].post_id.clone());
        println!("{}",s.len())// prints the s length
    }

    let template = IndexTemplate{ index_id: &pids, index_title: String::from("Blogs"), index_links: &plinks, index_sec: &psec, page_nav_links: &pnav};

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        ).into_response(),
    }
}