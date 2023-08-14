use crate::model::models::{get_all_categories, get_category_name_by_post_id, get_details_of_post, get_post_name_by_id, GuestTemplate, PostTemplate};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::string::String;

pub async fn show_post(Path(post_id): Path<i32>) -> impl IntoResponse {
    let post_name = post_id.clone();
    let mut psec: Vec<String> = vec![];
    let mut p_name = String::from("");
    let mut c_name = String::from("");
    let s2 = get_details_of_post(post_id).await;
    let a = get_post_name_by_id(post_id).await;
    let b = a.iter();
    for i in b{
        p_name = i.post_title.clone();
    }
    let c = get_category_name_by_post_id(p_name).await;
    let d = c.iter();
    for j in d{
        c_name = j.category_name.clone();
    }
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
        selected_category: "",
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
                    selected_category: &c_name,
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
