
// This controller contains the CRUD operations of posts
// Create, Read, Update and Delete method for posts.
use crate::model::models::{get_all_categories, get_connection, get_count_of_posts, get_max_id_of_category, get_max_id_of_post, HomeTemplate, NewCategoryTemplate, NewPostTemplate};
use crate::{CreateCategory, CreatePost, global_number_of_items_per_page_64, UpdatePost};
use askama::Template;
use axum::extract::{Path};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn get_connection_for_crud() -> Pool<Postgres> {
        PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
        .await
        .expect("couldn't connect to the database")

}

pub async fn create_posts_form_ui() -> impl IntoResponse {
    let template = NewPostTemplate {};

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}

pub async fn create_posts_form(Form(create_post): Form<CreatePost>) -> impl IntoResponse {
    let pool = get_connection_for_crud().await;
    println!("Form {}", create_post.post_title);
    println!("Hello");
    let category_id;
    if create_post.category_name.eq("Category A") {
        category_id = 1;
    }
    //
    else if create_post.category_name.eq("Category B") {
        category_id = 2;
    } else if create_post.category_name.eq("Category C") {
        category_id = 3;
    } else {
        category_id = 4;
    }
    let m = get_max_id_of_post().await;
    let post_id = (m[0].max) + 1; //insert into posts(post_id,post_title,post_body,category_id) values (16,'random','random',4);
    let res = sqlx::query("insert into posts(post_id,post_title,post_body,category_id,post_description) values (($1),($2),($3),($4),($5))")
        .bind(post_id)// the id of post
        .bind(create_post.post_title)
        .bind(create_post.post_body)
        .bind(category_id)
        .bind("none")
        .execute(&pool)
        .await;

    println!("Success ---> row {:?}", &res);
    Redirect::to("/page/1")
}

pub async fn delete_posts_form(Path(post_id): Path<String>) -> Redirect {
    let pool = get_connection_for_crud().await;
    println!("Form {}", post_id);
    //let mut post_ids = post_id.clone().parse().u;
    let res = sqlx::query("delete from posts where post_title = ($1)")
        .bind(post_id)
        .execute(&pool)
        .await;

    println!("Success ---> row {:?}", &res);
    Redirect::to("/page/1")
}

pub async fn home_gui() -> impl IntoResponse {
    let mut psec: Vec<String> = Vec::new();
    psec.clear();

    let category_list = get_all_categories().await;
    let list_iters = category_list.iter();
    for i in list_iters {
        psec.push(i.category_name.clone());
    }
    let s = get_connection().await;
    // let number_of_pages: i64;
    let mut plinks: Vec<String> = Vec::new();
    let mut pids: Vec<i32> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    let number_of_posts_vector = get_count_of_posts().await;
    let m = number_of_posts_vector;
    let number_of_pages: i64 = if m[0].count % global_number_of_items_per_page_64() == 0 {
        (m[0].count) / global_number_of_items_per_page_64()
    } else {
        (m[0].count) / global_number_of_items_per_page_64() + 1
    };
    println!(
        "the number of pages are {}, count of posts {}",
        number_of_pages, m[0].count
    );
    for i in 1..number_of_pages + 1 {
        pnav.push(i.to_string())
    }

    for i in 0..s.len() {
        plinks.push(s[i].post_title.clone());
        pids.push(s[i].post_id);
        println!("{}", s.len()) // prints the s length
    }

    let template = HomeTemplate {
        index_id: &pids,
        index_title: String::from("Blogs"),
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
    };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}

pub async fn create_catgories_form(Form(create_category): Form<CreateCategory>) -> Redirect {
    let pool = get_connection_for_crud().await;
    let m = get_max_id_of_category().await;
    let category_id = m[0].max + 1;
    let res =
        sqlx::query("insert into category_post(category_id,category_name) values (($1),($2))")
            .bind(category_id) // category id
            .bind(create_category.category_name)
            .execute(&pool)
            .await;

    println!("Success ---> row {:?}", &res);
    Redirect::to("/")
}

pub async fn update_posts_form(
    Path(post_id): Path<String>,
    Form(update_post): Form<UpdatePost>,
) -> Redirect {
    let pool = get_connection_for_crud().await;
    println!("Form {}", update_post.post_title);
    let res =
        sqlx::query("update posts set post_title = ($1), post_body = ($2) where post_title = ($3)")
            .bind(update_post.post_title)
            .bind(update_post.post_body)
            .bind(post_id)
            .execute(&pool)
            .await;

    println!("Success ---> row {:?}", &res);
    Redirect::to("/posts")
}

pub async fn create_category_form_ui() -> impl IntoResponse {
    let template = NewCategoryTemplate {};

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::UNAUTHORIZED,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(), // which is the most important
    }
}
