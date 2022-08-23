use std::sync::Arc;

use axum::{extract::Path, Extension, Json};

use crate::YodaTaller;

pub async fn health_check() {}

pub async fn taller_than(
    Path(person_name): Path<String>,
    Extension(yoda_taller): Extension<Arc<YodaTaller>>,
) -> Json<YodaTallerResponse> {
    let taller = yoda_taller.is_taller_than(&person_name).await.unwrap();
    YodaTallerResponse { taller }.into()
}

#[derive(Debug, serde::Serialize)]
// derive serialize only on tests
#[cfg_attr(feature = "test_fixture", derive(serde::Deserialize))]
pub struct YodaTallerResponse {
    pub taller: bool,
}
