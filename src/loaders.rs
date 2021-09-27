use crate::{database::repository::SharedWookiepediaRepository, schema::filmmaker::Filmmaker};
use async_graphql::dataloader::Loader;
use async_graphql::Error;
use std::collections::HashMap;

pub struct FilmmakerLoader {
    repository: SharedWookiepediaRepository,
}

impl FilmmakerLoader {
    pub fn new(repository: SharedWookiepediaRepository) -> Self {
        FilmmakerLoader { repository }
    }
}

#[async_trait::async_trait]
impl Loader<i64> for FilmmakerLoader {
    type Value = Filmmaker;
    type Error = Error;

    async fn load(&self, keys: &[i64]) -> Result<HashMap<i64, Self::Value>, Self::Error> {
        let filmmakers = self
            .repository
            .list_filmmakers(&Vec::from(keys))
            .await
            .expect("can't load filmmakers");

        Ok(filmmakers
            .iter()
            .map(|filmmaker| (filmmaker.id, Filmmaker::from(filmmaker)))
            .collect::<HashMap<_, _>>())
    }
}
