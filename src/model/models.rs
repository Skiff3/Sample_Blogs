use crate::global_number_of_items_per_page;
use askama::Template;
use axum_login::axum_sessions::async_session::serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

use crate::controllers::posts_crud_controller::get_connection_for_crud;
use sqlx::*;

mod filters {
    pub fn rmdashes(title: &str) -> askama::Result<String> {
        let a: char = '-';
        let b: &str = " ";
        Ok(title.replace(a, b))
    }
}

#[derive(FromRow, Debug, Clone)]
pub struct Post {
    pub post_id: i32,
    pub post_title: String,
    pub post_description: String,
    pub post_body: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct Category {
    pub category_id: i32,
    pub category_name: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct Count {
    pub count: i64,
}

#[derive(FromRow, Debug, Clone)]
pub struct Max {
    pub max: i32,
}

#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostTemplate<'a> {
    pub post_ids: i32,
    pub index_sec: &'a Vec<String>,
    pub post_title: &'a str,
    pub selected_category: &'a str,
    pub post_description: &'a str,
    pub post_body: &'a str, //
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate<'a> {
    pub user_name: &'a str,
    pub password: &'a str,
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate<'a> {
    pub user_name: &'a str,
    pub password: &'a str,
    pub repeat_password: &'a str,
}

#[derive(Template)]
#[template(path = "admins.html")]
pub struct AdminTemplate {}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    pub post_id_title: BTreeMap<i32, String>,
    pub category_id_title: BTreeMap<i32, String>,
    pub index_id: &'a Vec<i32>,
    pub index_title: String,
    pub page_number: &'a i32,
    pub index_links: &'a Vec<String>,
    pub index_sec: &'a Vec<String>,
    pub page_nav_links: &'a Vec<i32>,
    pub current_url_page: String,
}

#[derive(Template)]
#[template(path = "home_filter_navigation.html")]
pub struct HomeFilterTemplate<'a> {
    pub post_id_title: BTreeMap<i32, String>,
    pub category_id_title: BTreeMap<i32, String>,
    pub index_id: &'a Vec<i32>,
    pub index_title: String,
    pub page_number: &'a i32,
    pub category_name: &'a String,
    pub index_links: &'a Vec<String>,
    pub index_sec: &'a Vec<String>,
    pub page_nav_links: &'a Vec<i32>,
    pub current_url_page: String,
}

#[derive(Template)]
#[template(path = "new_post.html")]
pub struct NewPostTemplate<'a> {
    pub index_sec: &'a Vec<String>,
}

#[derive(Template)]
#[template(path = "new_category.html")]
pub struct NewCategoryTemplate {}

#[derive(Template)]
#[template(path = "home_post.html")]
pub struct GuestTemplate<'a> {
    pub post_title: &'a str,
    pub post_description: &'a str,
    pub post_body: &'a str,
}

