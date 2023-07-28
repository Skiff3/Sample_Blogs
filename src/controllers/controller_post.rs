use crate::model::models::{get_all_categories, get_details_of_post, get_post_name_by_id, GuestTemplate, PostTemplate};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};

pub async fn show_post(Path(post_id): Path<i32>) -> impl IntoResponse {
    let post_name = post_id.clone();
    let mut psec: Vec<String> = vec![];
    let s2 = get_details_of_post(post_id).await;
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            psec.push(category.clone().category_name);
        })
    });
    let mut template = PostTemplate {
        post_ids: post_id,
        index_sec: &psec,
        post_title: "none",
        post_description: "",
        post_body: "none",
    };

    s2.iter().for_each(|posts| {
        posts.into_iter().for_each(|post| {
            if post_name == post.post_id {
                template = PostTemplate {
                    post_ids: post_id,
                    index_sec: &psec,
                    post_title: &post.post_title,
                    post_description: " ",
                    post_body: &post.post_body,
                };
            } else {
            }
        })
    });

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn show_posts(Path(post_name): Path<i32>) -> impl IntoResponse {
    let post_name = post_name.clone();
    let vec_post_title = get_post_name_by_id(post_name).await;
    let iters = vec_post_title.iter();
    let mut str_post_title = " ".to_string();
    for i in iters{
        str_post_title = i.post_title.to_string();
    }
    let mut psec: Vec<String> = vec![];
    //let s2 = get_details_of_post(post_id).await;
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            psec.push(category.clone().category_name);
        })
    });
    let template = GuestTemplate {
        post_title: &str_post_title,
        post_description: "Description of the post in detail",
        post_body: "Description of the post in detail",
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}
