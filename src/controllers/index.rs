use std::sync::Arc;
use askama::Template;
use axum::{
    http::StatusCode, routing::{get, Router},
    response::{Html, IntoResponse},
    extract::{State, Path},
};
use crate::model::models::{BlogTemplate,IndexTemplate,Post,Blog,PostTemplate,get_connection};


pub async fn index(State(state): State<Arc<Vec<Post>>>) -> impl IntoResponse{
    let mut psec: Vec<String> = Vec::new();
    psec.clear();
    psec.push("Category A".to_string());
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());
    let number_of_pages = 5;
    let s = state.clone();
    let mut plinks: Vec<String> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    for i in 1 .. number_of_pages+1 {
        pnav.push(i.to_string())
    }

    for i in 0 .. 3 {
        plinks.push(s[i].post_title.clone());
        println!("{}",s.len())
    }

    let template = IndexTemplate{index_title: String::from("Sakib"), index_links: &plinks, index_sec: &psec, page_nav_links: &pnav};

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        ).into_response(),
    }
}