#[derive(Template)]
#[template(path = "update_category.html")]
pub struct UpdateCategoryTemplate {
    pub index_name: String,
    pub index_sec: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Users {
    pub user_name: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCategory {
    pub category_name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RegisterUsers {
    pub user_names: String,
    pub passwords: String,
    pub repeat_passwords: String,
}

#[derive(Template)]
#[template(path = "blogs.html")]
pub struct BlogTemplate<'a> {
    pub post_id_title: BTreeMap<i32, String>,
    pub category_id_title: BTreeMap<i32, String>,
    pub index_id: &'a Vec<i32>,
    pub index_title: String,
    pub page_number: &'a i32,
    pub category_name: &'a String,
    pub index_links: &'a Vec<String>,
    pub index_sec: &'a Vec<String>,
    pub page_nav_links: &'a Vec<i32>,
    pub current_url_page: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct Blog {
    pub post_id: i32,
    pub post_title: String,
    pub post_description: String,
    pub post_body: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct Category_Id {
    pub category_id: i32,
}

#[derive(FromRow, Debug, Clone)]
pub struct Post_Name {
    pub post_title: String,
}

#[derive(FromRow, Debug, Clone)]
pub struct Category_Name {
    pub category_name: String,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub post_id_title: BTreeMap<i32, String>,
    pub category_id_title: BTreeMap<i32, String>,
    pub index_id: &'a Vec<i32>,
    pub index_title: String,
    pub page_number: &'a i32,
    pub selected_category: &'a String,
    pub index_links: &'a Vec<String>,
    pub index_sec: &'a Vec<String>,
    pub page_nav_links: &'a Vec<i32>,
}

#[derive(Template)]
#[template(path = "categories.html")]
pub struct CategoryTemplate<'a> {
    pub category_id_title: BTreeMap<i32, String>,
    pub index_id: &'a Vec<i32>,
    pub category_id: &'a Vec<i32>,
    pub index_title: String,
    pub index_links: &'a Vec<String>,
    pub index_sec: &'a Vec<String>,
    pub page_nav_links: &'a Vec<String>,
}

#[derive(Template)]
#[template(path = "category_pagination.html")]
pub struct CategoryTemplatePagination<'a> {
    pub category_id_title: BTreeMap<i32, String>,
    pub index_id: &'a Vec<i32>,
    pub category_id: &'a Vec<i32>,
    pub index_title: String,
    pub index_links: &'a Vec<String>,
    pub index_sec: &'a Vec<String>,
    pub page_nav_links: &'a Vec<String>,
}

pub async fn get_connection() -> std::result::Result<Vec<Post>, Error> {
    let pool = get_connection_for_crud().await;
    sqlx::query_as::<_, Post>(
        "select post_id,post_title, post_body, post_description from posts limit 3 offset 0",
    )
    .fetch_all(&pool)
    .await
}

pub async fn get_posts_per_page(offset_value: i32) -> std::result::Result<Vec<Post>, Error> {
    let pool = get_connection_for_crud().await;
    sqlx::query_as::<_, Post>(
        "select post_id, post_title, post_body, post_description from posts limit ($1) offset ($2)",
    )
    .bind(global_number_of_items_per_page())
    .bind(offset_value)
    .fetch_all(&pool)
    .await
}

pub async fn get_categories_per_page(
    offset_value: i32,
) -> std::result::Result<Vec<Category>, Error> {
    let pool = get_connection_for_crud().await;
    sqlx::query_as::<_, Category>(
        "select * from category_post limit ($1) offset ($2) ORDER BY category_id ASC",
    )
    .bind(global_number_of_items_per_page())
    .bind(offset_value)
    .fetch_all(&pool)
    .await
}

pub async fn get_all_categories() -> std::result::Result<Vec<Category>, Error> {
    let pool = get_connection_for_crud().await;

    sqlx::query_as::<_, Category>("select * from category_post ORDER BY category_id ASC")
        .fetch_all(&pool)
        .await
}

pub async fn get_category_id_by_name(category_name: String) -> Vec<Category_Id> {
    let pool = get_connection_for_crud().await;
    let res = sqlx::query_as::<_, Category_Id>(
        "select category_id from category_post where category_name = ($1);",
    )
    .bind(category_name)
    .fetch_all(&pool)
    .await;
    res.unwrap()
}

pub async fn get_post_name_by_id(post_id: i32) -> Vec<Post_Name> {
    let pool = get_connection_for_crud().await;
    let res = sqlx::query_as::<_, Post_Name>("select post_title from posts where post_id = ($1);")
        .bind(post_id)
        .fetch_all(&pool)
        .await;
    res.unwrap()
}

pub async fn get_category_name_by_id(category_id: i32) -> Vec<Category_Name> {
    let pool = get_connection_for_crud().await;
    let res = sqlx::query_as::<_, Category_Name>(
        "select category_name from category_post where category_id = ($1);",
    )
    .bind(category_id)
    .fetch_all(&pool)
    .await;
    res.unwrap()
}

pub async fn get_category_name_by_post_id(post_name: String) -> Vec<Category_Name> {
    let pool = get_connection_for_crud().await;
    let res = sqlx::query_as::<_, Category_Name>(
        "select category_name from category_post c,posts p where c.category_id = p.category_id and post_title = ($1);",
    )
        .bind(post_name)
        .fetch_all(&pool)
        .await;
    res.unwrap()
}

pub async fn get_all_categories_with_limit() -> std::result::Result<Vec<Category>, Error> {
    let pool = get_connection_for_crud().await;

    sqlx::query_as::<_, Category>("select * from category_post limit 3")
        .fetch_all(&pool)
        .await
}

pub async fn get_details_of_post(post_id: i32) -> std::result::Result<Vec<Post>, Error> {
    let pool = get_connection_for_crud().await;

    sqlx::query_as::<_, Post>(
        "select post_id, post_title, post_body, post_description from posts where post_id = ($1)",
    )
    .bind(post_id)
    .fetch_all(&pool)
    .await
}

pub async fn get_max_id_of_post() -> std::result::Result<Vec<Max>, Error> {
    let pool = get_connection_for_crud().await;

    sqlx::query_as::<_, Max>("select max(post_id) from posts;")
        .fetch_all(&pool)
        .await
}

pub async fn get_max_id_of_category() -> std::result::Result<Vec<Max>, Error> {
    let pool = get_connection_for_crud().await;

    sqlx::query_as::<_, Max>("select max(category_id) from category_post;")
        .fetch_all(&pool)
        .await
}

pub async fn get_count_of_posts() -> std::result::Result<Vec<Count>, Error> {
    let pool = get_connection_for_crud().await;

    sqlx::query_as::<_, Count>("select count(*) from posts;")
        .fetch_all(&pool)
        .await
}

pub async fn get_count_of_categories() -> std::result::Result<Vec<Count>, Error> {
    let pool = get_connection_for_crud().await;

    sqlx::query_as::<_, Count>("select count(*) from category_post;")
        .fetch_all(&pool)
        .await
}

pub async fn get_filtered_from_database(
    final_category: i32,
    page_number: i32,
) -> std::result::Result<Vec<Blog>, Error> {
    let pool = get_connection_for_crud().await;
    sqlx::query_as::<_, Blog>("select p.post_id, p.post_title, p.post_description, p.post_body, c.category_id, c.category_name from posts p, category_post c where p.category_id=c.category_id and c.category_id = ($1) limit 3 offset ($2)")
        .bind(final_category)
        .bind(page_number)
        .fetch_all(&pool)
        .await
}

pub async fn get_filtered_from_database_by_category(
    final_category: i32,
) -> std::result::Result<Vec<Blog>, Error> {
    let pool = get_connection_for_crud().await;
    sqlx::query_as::<_, Blog>("select p.post_id, p.post_title, p.post_description, p.post_body, c.category_id, c.category_name from posts p, category_post c where p.category_id=c.category_id and c.category_id = ($1) limit 3")
        .bind(final_category)
        .fetch_all(&pool)
        .await
}
pub async fn get_filtered_from_database_by_category2() -> std::result::Result<Vec<Blog>, Error> {
    let pool = get_connection_for_crud().await;
    sqlx::query_as::<_, Blog>("select p.post_id, p.post_title, p.post_description, p.post_body, c.category_id, c.category_name from posts p, category_post c where p.category_id=c.category_id limit 3")

        .fetch_all(&pool)
        .await
}
pub async fn count_of_get_filtered_from_database_by_category(
    final_category: i32,
) -> std::result::Result<Vec<Count>, Error> {
    let pool = get_connection_for_crud().await;
    sqlx::query_as::<_, Count>("select count(p.post_id) from posts p, category_post c where p.category_id=c.category_id and c.category_id = ($1)")
        .bind(final_category)
        .fetch_all(&pool)
        .await
}
pub async fn count_of_get_filtered_from_database_by_category2(
) -> std::result::Result<Vec<Count>, Error> {
    let pool = get_connection_for_crud().await;
    sqlx::query_as::<_, Count>("select count(p.post_id) from posts p; ")
        .fetch_all(&pool)
        .await
}
