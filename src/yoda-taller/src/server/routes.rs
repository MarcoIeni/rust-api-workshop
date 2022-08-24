use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};

use crate::{YodaTaller, YodaTallerError};

pub async fn health_check() {}

pub async fn taller_than(
    Path(person_name): Path<String>,
    Extension(yoda_taller): Extension<Arc<YodaTaller>>,
) -> Result<Json<YodaTallerResponse>, StatusCode> {
    let taller = yoda_taller.is_taller_than(&person_name).await?;
    let response = YodaTallerResponse { taller }.into();
    Ok(response)
}

#[derive(Debug, serde::Serialize)]
// derive serialize only on tests
#[cfg_attr(feature = "test_fixture", derive(serde::Deserialize))]
pub struct YodaTallerResponse {
    pub taller: bool,
}

impl From<YodaTallerError> for StatusCode {
    fn from(e: YodaTallerError) -> Self {
        match e {
            YodaTallerError::HeightNotFound(_) => StatusCode::NOT_FOUND,
            YodaTallerError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
