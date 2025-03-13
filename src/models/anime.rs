use serde::Serialize;
use sqlx::FromRow;
use chrono::{ DateTime, Utc };

#[derive(Debug, FromRow)]
#[derive(Serialize)]
pub struct Anime {
    pub id: u32,
    pub mal_id: u32,
    #[sqlx(rename = "localName")]
    pub local_name: String,
    pub title: String,
    pub title_english: Option<String>,
    pub title_japanese: Option<String>,
    pub r#type: String,
    pub source: String,
    pub episodes: u16,
    pub status: String,
    pub aired_from: i64,
    pub aired_to: i64,
    pub duration: String,
    pub rating: String,
    pub score: Option<f32>,
    pub popularity: u16,
    pub rank: Option<u16>,
    pub background: String,
    pub season: Option<String>,
    pub year: Option<u16>,
    pub broadcast_day: Option<String>,
    pub broadcast_time: Option<String>,
    pub studio: Option<String>,
    #[sqlx(skip)]
    pub genres: Vec<String>,
    #[sqlx(skip)]
    pub themes: Vec<String>,
}

impl Anime {
    pub fn get_aired_to (&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.aired_to / 1000, 0).expect("aired_to is not a valid timestamp")
    }

    pub fn get_aired_from (&self) -> DateTime<Utc> {
        DateTime::from_timestamp(self.aired_from / 1000, 0).expect("aired_from is not a valid timestamp")
    }
}

#[derive(FromRow, Serialize, Debug)]
pub struct SearchResult {
    pub id: u32,
    pub mal_id: u32,
    pub title: String,
    #[sqlx(rename = "localName")]
    pub local_name: String,
    pub r#type: String,
    pub rating: String,
    pub score: Option<f32>,
    pub status: String,
    pub episodes: u16,
    #[sqlx(skip)]
    pub genres: Vec<String>,
    #[sqlx(skip)]
    pub themes: Vec<String>,
}
