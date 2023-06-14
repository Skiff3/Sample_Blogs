use crate::model::models::{get_details_of_post, PostTemplate};
use askama::Template;
use axum::{
    extract::{Path},
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn show_post(
    Path(post_id): Path<String>
) -> impl IntoResponse {
    let _map_post:Vec<String>= Vec::new();
    println!("post name {}", post_id.clone());
    let post_name = post_id.clone();
    let s2 = get_details_of_post(post_id).await;
    let mut template = PostTemplate {
        post_title: "none",
        post_description: "",
        post_body: "none",
    };
    let list_iter = s2.as_ref().map(|posts|{
        let _v1:Vec<_> = posts.into_iter().map(|post|{
            if post_name == post.post_title {
                template = PostTemplate {
                    post_title: &post.post_title,
                    post_description: " ",
                    post_body: &post.post_body,
                };
            } else {

            }
        }).collect();

    });
    list_iter.unwrap_or_default();

    if template.post_title == "none" {
        return (StatusCode::NOT_FOUND, "404 not found").into_response();
    }
    println!("{}", template);

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "try again later").into_response(),
    }
}
