use crate::model::models::{get_all_categories, get_category_name_by_post_id, get_details_of_post, get_post_name_by_id, GuestTemplate, PostTemplate};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::string::String;
pub async fn show_post(Path(post_id_tmp): Path<i32>) -> impl IntoResponse {
    let post_id = post_id_tmp.clone();
    let mut category_in_template: Vec<String> = vec![];
    let mut post_name = String::from("");
    let mut category_name = String::from("");
    let post_details = get_details_of_post(post_id_tmp).await;
    let post_name_vec = get_post_name_by_id(post_id_tmp).await;
    let tmp = post_name_vec.iter();
    for index in tmp {
        post_name = index.post_title.clone();
    }
    let category_name_vec = get_category_name_by_post_id(post_name).await;
    let category_name_iter = category_name_vec.iter();
    for index in category_name_iter{
        category_name = index.category_name.clone();
    }
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_in_template.push(category.clone().category_name);
        })
    });
    let mut template = PostTemplate {
        post_ids: post_id_tmp,
        index_sec: &category_in_template,
        post_title: "none",
        selected_category: "",
        post_description: "",
        post_body: "none",
    };

    post_details.iter().for_each(|posts|{
       let mut post1 = posts.into_iter().find(|&x| post_id == x.post_id ).unwrap();
        template = PostTemplate {
            post_ids: post1.post_id ,
            index_sec: &category_in_template,
            post_title: &post1.post_title,
            selected_category: &category_name,
            post_description: " ",
            post_body: &post1.post_body,
        };
    });

    // post_details.iter().for_each(|posts| {
    //     posts.into_iter().for_each(|post| {
    //         if post_id == post.post_id {
    //             template = PostTemplate {
    //                 post_ids: post_id_tmp,
    //                 index_sec: &category_in_template,
    //                 post_title: &post.post_title,
    //                 selected_category: &category_name,
    //                 post_description: " ",
    //                 post_body: &post.post_body,
    //             };
    //         } else {
    //         }
    //     })
    // });

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn show_posts(Path(post_id_tmp): Path<i32>) -> impl IntoResponse {
    let post_id = post_id_tmp.clone();
    let post_name_vec = get_post_name_by_id(post_id).await;
    let post_name_iter = post_name_vec.iter();
    let mut post_name = " ".to_string();
    for index in post_name_iter {
        post_name = index.post_title.to_string();
    }
    let mut category_in_template: Vec<String> = vec![];
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_in_template.push(category.clone().category_name);
        })
    });
    let template = GuestTemplate {
        post_title: &post_name,
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
