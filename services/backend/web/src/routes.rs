use rocket::Route;
use rocket_contrib::json::Json;
use rocket_contrib::uuid::Uuid;

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
    id: String,
    name: String,
}

/*
curl -X PUT \
-H "Content-Type: application/json" \
-d "{\"name\":\"richo\"}" \
localhost:8000/api/v1/nodes/00000000-0000-0000-0000-000000000000
*/
#[put("/api/v1/nodes/<id>", data = "<create_node_request>", format = "json")]
fn add_node(id: Uuid, create_node_request: Json<CreateNodeRequest>) -> Json<Node> {
    println!("Add node: {} - {}", id, create_node_request.name);
    Json(Node {
        id: id.to_string(),
        name: create_node_request.name.clone(),
    })
}

pub fn routes() -> Vec<Route> {
    routes![index, add_node]
}
