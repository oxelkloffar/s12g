use rocket::Route;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn routes() -> Vec<Route> {
    let asd:Vec<Route> = routes![index];
    return asd
}
