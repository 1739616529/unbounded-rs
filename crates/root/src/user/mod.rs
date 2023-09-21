use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug)]
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
