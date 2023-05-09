use std::sync::Arc;
use askama::Template;
use axum::{
    http::StatusCode, routing::{get, Router},
    response::{Html, IntoResponse},
    extract::{State, Path},// extract includes state and path.
};
use crate::{IndexTemplate, Post};

pub async fn page(Path(page_number): Path<String> , State(state): State<Arc<Vec<Post>>>) -> impl IntoResponse {
    println!("{}", page_number);
    let mut plinks: Vec<String> = Vec::new();
    let number_of_pages = 5;
    let mut psec: Vec<String> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    psec.clear();
    //let mut psec: Vec<String> = Vec::new();
    psec.push("Category A".to_string());
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());
    for i in 1 .. number_of_pages+1 {
        pnav.push(i.to_string())
    }

    let s = state.clone();// s indicates state
    println!("{}", s[10].post_title);
    if  page_number == "1" {
        plinks.clear();
        for i in 0 .. 3 {
            plinks.push(s[i].post_title.clone());// clone of s[i]
        }
    }
    else if page_number == "2" {
        plinks.clear();
        for i in 3 .. 6 {
            plinks.push(s[i].post_title.clone());// clone of s[i]
            println!("{}",plinks.len());
            println!("new {}",s[i].post_title)// to check the items in vector.
        }
    }
    else if  page_number == "3" {
        println!("in {}",page_number);
        plinks.clear();
        for i in 6 .. 9 {
            plinks.push(s[i].post_title.clone());// clone of s[i]
            println!("{}",s[i].post_title)// to check the items in vector.
        }
    }
    else if  page_number == "4" {
        plinks.clear();
        for i in 9 .. 12 {
            plinks.push(s[i].post_title.clone());// clone of s[i]
            println!("{}",s[i].post_title)// to check the items in vector.
        }
    }
    else if  page_number == "5" {
        plinks.clear();
        for i in 12 .. 15 {
            plinks.push(s[i].post_title.clone());// clone of s[i]
            println!("{}",s[i].post_title)// to check the items in vector.
        }
    }
    else {
        plinks.clear();
        for i in 0 .. 3 {
            plinks.push(s[i].post_title.clone());// clone of s[i]
            println!("{}",s.len())// to check the items in vector.
        }
    }


    let template = IndexTemplate{index_title: String::from("Blogs"), index_links: &plinks, index_sec: &psec, page_nav_links: &pnav};

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        ).into_response(),
    }
}