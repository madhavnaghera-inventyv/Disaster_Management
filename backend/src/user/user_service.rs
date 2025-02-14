use axum::{extract::State, http::StatusCode, Json};
use validator::Validate;
use crate::{user::user_structure::User, utils::db::AppState};

use super::user_model::get_users;

pub async fn add_user_service(
    State(state): State<AppState>,
    Json(user): Json<User>,
) -> Result<Json<User>, (StatusCode, String)> {
    if let Err(errors) = user.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            format!("Validation error: {:?}", errors),
        ));
    }

    let users = get_users(axum::extract::State(state)).await;  // You may want to adjust `get_users` to return a result instead of just printing
    Ok(Json(user))
}
