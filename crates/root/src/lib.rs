use user::User;

pub mod net;
pub mod service;
pub mod user;
pub mod config;


pub async fn init() {
    let user_info = User {
        name: "name".to_string(),
        uid: "uid".to_string(),
        icon: "icon".to_string(),
        addr: "addr".to_string(),
    };

    user::set_user(user_info.uid.clone(), user_info);

    if let Some(user) = user::get_user("uid") {
        println!("{:?}", user)
    }
}
