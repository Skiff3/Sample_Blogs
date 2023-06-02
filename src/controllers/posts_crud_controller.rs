// This controller contains the CRUD operations of posts
// Create, Read, Update and Delete method for posts.
use askama::Template;
use axum::extract::Path;
use axum::Form;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::{CreateCategory, CreatePost, UpdatePost};
use crate::model::models::{get_connection, get_max_id_of_category, get_max_id_of_post, NewCategoryTemplate, NewPostTemplate};

pub async fn get_connection_for_crud() -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
        .await
        .expect("couldn't connect to the database");
    pool
}

pub async fn create_posts_form_ui() -> impl IntoResponse {
    let template = NewPostTemplate {};

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        ).into_response(),
    }
}

pub async fn create_posts_form(Form(create_post): Form<CreatePost>) -> impl IntoResponse {
    let pool = get_connection_for_crud().await;
    println!("Form {}", create_post.post_title);
    println!("Hello");
    let mut category_id = 0;
    if create_post.category_name.eq("Category A") { category_id = 1;}//
    else if create_post.category_name.eq("Category B") { category_id = 2;}
    else if create_post.category_name.eq("Category C") { category_id = 3;}
    else { category_id = 4;}
    let m = get_max_id_of_post().await;
    let post_id = (m[0].max)+1;//insert into posts(post_id,post_title,post_body,category_id) values (16,'random','random',4);
    let mut res = sqlx::query("insert into posts(post_id,post_title,post_body,category_id,post_description) values (($1),($2),($3),($4),($5))")
        .bind(post_id)// the id of post
        .bind(create_post.post_title)
        .bind(create_post.post_body)
        .bind(category_id)
        .bind("none")
        .execute(&pool)
        .await;

    println!("Success ---> row {:?}",&res);
    Redirect::to("/page/1")//posts
}

pub async fn delete_posts_form(Path(post_id): Path<String>) -> Redirect {
    let pool = get_connection_for_crud().await;
    println!("Form {}", post_id);
    //let mut post_ids = post_id.clone().parse().u;
    let mut res = sqlx::query("delete from posts where post_title = ($1)")
        .bind(post_id)
        .execute(&pool)
        .await;

    println!("Success ---> row {:?}",&res);
    Redirect::to("/page/1")
}

pub async fn create_catgories_form(Form(create_category): Form<CreateCategory>) -> Redirect {
    let pool = get_connection_for_crud().await;
    let m = get_max_id_of_category().await;
    let mut category_id=m[0].max+1;
    let mut res = sqlx::query("insert into category_post(category_id,category_name) values (($1),($2))")
        .bind(category_id)// category id
        .bind(create_category.category_name)
        .execute(&pool)
        .await;

    println!("Success ---> row {:?}",&res);
    Redirect::to("/")
}

pub async fn update_posts_form(Path(post_id): Path<String>,Form(update_post):Form<UpdatePost>) -> Redirect{
    let pool = get_connection_for_crud().await;
    println!("Form {}", update_post.post_title);
    let mut res = sqlx::query("update posts set post_title = ($1), post_body = ($2) where post_title = ($3)")
        .bind(update_post.post_title)
        .bind(update_post.post_body)
        .bind(post_id)
        .execute(&pool)
        .await;

    println!("Success ---> row {:?}",&res);
    Redirect::to("/posts")
}

pub async fn create_category_form_ui() -> impl IntoResponse {
    let template = NewCategoryTemplate {};

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::UNAUTHORIZED,
            format!("Failed to render template. Error {}", err),
        ).into_response(),// which is the most important
    }
}