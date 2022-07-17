pub mod swapi;

use std::time::Duration;

use crate::swapi::SwapiClient;

pub struct YodaTaller {
    swapi_client: SwapiClient,
}

#[derive(thiserror::Error, Debug)]
pub enum YodaTallerError {
    /// No person with the given name.
    #[error("Person not found")]
    PersonNotFound,
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
    pub async fn is_taller_than(&self, name: &str) -> Result<bool, YodaTallerError> {
        let yoda_height = 66;
        let characters = self
            .swapi_client
            .people_by_name(name)
            .await
            .map_err(YodaTallerError::UnexpectedError)?;
        let first_match = characters.get(0).ok_or(YodaTallerError::PersonNotFound)?;
        let other_height = first_match.height().unwrap();
        let is_taller = yoda_height > other_height;
        Ok(is_taller)
    }
}
