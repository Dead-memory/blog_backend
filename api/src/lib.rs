use axum::{
    extract::{Form, Path, Query, State},
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use blog_backend_core::{
    sea_orm::{Database, DatabaseConnection},
    Mutation as MutationCore, Query as QueryCore
};
use migration::{Migrator, MigratorTrait};
use sea_orm::DbConn;
use serde::{Deserialize, Serialize};
use std::{env, net::SocketAddr, str::FromStr};
use dotenvy::dotenv;

use tower_cookies::{Cookie, CookieManagerLayer, Cookies};

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {}", err);
    }
}

const API_URL: &'static str = "/api/v1_dev";

#[derive(Clone)]
struct AppState {
    conn: DatabaseConnection
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = match env::var("HOST")
        .expect("HOST is not set in .env file")
        .to_lowercase()
        .as_str() 
    {
        "localhost" => String::from("127.0.0.1"),
        x => x.to_string()
    };
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let db_conn = Database::connect(db_url)
        .await.expect("Database conneciton failed");
    Migrator::up(&db_conn, None).await.unwrap();

    let state = AppState { 
        conn: db_conn 
    };

    // build our application with a route
    let app = Router::new()
        .route(format!("{}{}", API_URL, "/user/:id").as_str(), get(get_user))
        .route(format!("{}{}", API_URL, "/auth/login").as_str(), post(login))
        .route(format!("{}{}", API_URL, "/auth/logout").as_str(), get(logout))
        .route(format!("{}{}", API_URL, "/test").as_str(), get(test_fn))
        .route(format!("{}{}", API_URL, "/article").as_str(), get(get_article))
        .with_state(state)
        .layer(CookieManagerLayer::new());

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from_str(&server_url).unwrap();
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// basic handler that responds with a static string
async fn get_user(
    state: State<AppState>,
    Path(id): Path<i32>
) -> Result<String, (StatusCode, &'static str)> {
    let user = QueryCore::find_user_by_id(&state.conn, id)
        .await
        .expect("Could not find user");

    match user {
        Option::Some(x) => Result::Ok(x.pseudo),
        Option::None => Result::Err((StatusCode::NOT_FOUND, "Utilisateur non trouvé"))
    }
}

const COOKIE_TOKEN_NAME: &'static str = "TOKEN";

#[derive(Deserialize)]
struct Login {
    username: String,
    password: String
}

async fn login(
    state: State<AppState>,
    mut cookies: Cookies,
    form: Form<Login>
) -> &'static str {
    let users = QueryCore::find_user_by_username(&state.conn, &form.username).await.unwrap();

    for user in users {
        if user.hashed_password == form.password {
            cookies.add(Cookie::new(COOKIE_TOKEN_NAME, "Token caca"));
            return "ok";
        }
    }

    "Nop"
}

async fn logout(
    state: State<AppState>,
    cookies: Cookies
) -> StatusCode {

    let tmp_local_cookie = cookies.get(COOKIE_TOKEN_NAME);
    if let Option::None = tmp_local_cookie  {
        return StatusCode::UNPROCESSABLE_ENTITY;
    }
    let local_cookie = tmp_local_cookie.unwrap();

    let token = local_cookie.value().to_string();
    cookies.remove(Cookie::new(COOKIE_TOKEN_NAME, ""));

    if let Result::Ok(x) = QueryCore::remove_session_by_token(&state.conn, token).await {
        if x.rows_affected >= 1 {
            StatusCode::OK
        } else {
            StatusCode::UNPROCESSABLE_ENTITY
        }
    } else {
        StatusCode::UNPROCESSABLE_ENTITY
    }
}

async fn token_verify(
    db: &DbConn,
    cookies: Cookies
) -> Option <entity::user::Model> {
    let token = &cookies
        .get(COOKIE_TOKEN_NAME);

    println!("{:?}", token);

    match token {
        Some(x) => {
            QueryCore::find_user_by_token(
                &db, 
                &x
                    .value()
                    .to_string()
            ).await.unwrap()
        },
        None => Option::None
    }
}

async fn test_fn (
    state: State<AppState>,
    cookies: Cookies
) -> String {
    match token_verify(&state.conn, cookies).await {
        Option::Some(x) => format!("Vous êtes {}", x.pseudo),
        Option::None => "Not".to_string()
    }
}

// Articles
mod serialization_struct;
use serialization_struct::article::Article;
use serialization_struct::tags::Tag;

async fn get_article (
    state: State<AppState>
) -> String {
    serde_json::to_string(&Article {
        id: 1,
   
        author_id: 1,
        author_pseudo: "Dofe".to_string(),
    
        title: "Safety".to_string(),
        content: "?".to_string(),
    
        creation_date: "20221216".to_string(),
        tags: vec![
            Tag {
                id: 1,

                name: "Rust".to_string(),
                description: "Programming language".to_string()
            }
        ],
        picture_url: "https://media.istockphoto.com/id/1413025608/fr/vectoriel/premier-signe-de-s%C3%A9curit%C3%A9-jaune-signe-de-style-isol%C3%A9-sur-fond-blanc-illustration.jpg?s=612x612&w=0&k=20&c=IPhgHvg06Mq2_mz53uUjpmvJ-f1KgD58lqzJZkzzfhk=".to_string()
    }).unwrap()
}

/*
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::CREATED, Json(user))
}


// the input to our `create_user` handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
*/