use std::sync::Arc;
use askama::Template;
use axum::{
    http::StatusCode, routing::{get, Router},
    response::{Html, IntoResponse},
    extract::{State, Path},
};
use sqlx::postgres::PgPoolOptions;
use crate::{Blog, BlogTemplate, IndexTemplate, Post};
use crate::controllers::navigate::page;

pub async fn blog_pagination(Path(query_title): Path<String>) -> impl IntoResponse {
    println!("page {} ",query_title);
    //let query_title = "1".to_string();// query
    let pools = PgPoolOptions::new()
        .max_connections(5)// 65
        .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
        .await
        .expect("couldn't connect to the database");
    let mut plinks: Vec<String> = Vec::new();
    let mut number_of_pages = 0;
    let mut psec: Vec<String> = Vec::new();// psec psec consists of vector string
    let mut pnav: Vec<String> = Vec::new();// pnav pnav consists of vector string
    psec.clear();
    //let mut psec: Vec<String> = Vec::new();
    psec.push("Category A".to_string());
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());


    
    let final_category = "Category B";

    let mut posts2 = sqlx::query_as::<_, Blog>("select p.post_title, p.post_description, p.post_body, c.category_id, c.category_name from posts p, category_post c where p.category_id=c.category_id and c.category_name = ($1)")
        .bind(final_category)
        .fetch_all(&pools)
        .await
        .unwrap();

    for post in &mut posts2 {
        post.post_title = post.post_title.replace("-", " ");
    }

    let shared_state2 = Arc::new(posts2);
    println!("#3 {}",shared_state2.len());
    if shared_state2.len()%3==0 {
        number_of_pages = shared_state2.len()/3;
    }
    else{
        number_of_pages = (shared_state2.len()/3)+1;
    }
    println!("total{} number of pages {}",shared_state2.len(),number_of_pages);
    for i in 1 .. number_of_pages+1 {
        pnav.push(i.to_string())
    }
    for i in 0 .. shared_state2.len() {
        plinks.push(shared_state2[i].post_title.clone());
    }

    if  query_title.clone() == "1" {
        println!("in {}", query_title);
        plinks.clear();
        for i in 0 .. 3 {
            plinks.push(shared_state2[i].post_title.clone());
        }
    }
    else if query_title.clone() == "2" {
        println!("in {}", query_title);
        plinks.clear();
        for i in 3 .. 6 {
            plinks.push(shared_state2[i].post_title.clone());
            println!("len of plinks {}",plinks.len());
        }
    }
    else if  query_title.clone() == "3" {
        //println!("in {}", query_title);
        println!("in {}",query_title);
        plinks.clear();
        for i in 6 .. shared_state2.len() {
            plinks.push(shared_state2[i].post_title.clone());
            //println!("{}",s[i].post_title)
        }
    }
    else if  query_title.clone() == "4" {
        plinks.clear();
        for i in 9 .. 12 {
            plinks.push(shared_state2[i].post_title.clone());
           // println!("{}",s[i].post_title)
        }
    }
    else if  query_title.clone() == "5" {
        plinks.clear();
        for i in 12 .. 15 {
            plinks.push(shared_state2[i].post_title.clone());
            //println!("{}",s[i].post_title)
        }
    }
    else {
        plinks.clear();
        for i in 0 .. 3 {
            plinks.push(shared_state2[i].post_title.clone());
            //println!("{}",s.len())
        }
    }


    let template = BlogTemplate{index_title: String::from("Sakib"), index_links: &plinks, index_sec: &psec, page_nav_links: &pnav};

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        ).into_response(),
    }
}