use crate::model::models::{
    GuestTemplate, PostTemplate,
};
use askama::Template;
use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use std::string::String;
use crate::controllers::base_controller::{get_all_categories, category_by_post_id, details_of_post, post_by_id};

pub async fn show_post(Path(post_id_tmp): Path<i32>) -> impl IntoResponse {
    let post_id = post_id_tmp.clone();
    let mut category_in_template: Vec<String> = vec![];
    let mut post_name = String::from("");
    let mut category_name = String::from("");
    let post_details = details_of_post(post_id_tmp).await;
    let post_name_vec = post_by_id(post_id_tmp).await;
    let tmp = post_name_vec.iter();
    for index in tmp {
        post_name = index.post_title.clone();
    }
    let category_name_vec = category_by_post_id(post_name).await;
    let category_name_iter = category_name_vec.iter();
    for index in category_name_iter {
        category_name = index.category_name.clone();
    }
    let category_list = get_all_categories().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_in_template.push(category.clone().category_name);
        })
    });// todo add flatten() here
    let mut template = PostTemplate {
        post_ids: post_id_tmp,
        index_sec: &category_in_template,
        post_title: "none",
        selected_category: "",
        post_description: "",
        post_body: "none",
    };

    post_details.iter().for_each(|posts| {
        let mut post1 = posts.into_iter().find(|&x| post_id == x.post_id).unwrap();
        template = PostTemplate {
            post_ids: post1.post_id,
            index_sec: &category_in_template,
            post_title: &post1.post_title,
            selected_category: &category_name,
            post_description: " ",
            post_body: &post1.post_body,
        };
    });

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn show_posts(Path(post_id_tmp): Path<i32>) -> impl IntoResponse {
    let post_id = post_id_tmp.clone();
    let post_name_vec = post_by_id(post_id).await;
    let post_name_iter = post_name_vec.iter();
    let mut post_name = " ".to_string();
    for index in post_name_iter {
        post_name = index.post_title.to_string();
    }
    let mut category_in_template: Vec<String> = vec![];
    let category_list = get_all_categories().await;
    // category_list.iter().for_each(|categories| {
    //     categories.iter().for_each(|category| {
    //         category_in_template.push(category.clone().category_name);
    //     })
    // });// todo add flatten() here
    let tmp = category_list.into_iter()
        .map(|x| x.into_iter().collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>();
    tmp.iter().for_each(|category| {
        category_in_template.push(category.clone().category_name);
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
