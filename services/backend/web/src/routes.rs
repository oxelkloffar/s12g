use rocket::Route;
use rocket_contrib::json::Json;
use uuid::Uuid;
use s12g_mail;

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
    create_node_request: Json<CreateNodeRequest>
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
struct SignupRequest {
    email: String,
}

/*
curl -X PUT \
-H "Content-Type: application/json" \
-d "{\"email\":\"richodemus@gmail.com\"}" \
localhost:8000/api/v1/users/00000000-0000-0000-0000-000000000000
*/
#[put("/api/v1/users/<id>", data = "<signup_request>", format = "json")]
fn signup(
    id: rocket_contrib::uuid::Uuid,
    signup_request: Json<SignupRequest>
) {
    println!("Signup: {}", signup_request.email);
    let email = signup_request.email.clone();
    s12g_mail::send_email(&email);
}

pub fn routes() -> Vec<Route> {
    routes![index, add_node, signup]
}
