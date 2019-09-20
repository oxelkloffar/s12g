use rocket::Route;
use rocket::http::{Cookie, Cookies, SameSite};
use rocket_contrib::json::Json;
use uuid::Uuid;
use time::Duration;

use s12g_mail;

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
fn login_url_clicked(code: String, mut cookies: Cookies) -> Json<Option<User>> {
    println!("User clicked login link with code {}", code);
    let user = user::login(LoginCode { login_code: code });
    if let Some(u) = &user {
        let cookie = Cookie::build("name", "value")
            .path("/")
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Strict)
            .max_age(Duration::weeks(52))
            .finish();
        cookies.add(cookie);
    }
    Json(user)
}

/*
curl -X POST -v --cookie cookies --cookie-jar cookies http://localhost:8000/api/v1/cookie && curl -v --cookie cookies --cookie-jar cookies http://localhost:8000/api/v1/cookie
*/

#[post("/api/v1/cookie")]
fn set_cookies(mut cookies: Cookies) {
    let cookie = Cookie::build("name", "this is a secret message, not to be visible in headers")
        .path("/")
//        .secure(true)
//        .http_only(true)
//        .same_site(SameSite::Strict)
        .max_age(Duration::weeks(52))
        .finish();
    cookies.add_private(cookie);
}

#[get("/api/v1/cookie")]
fn get_cookies(mut cookies: Cookies) -> Json<String>{
    println!("cookies: {:?}", cookies);
    let cookie = cookies.get_private("name");
    let cookie = cookie.as_ref();
    let name = cookie.unwrap().name().to_owned();
    let val = cookie.unwrap().value().to_owned();
    Json(format!("cookie:{}-{}", name, val))
}

pub fn routes() -> Vec<Route> {
    routes![index, add_node, login, login_url_clicked, set_cookies, get_cookies]
}
