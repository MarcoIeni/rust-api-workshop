use std::sync::Arc;

use axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Serialize;
use tracing::{error, warn};

use crate::{YodaTaller, YodaTallerError};

pub async fn health_check() {}

pub async fn taller_than(
    Path(person_name): Path<String>,
    Extension(yoda_taller): Extension<Arc<YodaTaller>>,
) -> Result<Json<YodaTallerResponse>, YodaTallerError> {
    match yoda_taller.is_taller_than(&person_name).await {
        Ok(taller) => {
            let response = YodaTallerResponse {
                person: person_name,
                taller,
            }
            .into();
            Ok(response)
        }
        Err(e) => {
            log_error(&e);
            Err(e)
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
// derive deserialize only on tests
#[cfg_attr(feature = "test_fixture", derive(serde::Deserialize))]
pub struct YodaTallerResponse {
    pub person: String,
    pub taller: bool,
}

impl IntoResponse for YodaTallerError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_message) = match self {
            YodaTallerError::HeightNotFound { .. } | YodaTallerError::PersonNotFound(_) => {
                (StatusCode::NOT_FOUND, format!("{self}"))
            }
            YodaTallerError::UnexpectedError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "unexpected error".to_string(),
            ),
        };

        (
            status_code,
            Json(ErrorBody {
                error: error_message,
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
// derive deserialize only on tests
#[cfg_attr(feature = "test_fixture", derive(serde::Deserialize))]
pub struct ErrorBody {
    /// Error message
    pub error: String,
}
