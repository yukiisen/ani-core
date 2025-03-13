use sqlx::{ Result, SqlitePool };

use crate::models::Episode;

pub async fn get_anime_episodes (mal_id: u32, pool: &SqlitePool) -> Result<Vec<Episode>> {
    let eps = sqlx::query_as("SELECT * FROM episodes WHERE anime_id = (SELECT id FROM animes WHERE mal_id = $1)")
        .bind(mal_id)
        .fetch_all(pool)
        .await?;

    Ok(eps)
}
