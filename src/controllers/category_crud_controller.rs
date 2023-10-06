use std::collections::BTreeMap;
use askama::Template;
use axum::extract::Path;
use axum::Form;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use crate::controllers::base_controller::{categories_with_limit, category_by_id, count_of_categories, max_of_category, pool_for_crud, posts_limit_3};
use crate::controllers::posts_crud_controller::{get_max, get_vec_len_of_count};
use crate::{CreateCategory, global_number_of_items_per_page_64};
use crate::model::models::{CategoryTemplate, UpdateCategory, UpdateCategoryTemplate};

pub async fn delete_categories_form(Path(category_id): Path<i32>) -> Redirect {
    let pool = pool_for_crud().await;
    let _res = sqlx::query("update posts set category_id = null where category_id = ($1)")
        .bind(category_id)
        .execute(&pool)
        .await;
    let _res = sqlx::query("delete from category_post where category_id = ($1)")
        .bind(category_id)
        .execute(&pool)
        .await;

    Redirect::to("/admin/categories")
}

pub async fn create_categories_form(Form(create_category): Form<CreateCategory>) -> Redirect {
    let pool = pool_for_crud().await;
    let max_category_id = max_of_category().await;
    let category_id = get_max(max_category_id) + 1;
    let _res =
        sqlx::query("insert into category_post(category_id,category_name) values (($1),($2))")
            .bind(category_id)
            .bind(create_category.category_name)
            .execute(&pool)
            .await;
    Redirect::to("/admin")
}

pub async fn show_all_categories() -> impl IntoResponse {
    let mut category_in_template = vec![];
    let mut category_ids = vec![];
    let mut category_id_with_title: BTreeMap<i32, String> = BTreeMap::new();
    let category_list = categories_with_limit().await;
    category_list.iter().for_each(|categories| {
        categories.iter().for_each(|category| {
            category_id_with_title.insert(category.category_id, category.category_name.clone());
            category_in_template.push(category.clone().category_name);
            category_ids.push(category.clone().category_id);
        })
    });
    let posts = posts_limit_3().await;
    let mut page_number_in_navigation = vec![];
    let number_of_posts_vector = count_of_categories().await;
    let count_of_posts = get_vec_len_of_count(number_of_posts_vector);
    let number_of_pages: i64 = (count_of_posts + 2) / global_number_of_items_per_page_64();
    (1..number_of_pages + 1)
        .into_iter()
        .for_each(|i| page_number_in_navigation.push(i.to_string()));
    let temp = posts.as_ref();
    let list_iter = temp.map(|posts| {
        let v: Vec<_> = posts.iter().map(|post| post.post_title.clone()).collect();
        let v2: Vec<_> = posts.iter().map(|post| post.post_id.clone()).collect();
        (v, v2)
    });

    let (post_title_in_template, post_id_in_template) = list_iter.unwrap_or_default();
    let template = CategoryTemplate {
        category_id_title: category_id_with_title,
        index_id: &post_id_in_template,
        category_id: &category_ids,
        index_title: String::from("Posts"),
        index_links: &post_title_in_template,
        index_sec: &category_in_template,
        page_nav_links: &page_number_in_navigation,
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn update_category_form_ui(Path(category_id): Path<i32>) -> impl IntoResponse {
    let mut category_names = category_by_id(category_id)
        .await
        .first()
        .unwrap()
        .clone()
        .category_name;
    let template = UpdateCategoryTemplate {
        index_name: category_names,
        index_sec: category_id,
    };

    template.render().map(|html| Html(html)).map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render {}", err),
        )
    })
}

pub async fn update_category_form(
    Path(category_id): Path<i32>,
    Form(update_category): Form<UpdateCategory>,
) -> Redirect {
    let pool = pool_for_crud().await;
    let _res =
        sqlx::query("update category_post set category_name = ($1) where category_id = ($2)")
            .bind(update_category.category_name)
            .bind(category_id)
            .execute(&pool)
            .await;

    Redirect::to("/admin/categories")
}
