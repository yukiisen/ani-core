use sqlx::SqlitePool;

pub async fn get_synopsis (mal_id: u32, lang: &str, pool: &SqlitePool) -> Result<String, sqlx::Error> {
    let lang = if lang == "ar" { "synopsis_ar" } else { "synopsis" };
    let synopsis = sqlx::query_as::<_, (String, )>(
        format!("SELECT {lang} FROM synopsis WHERE anime_id = (SELECT id FROM animes WHERE mal_id = $1)").as_str()
    )
        
        .bind(mal_id)
        .fetch_optional(pool)
        .await?.unwrap_or_default();

    Ok(synopsis.0)
}
