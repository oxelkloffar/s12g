use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

lazy_static! {
    static ref USERS: Arc<Mutex<HashMap<LoginCode, User>>> = {
        let u = Arc::new(Mutex::new(HashMap::new()));
        u
    };
}


pub fn generate_login_code(email: &str) -> LoginCode {
    let code = LoginCode { login_code: String::from("fancy-code") };
    let arc = Arc::clone(&USERS);
    {
        let mut map: MutexGuard<HashMap<LoginCode, User>> = arc.lock().unwrap();
        map.insert(code.clone(), User { email: email.to_string() });
    }
    code
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct LoginCode { pub login_code: String }


pub fn login(code: LoginCode) -> Option<User> {
    let arc = Arc::clone(&USERS);
    let map: MutexGuard<HashMap<LoginCode, User>> = arc.lock().unwrap();
    let user = map.get(&code);
    match user {
        Some(reference) => {
            let owned = reference.clone();
            let email = owned.email.clone();
            Some(User{email})
        },
        None => None,
    }
}

#[derive(Serialize, Deserialize)]
pub struct User {
    email: String,
}
