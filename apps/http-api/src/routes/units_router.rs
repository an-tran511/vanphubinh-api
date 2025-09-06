use crate::app_state::AppState;
use crate::handlers::units::create_unit;
use crate::handlers::units::list_units;
use std::sync::Arc;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub fn init() -> OpenApiRouter<Arc<AppState>> {
    OpenApiRouter::new()
        .routes(routes!(list_units::list_units))
        .routes(routes!(create_unit::create_unit))
}
