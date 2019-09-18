use rocket::Route;
use rocket_contrib::json::Json;
use uuid::Uuid;

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
fn login_url_clicked(code: String) -> Json<Option<User>> {
    println!("User clicked login link with code {}", code);
    let user = user::login(LoginCode { login_code: code });
    Json(user)
}

pub fn routes() -> Vec<Route> {
    routes![index, add_node, login, login_url_clicked]
}
