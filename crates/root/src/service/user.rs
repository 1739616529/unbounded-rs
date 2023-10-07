use axum::{http::StatusCode, routing, Json, Router, extract::Path};

use crate::user::{self, User};

pub fn get_user_router() -> Router {
    let router = Router::new()
        .route("/:userid", routing::post(get_user))
        .route("/all", routing::post(get_user_all))
        .route("/add", routing::post(set_user))
        ;
    Router::new().nest("/user", router)
}



async fn get_user_all() -> Json<Vec<User>> {
    Json(user::get_user_all())
}

async fn get_user(Path(userid): Path<String>) -> Result<Json<User>, StatusCode> {
    let user = user::get_user(&userid);

    if let Some(user) = user {
        return Ok(Json(user.clone()))
    }

    println!("not found user");
    Err(StatusCode::OK)
}


async fn set_user(Json(user): Json<User>) -> StatusCode {

    println!("{:?}", user);

    StatusCode::OK

}
