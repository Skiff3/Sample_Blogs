
use askama::Template;
use sqlx::*;

mod filters {

    pub fn rmdashes(title: &str) -> askama::Result<String> {
        Ok(title.replace("-", " ").into())
    }
}

#[derive(FromRow,Debug, Clone)]
pub struct Post {
    pub post_title: String,
    pub post_description: String,
    pub post_body: String,
}

#[derive(Template)]
#[template(path = "posts.html")]
pub struct PostTemplate<'a> {
    pub  post_title: &'a str,
    pub post_description: &'a str,
    pub post_body: &'a str,
}


#[derive(Template)]
#[template(path = "blogs.html")]
pub struct BlogTemplate<'a> {
    pub index_title: String,
    pub index_links: &'a Vec<String>,
    pub index_sec: &'a Vec<String>,
    pub page_nav_links: &'a Vec<String>,
}


#[derive(FromRow,Debug, Clone)]
pub struct Blog {
    pub post_title: String,
    pub post_description: String,
    pub post_body: String,
}


#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub index_title: String,
    pub index_links: &'a Vec<String>,
    pub index_sec: &'a Vec<String>,
    pub page_nav_links: &'a Vec<String>,
}