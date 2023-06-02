mod model;
mod controllers;
mod authentication;

use crate::controllers::filter_navigate::blog_pagination;
use crate::controllers::navigate::page;
use crate::model::models::{BlogTemplate, IndexTemplate, Post, Blog, PostTemplate, get_connection, LoginTemplate, Users, Pagination, Count, get_max_id_of_post, get_max_id_of_category, NewPostTemplate, NewCategoryTemplate};
use crate::controllers::index::index;
use crate::controllers::filter_post::blogs;
use tower_http::services::ServeDir;
use std::sync::Arc;
use askama::{DynTemplate, Template};
use core::fmt::Write;
use axum::http::{Request, StatusCode};
use axum::extract::{OriginalUri, Path};
use std::{collections::HashMap};
use std::arch::asm;
use std::io::Read;
use std::time::Duration;
use axum::routing::post;
use axum::{response::IntoResponse, routing::{get,delete}, Extension, Router, Form};
use axum::handler::Handler;
use axum::response::{Html, Redirect};
use axum::routing::any;
use axum_login::{
    axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer},
    memory_store::MemoryStore as AuthMemoryStore,
    secrecy::SecretVec,
    AuthLayer, AuthUser, RequireAuthorizationLayer,
};
use axum_login::axum_sessions::async_session::SessionStore;
use axum_login::secrecy::{ExposeSecret, Secret};
use axum_macros::debug_handler;
use rand::Rng;
use tokio::sync::RwLock;
use sqlx::Postgres;
use crate::controllers::controller_post::show_post;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use crate::authentication::login::{admin_gui, login_user, login_user_ui, register_user, register_user_ui};
use crate::controllers::posts_crud_controller::{create_category_form_ui, create_catgories_form, create_posts_form, create_posts_form_ui, delete_posts_form, update_posts_form};


#[derive(Deserialize)]
pub struct CreatePost {
   // pub post_id: i32,
    pub post_title: String,
    pub post_body: String,
    pub category_name: String,
}

#[derive(Deserialize)]
pub struct CreateCategory {
    // pub post_id: i32,
    pub category_name: String,
}

#[derive(Deserialize)]
pub struct UpdatePost {
    // pub post_id: i32,
    pub post_title: String,
    pub post_body: String,
}

#[derive(Deserialize)]
pub struct DeletePost {
    // pub post_id: i32,
    pub post_title: String,
}

#[derive(Debug, Clone,Default)]
pub struct User {
    id: i64,
    name: String,
    password_hash: String,
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub user_name: String,
    pub password: String,
}



pub fn global_number_of_items_per_page() -> i32 {
    3
}

pub fn global_number_of_items_per_page_64() -> i64 {
    3
}

impl User {
    fn get_rusty_user() -> Self {
        Self {
            id: 2,
            name: "Manny".to_string(),
            password_hash: "Password".to_string(),
        }
    }
}

impl AuthUser<i64> for User {
    fn get_id(&self) -> i64 {
        self.id
    }

 fn get_password_hash(&self) -> SecretVec<u8> {
        let sec = SecretVec::new(self.password_hash.clone().into());
        sec
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[allow(dead_code)]
enum Role {
    User,
    Admin,
}

type AuthContext = axum_login::extractors::AuthContext<i64,User,AuthMemoryStore<i64,User>>;



#[tokio::main]
async fn main() {

    let secret = rand::thread_rng().gen::<[u8; 64]>();

    let session_store = SessionMemoryStore::new();
    let session_timeout_duration = Some(Duration::new(600,0));
    let session_layer = SessionLayer::new(session_store, &secret)
        .with_secure(false)
        .with_cookie_name("user")
        .with_session_ttl(session_timeout_duration);
    let store = Arc::new(RwLock::new(HashMap::default()));

    let user = User::get_rusty_user();
    store.write().await.insert(user.get_id(), user.clone());

    let user_store = AuthMemoryStore::new(&store);
    let auth_layer = AuthLayer::new(user_store, &secret);// auth layer


    let mut user_vector: Vec<User> = Vec::new();
    let mut user1:User = User {
        id: 2,
        name: "Manny".to_string(),
        password_hash: "password".to_string(),
    };
    user_vector.push(user1);
    let user_state = Arc::new(user_vector);
    // Password - \x50617373776f7264 MannyP - \x4d616e6e7950

    async fn logout_handler(mut auth: AuthContext) -> Redirect {
        dbg!("Logging out user: {}", &auth.current_user);
        println!("Logged out ---> {:?}",&auth.current_user);
        auth.logout().await;
        Redirect::to("/login")// logout handler logs out the user to the home page.
    }

    #[debug_handler]
    pub async fn protected_handler(Extension(user_state): Extension<User>) -> impl IntoResponse {
        println!("Logged in router {:?}",user_state.name);
        format!("Logged in as: {}, ---> {}", user_state.name,user_state.password_hash)
    }

    async fn admin_handler(Extension(user): Extension<User>) -> impl IntoResponse {
        format!("Logged in as admin: {}", user.name)// user is admin
    }

    let mut posts = get_connection().await;
    let shared_state = Arc::new(posts);


    let blog_routes = Router::new()// reg log in log out email pass user table - username, email, pass - encrypted.  crates.
        .route("/posts/category/:category",get(blogs))
        .route("/posts/category/:category/pages/:page_number", get(blog_pagination));

     let app = Router::new()
        .route("/posts", get(index))
        .route("/post/:post_id", get(show_post))
        .route("/page/:page_number", get(page))
        .with_state(shared_state)
        .merge(blog_routes)
        .route("/post/new",get(create_posts_form_ui).post(create_posts_form))
        .route("/delete/:post_id",get(delete_posts_form))
        .route("/update_post/:post_id",post(update_posts_form))
        .route("/category/new",get(create_category_form_ui).post(create_catgories_form))
         .route("/admins", get(admin_gui))
        //.route_layer(RequireAuthorizationLayer::<i64,User>::login())
         .route("/register",post(register_user))
         .route("/register/new",get(register_user_ui))
        .route("/login", get(login_user_ui).post(login_user))
        .route("/logout", get(logout_handler))
         .layer(Extension(user.clone()))
        .layer(auth_layer)
        .layer(session_layer)
        .nest_service("/assets", ServeDir::new("assets/css"));

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}
