use axum::{
    routing::get,
    Router
};

pub fn get_routes() -> Router {
    Router::new()
        .merge(user_routes())
        .merge(event_routes())
        .route("/", get("Hello World"))
}
