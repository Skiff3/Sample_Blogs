use std::sync::Arc;
// This controller contains the CRUD operations of posts
// Create, Read, Update and Delete method for posts.
use crate::model::models::{Blog, Count, get_all_categories, get_connection, get_count_of_posts, get_max_id_of_category, get_max_id_of_post, HomeTemplate, Max, NewCategoryTemplate, NewPostTemplate, UpdateCategory, UpdateCategoryTemplate};
use crate::{global_number_of_items_per_page_64, CreateCategory, CreatePost, UpdatePost};
use askama::Template;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};

pub async fn get_connection_for_crud() -> Pool<Postgres> {
   PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://sakibbagewadi:Sakib123@localhost/blog_temp")
        .await
        .expect("failed to connect")

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
    let post_id = (get_max(m)) + 1; //insert into posts(post_id,post_title,post_body,category_id) values (16,'random','random',4);
    let res = sqlx::query("insert into posts(post_id,post_title,post_body,category_id,post_description) values (($1),($2),($3),($4),($5))")
        .bind(post_id)// the id of post
        .bind(create_post.post_title)
        .bind(create_post.post_body)
        .bind(category_id)
        .bind("none")
        .execute(&pool)
        .await;

    println!("Success ---> row {:?}", &res);
    Redirect::to("/admin/page/1")
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
    Redirect::to("/admin/page/1")
}

pub async fn delete_categories_form(Path(category_id): Path<String>) -> Redirect {
    let pool = get_connection_for_crud().await;
    //let mut post_ids = post_id.clone().parse().u;
    let res = sqlx::query("delete from category_post where category_name = ($1)")
        .bind(category_id)
        .execute(&pool)
        .await;

    println!("Success ---> row {:?}", &res);
    Redirect::to("/admin/page/1")
}

pub async fn home_gui() -> impl IntoResponse {
    let mut psec: Vec<String> = Vec::new();
    psec.clear();

    let category_list = get_all_categories().await;
    let _list_iters = category_list.iter();
    psec.clear();
    //let mut psec: Vec<String> = Vec::new();
    psec.push("Category A".to_string()); // psec.push("Category A")
    psec.push("Category B".to_string());
    psec.push("Category C".to_string());
    psec.push("No Category".to_string());
    let s = get_connection().await;
    // let number_of_pages: i64;
    let mut plinks: Vec<String> = Vec::new();
    let mut pids: Vec<i32> = Vec::new();
    let mut pnav: Vec<String> = Vec::new();
    let number_of_posts_vector = get_count_of_posts().await;
    let _m = number_of_posts_vector;
    let number_of_pages: i64 = if get_vec_len_of_count(get_count_of_posts().await) % global_number_of_items_per_page_64() == 0 {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64()
    } else {
        get_vec_len_of_count(get_count_of_posts().await) / global_number_of_items_per_page_64() + 1
    };
    for i in 1..number_of_pages + 1 {
        pnav.push(i.to_string())
    }
    let list_iter = s.map(|posts| {
        //plinks = posts.iter()
        //.map(|post| {post.post_title.clone()}).collect();
        let v: Vec<_> = posts.iter()
            .map(|post| {post.post_title.clone()}).collect();
        let v2: Vec<_> = posts.iter()
            .map(|post| {post.post_id.clone()}).collect();

        (v,v2)
    });

    (plinks,pids) = list_iter.unwrap_or_default();

    let template = HomeTemplate {
        index_id: &pids,
        index_title: String::from("Blogs"),
        index_links: &plinks,
        index_sec: &psec,
        page_nav_links: &pnav,
        current_url_page: ".".to_string(),
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
    let m2 = get_max(m);
    let category_id =m2+1;
    let res =
        sqlx::query("insert into category_post(category_id,category_name) values (($1),($2))")
            .bind(category_id) // category id
            .bind(create_category.category_name)
            .execute(&pool)
            .await;

    println!("Success ---> row {:?}", &res);
    Redirect::to("/admin/page/1")
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
            .into_response(),
    }
}

pub async fn update_category_form_ui(Path(category_id): Path<String>) -> impl IntoResponse {
    let template = UpdateCategoryTemplate { index_sec: category_id };

    match template.render() {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            StatusCode::UNAUTHORIZED,
            format!("Failed to render template. Error {}", err),
        )
            .into_response(),
    }
}

pub async fn update_category_form(Path(category_id): Path<String>,Form(update_category): Form<UpdateCategory>,) -> Redirect {
    let pool = get_connection_for_crud().await;
    println!("category {} {}",category_id,update_category.category_name);
    let res =
        sqlx::query("update category_post set category_name = ($1) where category_name = ($2)")
            .bind(update_category.category_name)
            .bind(category_id)
            .execute(&pool)
            .await;

    println!("Success ---> row {:?}", &res);
    Redirect::to("/posts")
}


pub fn get_vec_len(shared_state2: Arc<Result<Vec<Blog>, Error>>) -> i64{
    let temp = shared_state2.as_ref().as_ref();
    let mut len:i64=0;
    let _iter = temp.map(|posts| {
        len = posts.len() as i64;
    });
    len
}
pub fn get_vec_len_of_count(shared_state2: Result<Vec<Count>, Error>) -> i64{
    let mut len1:i64=15;
    let temp = shared_state2.as_ref();
    let _iter= temp.map(|posts| {
        posts.iter().map(|count| {
            len1= count.count
        })

    });
    len1
}

pub fn get_max(shared_state2: Result<Vec<Max>, Error>) -> i32{
    let mut len2:i32 = 0;
    let iters= shared_state2.map(|posts| {
        let _ = posts.iter().map( |count| {
            len2 = count.max;
        });
        len2
    });
    drop(iters);
    len2
}