use std::error::Error;
use std::sync::Arc;
use askama::Template;
use axum::{
    http::StatusCode, routing::{get, Router},
    response::{Html, IntoResponse},
    extract::{State, Path},
};
use tower_cookies::{Cookie, CookieManagerLayer, Cookies};
use sqlx::postgres::PgPoolOptions;
use crate::{Blog, BlogTemplate};
use crate::controllers::filter_navigate::blog_pagination;


pub async fn blogs(Path(query_title): Path<String>) -> impl IntoResponse {
    println!("{}",query_title);
    let name = query_title.clone();
    let pools = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
        .await
        .expect("couldn't connect to the database");


    let mut psec: Vec<String> = Vec::new();
    psec.clear();
    psec.push("Category A".to_string());// psec
    psec.push("Category B".to_string());// psec
    psec.push("Category C".to_string());// psec
    psec.push("No Category".to_string());
    let mut number_of_pages = 0;
    //let s = state.clone();
    let mut plinks: Vec<String> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
   // let shared_state2 = Arc::new(posts2);


//     let query = r#"
// insert into filter_category
//     ( filter_name )
// values
//     ( $1 )
// returning *
// "#;
//
//     let result: Result<> = sqlx::query_as::<_, String>(query)
//         .bind(&name);



    let mut posts2 = sqlx::query_as::<_, Blog>("select p.post_title, p.post_description, p.post_body, c.category_id, c.category_name from posts p, category_post c where p.category_id=c.category_id and c.category_name = ($1)")
        .bind(query_title)
        .fetch_all(&pools)
        .await
        .unwrap();

    for post in &mut posts2 {
        post.post_title = post.post_title.replace("-", " ");
    }

    let shared_state2 = Arc::new(posts2);
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

    if shared_state2.len() >= 3 {
        for i in 0 .. 3 {
            plinks.push(shared_state2[i].post_title.clone());
        }
    }
    else{
        for i in 0 .. shared_state2.len() {
            plinks.push(shared_state2[i].post_title.clone());
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
