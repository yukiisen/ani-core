use std::process;
use std::fs;
use sqlx::SqlitePool;
use anyhow::Result;

use crate::utils::config::Config;

pub async fn initialize (config: &Config) -> Result<SqlitePool, sqlx::Error> {
    let connection = SqlitePool::connect(&config.database_path).await?;

    if fs::exists(&config.database_path).unwrap_or(true) {
        return Ok(connection);
    }

    let Ok(raw_queries) = fs::read_to_string(&config.database_schema) else {
        eprintln!("no schema file found in {}", config.database_schema);
        eprintln!("The application will exit..");
        process::exit(1);
    };
    
    let queries = raw_queries.split(";");
    
    for query in queries {
        sqlx::query(query).execute(&connection).await?;
    }

    Ok(connection)
}
