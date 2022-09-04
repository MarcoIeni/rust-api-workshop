pub mod server;
pub mod settings;
pub mod swapi;
pub mod traces;

use std::time::Duration;

use tracing::instrument;

use crate::swapi::SwapiClient;

pub struct YodaTaller {
    swapi_client: SwapiClient,
}

#[derive(Debug, serde::Serialize)]
// derive only for tests
#[cfg_attr(feature = "test_fixture", derive(serde::Deserialize, PartialEq, Eq))]
pub struct YodaTallerResult {
    /// Name of the person to compare with Yoda.
    pub person: String,
    /// Whether Yoda is taller than this character or not.
    pub taller: bool,
}

#[derive(thiserror::Error, Debug)]
pub enum YodaTallerError {
    /// The person doesn't have a known or valid height.
    #[error("Person's height is unknown")]
    HeightNotFound,
    /// No person with the given name exists.
    #[error("Person not found")]
    PersonNotFound,
    /// Unexpected error while calling Swapi API.
    #[error("Unexpected error while retrieving person height: {0}")]
    UnexpectedError(#[from] reqwest::Error),
}

impl YodaTaller {
    pub fn new(swapi_base_url: String, swapi_timeout: Duration) -> Self {
        Self {
            swapi_client: SwapiClient::new(swapi_base_url, swapi_timeout),
        }
    }

    /// Is Yoda taller than the person with the given name?
    #[instrument(skip(self), fields(height))]
    pub async fn is_taller_than(&self, name: &str) -> Result<YodaTallerResult, YodaTallerError> {
        let yoda_height = 66;
        let characters = self
            .swapi_client
            .people_by_name(name)
            .await
            .map_err(YodaTallerError::UnexpectedError)?;
        let first_match = characters.get(0).ok_or(YodaTallerError::PersonNotFound)?;
        let person_height = &first_match.height;
        tracing::Span::current().record("height", person_height);

        let other_height = person_height.parse::<u32>().map_err(|e| {
            tracing::warn!("invalid height: {}", e);
            YodaTallerError::HeightNotFound
        })?;
        let response = YodaTallerResult {
            person: first_match.name.clone(),
            taller: yoda_height > other_height,
        };
        Ok(response)
    }
}
