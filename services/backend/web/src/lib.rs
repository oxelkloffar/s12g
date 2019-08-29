#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket::Rocket;

mod routes;

pub fn rocket() {
    build_rocket().launch();
}

fn build_rocket() -> Rocket {
    rocket::ignite().mount("/", routes::routes())
}

#[cfg(test)]
mod tests {
    use rocket::http::Status;
    use rocket::local::Client;

    use super::*;

    #[test]
    fn it_starts() {
        let client = Client::new(build_rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
}