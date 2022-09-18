use {
    crate::taller::{YodaTaller, YodaTallerError, YodaTallerOutcome},
    axum::{extract::Path, http::StatusCode, response::IntoResponse, Extension, Json},
    serde::Serialize,
    std::sync::Arc,
    tracing::{error, warn},
};

pub async fn health_check() {}

#[derive(Debug, serde::Serialize)]
// derive only for tests
#[cfg_attr(feature = "test_fixture", derive(serde::Deserialize, PartialEq, Eq))]
pub struct YodaTallerResponse {
    /// Name to identify a person.
    /// Parameter originally sent from the user.
    pub query: String,
    /// Query result.
    #[serde(flatten)]
    pub result: YodaTallerOutcome,
}

#[derive(Debug)]
pub struct YodaTallerResponseError {
    /// Name to identify a person.
    /// Parameter originally sent from the user.
    pub query: String,
    /// Query error.
    pub error: YodaTallerError,
}

pub async fn taller_than(
    Path(person_name): Path<String>,
    Extension(yoda_taller): Extension<Arc<YodaTaller>>,
) -> Result<Json<YodaTallerResponse>, YodaTallerResponseError> {
    match yoda_taller.is_taller_than(&person_name).await {
        Ok(result) => {
            let json_response = YodaTallerResponse {
                query: person_name,
                result,
            }
            .into();
            Ok(json_response)
        }
        Err(e) => {
            log_error(&e);
            Err(YodaTallerResponseError {
                query: person_name,
                error: e,
            })
        }
    }
}

fn log_error(e: &YodaTallerError) {
    match e {
        YodaTallerError::HeightNotFound | YodaTallerError::PersonNotFound => {
            warn!("{e}")
        }
        YodaTallerError::UnexpectedError(_) => error!("{e}"),
    }
}

impl IntoResponse for YodaTallerResponseError {
    fn into_response(self) -> axum::response::Response {
        let (status_code, error_message) = match self.error {
            YodaTallerError::HeightNotFound | YodaTallerError::PersonNotFound => {
                (StatusCode::NOT_FOUND, format!("{}", self.error))
            }
            YodaTallerError::UnexpectedError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unexpected error".to_string(),
            ),
        };

        (
            status_code,
            Json(ErrorBody {
                query: self.query,
                error: error_message,
            }),
        )
            .into_response()
    }
}

#[derive(Serialize)]
// derive deserialize only on tests
#[cfg_attr(
    feature = "test_fixture",
    derive(serde::Deserialize, Debug, PartialEq, Eq)
)]
pub struct ErrorBody {
    pub query: String,
    /// Error message
    pub error: String,
}
