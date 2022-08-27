use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, Extension, Json};
use tracing::{error, warn};

use crate::{YodaTaller, YodaTallerError};

pub async fn health_check() {}

pub async fn taller_than(
    Path(person_name): Path<String>,
    Extension(yoda_taller): Extension<Arc<YodaTaller>>,
) -> Result<Json<YodaTallerResponse>, StatusCode> {
    match yoda_taller.is_taller_than(&person_name).await {
        Ok(taller) => {
            let response = YodaTallerResponse { taller }.into();
            Ok(response)
        }
        Err(e) => {
            log_error(&e);
            Err(e.into())
        }
    }
}

fn log_error(e: &YodaTallerError) {
    match e {
        YodaTallerError::HeightNotFound { .. } | YodaTallerError::PersonNotFound(_) => {
            warn!("{e}")
        }
        YodaTallerError::UnexpectedError(_) => error!("{e}"),
    }
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
            YodaTallerError::HeightNotFound { .. } | YodaTallerError::PersonNotFound(_) => {
                StatusCode::NOT_FOUND
            }
            YodaTallerError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
