use axum::{Router, routing::get};
use dill::CatalogBuilder;

#[derive(Debug)]
pub struct AccessControlService;

pub fn register(builder: &mut CatalogBuilder) {
    builder.add_value(AccessControlService);
}

pub fn router(_catalog: &dill::Catalog) -> anyhow::Result<Router> {
    Ok(Router::new().route("/api/plugins/rbac/health", get(|| async { "ok" })))
}
