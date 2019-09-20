use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use uuid::Uuid;

lazy_static! {
    static ref USERS: Mutex<HashMap<LoginCode, User>> = Mutex::new(HashMap::new());
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct LoginCode { pub login_code: String }

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub email: String,
    pub user_id: Uuid,
}

pub fn generate_login_code(email: &str) -> LoginCode {
    let code = LoginCode { login_code: String::from("fancy-code") };
    {
        let mut map: MutexGuard<HashMap<LoginCode, User>> = USERS.lock().unwrap();
        map.insert(code.clone(), User {
            email: email.to_string(),
            user_id: Uuid::new_v4(),
        });
    }
    code
}

pub fn login(code: LoginCode) -> Option<User> {
    let map: MutexGuard<HashMap<LoginCode, User>> = USERS.lock().unwrap();
    let user = map.get(&code);
    user.cloned()
}
