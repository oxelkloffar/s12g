use rocket::http::{Cookie, Cookies, SameSite};
use rocket::Route;
use rocket_contrib::json::Json;
use time::Duration;
use uuid::Uuid;

use s12g_mail;

use crate::user::user;
use crate::user::user::{LoginCode, User};
use rocket::response::status::NotFound;
use rocket::http::Status;

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
fn login_url_clicked(code: String, mut cookies: Cookies) -> Result<Json<User>, Status> {
    println!("User clicked login link with code {}", code);
    let user = user::login(LoginCode { login_code: code });
    if user.is_none() {
        return Result::Err(Status::Unauthorized)
    }
    if let Some(u) = &user {
        let cookie_builder = Cookie::build("user", u.email.clone())
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
    }
    Result::Ok(Json(user.unwrap()))
}

/*
curl localhost:8000/api/v1/users/me
*/
#[get("/api/v1/users/me", format = "json")]
fn get_self(mut cookies: Cookies) -> Result<Json<User>, Status> {
    println!("cookies: {:?}", cookies);
    let cookie = cookies.get_private("user");
    if cookie.is_none() {
        return Result::Err(Status::Unauthorized)
    }
    let cookie = cookie.as_ref();
    let name = cookie.unwrap().name().to_owned();
    let val = cookie.unwrap().value().to_owned();
    Result::Ok(Json(User{ email: val }))
}

pub fn routes() -> Vec<Route> {
    routes![index, add_node, login, login_url_clicked, get_self]
}
