use crate::global_number_of_items_per_page;
use askama::Template;
use axum_login::axum_sessions::async_session::serde::Deserialize;
use std::collections::{BTreeMap, HashMap};

use crate::controllers::base_controller::get_connection_for_crud;
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
    pub post_body: &'a str,
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

