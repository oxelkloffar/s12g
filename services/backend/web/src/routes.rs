use chrono::{DateTime, Utc};
use rocket::http::{Cookie, Cookies, SameSite};
use rocket::http::Status;
use rocket::response::status::NotFound;
use rocket::Route;
use rocket_contrib::json::Json;
use time::Duration;
use uuid::Uuid;

use s12g_mail;

use crate::session::Session;
use crate::user::user;
use crate::user::user::{LoginCode, User};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Deserialize)]
struct CreateNodeRequest {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Node {
    id: Uuid,
    name: String,
}

/*
curl -X PUT \
-H "Content-Type: application/json" \
-d "{\"name\":\"richo\"}" \
localhost:8000/api/v1/nodes/00000000-0000-0000-0000-000000000000
*/
#[put("/api/v1/nodes/<id>", data = "<create_node_request>", format = "json")]
fn add_node(
    id: rocket_contrib::uuid::Uuid,
    create_node_request: Json<CreateNodeRequest>,
) -> Json<Node> {
    println!("Add node: {} - {}", id, create_node_request.name);
    let id: Uuid = *id; // the * is something with deref coercion
    let name = create_node_request.name.clone();
    Json(Node {
        id,
        name,
    })
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
}

/*
curl -X POST \
-H "Content-Type: application/json" \
-d "{\"email\":\"richodemus@gmail.com\"}" \
localhost:8000/api/v1/users/login
*/
#[post("/api/v1/users/login", data = "<login_request>", format = "json")]
fn login(
    login_request: Json<LoginRequest>
) {
    println!("Login: {}", login_request.email);
    let email = login_request.email.clone();
    let code = user::generate_login_code(&email);
    s12g_mail::send_login_email(&email, &code.login_code);
}

/*
curl localhost:8000/api/v1/users/login-link?code=fancy-code
*/
#[get("/api/v1/users/login-link?<code>")]
fn login_url_clicked(code: String, mut cookies: Cookies) -> Result<Json<Session>, Status> {
    println!("User clicked login link with code {}", code);
    let maybe_user = user::login(LoginCode { login_code: code });
    match maybe_user {
        Some(user) => {
            let session = Session {
                user_id: user.user_id,
                expires: Utc::now() + Duration::weeks(52),
            };
            let cookie_builder = Cookie::build("user", session.to_json())
                .path("/")
                .max_age(Duration::weeks(52));

            // if release/prod
            let cookie_builder = match cfg!(debug_assertions) {
                true => cookie_builder,
                false => cookie_builder
                    .secure(true)
                    .http_only(true)
                    .same_site(SameSite::Strict),
            };

            let cookie = cookie_builder
                .finish();

            cookies.add_private(cookie);
            Result::Ok(Json(session))
        }
        None => Result::Err(Status::Unauthorized)
    }
}

/*
curl localhost:8000/api/v1/users/me
*/
#[get("/api/v1/users/me", format = "json")]
fn get_self(mut cookies: Cookies) -> Result<Json<User>, Status> {
    println!("cookies: {:?}", cookies);
    let maybe_cookie = cookies.get_private("user");
    match maybe_cookie {
        Some(cookie) => {
            let name = cookie.name().to_owned();
            let val = Session::parse(cookie.value());
            let user_id = val.user_id;
            let user = user::get_user(user_id);
            match user {
                Some(user) => Result::Ok(Json(user)),
                None => Result::Err(Status::InternalServerError),
            }
        }
        None => Result::Err(Status::Unauthorized),
    }
}

/*
curl localhost:8000/api/v1/users
*/
#[get("/api/v1/users", format = "json")]
fn get_users() -> Json<Vec<User>> {
    Json(user::get_users())
}

pub fn routes() -> Vec<Route> {
    routes![index, add_node, login, login_url_clicked, get_self, get_users]
}
