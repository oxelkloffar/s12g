use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};

use uuid::Uuid;

lazy_static! {
    static ref LOGIN_CODES: Mutex<HashMap<LoginCode, User>> = Mutex::new(HashMap::new());
    static ref USERS: Mutex<HashMap<Uuid, User>> = Mutex::new(HashMap::new());
}

#[derive(Eq, PartialEq, Hash, Clone)]
pub struct LoginCode { pub login_code: String }

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub email: String,
    pub user_id: Uuid,
}

pub fn get_users() -> Vec<User> {
    let users_map: MutexGuard<HashMap<Uuid, User>> = USERS.lock().unwrap();
    users_map.values()
        .cloned()
        .collect()
}

pub fn generate_login_code(email: &str) -> LoginCode {
    let user = get_or_insert_user_by_email(email);
    let code = LoginCode { login_code: String::from("fancy-code") };
    {
        let mut code_map: MutexGuard<HashMap<LoginCode, User>> = LOGIN_CODES.lock().unwrap();
        code_map.insert(code.clone(), User {
            email: user.email,
            user_id: user.user_id,
        });
    }
    code
}

fn get_or_insert_user_by_email(email: &str) -> User {
    let mut users_map: MutexGuard<HashMap<Uuid, User>> = USERS.lock().unwrap();
    let users = users_map.values()
        .filter(|user| user.email == email)
        .cloned()
        .collect::<Vec<User>>();
    match users.as_slice() {
        [] => {
            let user = User {
                email: email.to_string(),
                user_id: Uuid::new_v4(),
            };
            users_map.insert(user.user_id, user.clone());
            user
        },
        [user] => {
            user.clone()
        },
        [user, ..] => {
            println!("Warning, more than 1 user with email {}", email);
            user.clone()
        },
    }
}

pub fn get_user(id: &Uuid) -> Option<User> {
    let users_map: MutexGuard<HashMap<Uuid, User>> = USERS.lock().unwrap();
    users_map.get(id).cloned()
}

pub fn login(code: &LoginCode) -> Option<User> {
    let map: MutexGuard<HashMap<LoginCode, User>> = LOGIN_CODES.lock().unwrap();
    let user = map.get(code);
    user.cloned()
}
