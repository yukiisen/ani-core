use crate::models::SearchResult;
use anyhow::{Ok, Result};
use sqlx::SqlitePool;

pub async fn search_anime_by_name (query: &str, pool: &SqlitePool) -> Result<Vec<SearchResult>> {
    let mut rows: Vec<SearchResult> = sqlx::query_as(
        "
        SELECT id, mal_id, title, localName, type, rating, status, episodes, score
        FROM animes WHERE 
        title LIKE CONCAT('%', $1, '%') OR 
        localName LIKE CONCAT('%', $1, '%') OR 
        title_english LIKE CONCAT('%', $1, '%') OR
        title_japanese LIKE CONCAT('%', $1, '%')
        "
    ).bind(query).fetch_all(pool).await?;

    for row in rows.iter_mut() {
        let genres: Vec<(String, )> = sqlx::query_as("SELECT genre FROM genres WHERE anime_id = $1").bind(&row.id).fetch_all(pool).await?;
        let themes: Vec<(String, )> = sqlx::query_as("SELECT theme FROM themes WHERE anime_id = $1").bind(&row.id).fetch_all(pool).await?;
        row.genres = genres.into_iter().map(|g| g.0).collect();
        row.themes = themes.into_iter().map(|g| g.0).collect();
    }

    Ok(rows)
}
