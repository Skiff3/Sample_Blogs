extern crate core;

mod authentication;
mod controllers;
mod model;

use crate::authentication::login::{
    admin_gui, login_user, login_user_ui, register_user, register_user_ui,
};
use crate::controllers::controller_post::{show_post, show_posts};
use crate::controllers::filter_navigate::{admin_blog_pagination, blog_pagination};
use crate::controllers::filter_post::{admin_blogs, blogs};
use crate::controllers::index::index;
use crate::controllers::navigate::{page, pages};
use crate::controllers::posts_crud_controller::{
    create_category_form_ui, create_catgories_form, create_posts_form, create_posts_form_ui,
    delete_categories_form, delete_posts_form, home_gui, show_all_categories,
    show_all_categories_with_pagination, update_category_form, update_category_form_ui,
    update_posts_form,
};
use crate::model::models::{BlogTemplate, IndexTemplate};
use axum::response::Redirect;
use axum::routing::post;
use axum::{routing::get, Extension, Router};
use axum_login::{axum_sessions::{async_session::MemoryStore as SessionMemoryStore, SessionLayer}, memory_store::MemoryStore as AuthMemoryStore, secrecy::SecretVec, AuthLayer, AuthUser, RequireAuthorizationLayer};
use rand::Rng;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
pub struct CreatePost {
    pub post_title: String,
    pub post_body: String,
    pub category_name: String,
}

#[derive(Deserialize)]
pub struct CreateCategory { 
    pub category_name: String,
}

#[derive(Deserialize)]
pub struct UpdatePost {
    pub post_title: String,
    pub post_body: String,
    pub category_name: String,
}

#[derive(Deserialize)]
pub struct DeletePost {
    pub post_title: String,
}

#[derive(Debug, Clone, Default)]
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
        SecretVec::new(self.password_hash.clone().into())
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[allow(dead_code)]
enum Role {
    User,
    Admin,
}

type AuthContext = axum_login::extractors::AuthContext<i64, User, AuthMemoryStore<i64, User>>;

#[tokio::main]
async fn main() -> std::result::Result<(), sqlx::Error> {
    let secret = rand::thread_rng().gen::<[u8; 64]>();
    let session_store = SessionMemoryStore::new();
    let session_timeout_duration = Some(Duration::new(600, 0));
    let session_layer = SessionLayer::new(session_store, &secret)
        .with_secure(false)
        .with_cookie_name("user")
        .with_session_ttl(session_timeout_duration);
    let store = Arc::new(RwLock::new(HashMap::default()));

    let user = User::get_rusty_user();
    store.write().await.insert(user.get_id(), user.clone());

    let user_store = AuthMemoryStore::new(&store);
    let auth_layer = AuthLayer::new(user_store, &secret);

    let mut user_vector: Vec<User> = vec![];
    let user1: User = User {
        id: 2,
        name: "Manny".to_string(),
        password_hash: "password".to_string(),
    };
    user_vector.push(user1);
    async fn logout_handler(mut auth: AuthContext) -> Redirect {
        auth.logout().await;
        Redirect::to("/login")
    }

    let admin_blog_routes = Router::new()
        .route("/admin/posts/category/:category", get(admin_blogs))
        .route(
            "/admin/posts/category/:category/pages/:page_number",
            get(admin_blog_pagination),
        );

    let blog_routes = Router::new()
        .route("/posts/category/:category", get(blogs))
        .route(
            "/posts/category/:category/pages/:page_number",
            get(blog_pagination),
        );

    let app = Router::new()
        .route("/admin", get(index))
        .route("/admin/categories", get(show_all_categories))
        .route(
            "/admin/categories/:page_number",
            get(show_all_categories_with_pagination),
        )
        .route("/post/:post_id", get(show_post))
        .route("/admin/page/:page_number", get(page))
        .merge(admin_blog_routes)
        .route(
            "/admin/post/new",
            get(create_posts_form_ui).post(create_posts_form),
        )
        .route("/delete/:post_id", get(delete_posts_form))
        .route("/delete/category/:category_id", get(delete_categories_form))
        .route(
            "/admin/category/update/:category_id",
            get(update_category_form_ui).post(update_category_form),
        )
        .route("/admin/update_post/:post_id", post(update_posts_form))
        .route(
            "/admin/category/new",
            get(create_category_form_ui).post(create_catgories_form),
        )
        .route("/admins", get(admin_gui))
        //.route_layer(RequireAuthorizationLayer::<i64, User>::login())
        .route("/", get(home_gui))
        .route("/posts/page/:page_number", get(pages))
        .merge(blog_routes)
        .route("/register", post(register_user))
        .route("/register/new", get(register_user_ui))
        .route("/login", get(login_user_ui).post(login_user))
        .route("/logout", get(logout_handler))
        .route("/posts/:post_id", get(show_posts))
        //.route("/post/main", get(create_guest_post_ui))
        .layer(Extension(user.clone()))
        .layer(auth_layer)
        .layer(session_layer)// session body
        .nest_service("/assets", ServeDir::new("assets"));

    axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
        .serve(app.into_make_service())
        .await;

    Ok(())
    /*
    change urls
    database column to null
    from name to id
     */
}
