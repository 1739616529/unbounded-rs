use std::collections::HashMap;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub(crate) uid: String,
    pub(crate) name: String,
    pub(crate) addr: String,
    pub(crate) icon: String,
}

static mut USER_MAP: Lazy<HashMap<String, User>> = Lazy::new(|| HashMap::new());

pub fn get_user(user_key: &str) -> Option<&User> {
    unsafe { USER_MAP.get(user_key) }
}

pub fn set_user(user_key: String, user_info: User) {
    unsafe {
        USER_MAP.insert(user_key, user_info);
    }
}

pub fn get_user_all() -> Vec<User> {
    let user_iter: Vec<User> = unsafe { USER_MAP.values().cloned().collect() };
    user_iter
}
