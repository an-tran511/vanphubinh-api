use crate::app_state::AppState;
use crate::handlers::product_categories::{create_product_category, list_product_categories};
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn init() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(list_product_categories::list_product_categories))
        .routes(routes!(create_product_category::create_product_category))
}
