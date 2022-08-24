pub mod server;
pub mod settings;
pub mod swapi;

use std::time::Duration;

use anyhow::Context;
use tracing::instrument;

use crate::swapi::SwapiClient;

pub struct YodaTaller {
    swapi_client: SwapiClient,
}

#[derive(thiserror::Error, Debug)]
pub enum YodaTallerError {
    /// The person doesn't have a valid height.
    #[error("Height not found")]
    HeightNotFound(#[source] anyhow::Error),
    /// No person with the given name exists.
    #[error("Person not found")]
    PersonNotFound(#[source] anyhow::Error),
    /// Unexpected error while calling Swapi API.
    #[error("Unexpected error while retrieving person height")]
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
    pub async fn is_taller_than(&self, name: &str) -> Result<bool, YodaTallerError> {
        let yoda_height = 66;
        let characters = self
            .swapi_client
            .people_by_name(name)
            .await
            .map_err(YodaTallerError::UnexpectedError)?;
        let first_match = characters
            .get(0)
            .with_context(|| format!("Person `{name}` not found"))
            .map_err(YodaTallerError::PersonNotFound)?;
        let person_height = &first_match.height;
        tracing::Span::current().record("height", person_height);

        let other_height = person_height
            .parse::<u32>()
            .with_context(|| format!("Height `{person_height}` of `{name}` is invalid"))
            .map_err(YodaTallerError::HeightNotFound)?;
        let is_taller = yoda_height > other_height;
        Ok(is_taller)
    }
}
