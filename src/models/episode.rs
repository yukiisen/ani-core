use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;


#[derive(FromRow, Debug, Serialize)]
pub struct AnimeWithEp {
    pub anime_id: u32,
    pub mal_id: u32,
    #[sqlx(rename = "localName")]
    pub local_name: String,
    pub title: String,
    pub episode_id: u32,
    pub aired: String,
    pub episode_number: u16,
    pub score: Option<f32>,
}


#[derive(Debug, FromRow, Serialize)]
pub struct Episode {
    pub id: u32,
    pub episode_number: u16,
    pub title: Option<String>,
    pub title_japanese: Option<String>,
    pub title_english: Option<String>,
    pub aired: Option<String>,
    pub filler: bool,
    pub recap: bool,
    pub score: Option<f32>,
    pub file_name: String,
    pub watched: bool,
    pub user_score: Option<f32>
}

impl Episode {
    pub fn get_aired (&self) -> Option<DateTime<Utc>> {
        if let Some(aired) = &self.aired {
            Some(aired.parse().unwrap())
        } else {
            None
        }
    }
}
