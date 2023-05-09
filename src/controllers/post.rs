use std::sync::Arc;
use askama::Template;
use axum::{
    http::StatusCode, routing::{get, Router},
    response::{Html, IntoResponse},
    extract::{State, Path},
};
use crate::model::models::{BlogTemplate,IndexTemplate,Post,Blog,PostTemplate,get_connection};


pub async fn show_post(Path(post_id): Path<String>, State(state): State<Arc<Vec<Post>>>) -> impl IntoResponse {
    let mut template = PostTemplate{post_title: "none", post_description: "", post_body: "none"};
    for i in 0..state.len() {
        if post_id == state[i].post_title {
            template = PostTemplate{post_title: &state[i].post_title,
                post_description: "",
                post_body: &state[i].post_body,
            };
            break;
        } else {
            continue
        }
    }


    if &template.post_title == &"none" {
        return (StatusCode::NOT_FOUND, "404 not found").into_response();
    }
    println!("{}",template);

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "try again later").into_response()
    }
}