use sqlx::SqlitePool;

use crate::models::Anime;
use crate::models::AnimeWithEp;

pub async fn get_anime_by_id (mal_id: u32, connection: &SqlitePool) -> Result<Option<Anime>, sqlx::Error> {
    let anime: Option<Anime> = sqlx::query_as("SELECT * FROM animes WHERE mal_id = $1")
        .bind(mal_id)
        .fetch_optional(connection)
        .await?;
    

    if let Some(mut anime) = anime {
        let genres: Vec<(String,)> = sqlx::query_as("SELECT genre FROM genres WHERE anime_id = ?")
            .bind(anime.id)
            .fetch_all(connection)
            .await?;

        let themes: Vec<(String,)> = sqlx::query_as("SELECT theme FROM themes WHERE anime_id = ?")
            .bind(anime.id)
            .fetch_all(connection)
            .await?;
        
        anime.genres = genres.into_iter().map(|g| g.0).collect();
        anime.themes = themes.into_iter().map(|g| g.0).collect();

        Ok(Some(anime))
    } else {
        Ok(anime)
    }
}


pub async fn get_latest_updates (offset: u16, pool: &SqlitePool) -> Result<Vec<AnimeWithEp>, sqlx::Error> {
    let query_string = "
    SELECT
        eps.id as episode_id,
        eps.aired,
        eps.episode_number,
        eps.score,
        eps.anime_id,
        ani.mal_id,
        ani.title,
        ani.localName,
        ani.type
    FROM episodes eps
    JOIN animes ani ON eps.anime_id = ani.id
    WHERE eps.aired = (
        SELECT MAX(aired)
        FROM episodes eps2
        WHERE eps2.anime_id = eps.anime_id
    ) AND 
          eps.episode_number = (
        SELECT MAX(episode_number)
        FROM episodes eps2
        WHERE eps2.anime_id = eps.anime_id
    )
    ORDER BY eps.aired DESC
    LIMIT 20 OFFSET $1
    ";

    let updates: Vec<AnimeWithEp> = sqlx::query_as(&query_string)
        .bind(offset)
        .fetch_all(pool)
        .await?;


    Ok(updates)
}
