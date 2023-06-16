use crate::model::models::{get_details_of_post, PostTemplate};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn show_post(Path(post_id): Path<String>) -> impl IntoResponse {
    let post_name = post_id.clone();
    let s2 = get_details_of_post(post_id).await;
    let mut template = PostTemplate {
        post_title: "none",
        post_description: "",
        post_body: "none",
    };
    s2.iter().for_each(|posts| {
        posts.into_iter().for_each(|post| {
            if post_name == post.post_title {
                template = PostTemplate {
                    post_title: &post.post_title,
                    post_description: " ",
                    post_body: &post.post_body,
                };
            } else {
            }
        })
    });

    // if template.post_title == "none" {
    //     return (StatusCode::NOT_FOUND, "404 not found").into_response();
    // }
    println!("{}", template);

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}
