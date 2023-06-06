use crate::model::models::{get_details_of_post, Post, PostTemplate};
use askama::Template;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse}
};
use std::sync::Arc;

pub async fn show_post(
    Path(post_id): Path<String>,
    State(_state): State<Arc<Vec<Post>>>,
) -> impl IntoResponse {
    println!("post name {}", post_id.clone());
    let post_name = post_id.clone();
    let s2 = get_details_of_post(post_id).await;
    let mut template = PostTemplate {
        post_title: "none",
        post_description: "",
        post_body: "none",
    };
    let list_iter = s2.iter();
    for i in list_iter {
        if post_name == i.post_title {
            template = PostTemplate {
                post_title: &i.post_title,
                post_description: "",
                post_body: &i.post_body,
            };
            break;
        } else {
            continue;
        }
    }

    if template.post_title == "none" {
        return (StatusCode::NOT_FOUND, "404 not found").into_response();
    }
    println!("{}", template);

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "try again later").into_response(),
    }
}
