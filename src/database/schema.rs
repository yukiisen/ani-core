use std::process;
use std::fs;

use rusqlite::{Connection, Result};

use crate::utils::config::Config;

pub fn initialize (config: &Config) -> Result<()> {
    if fs::exists(&config.database_path).unwrap_or(true) {
        return Ok(());
    }

    let connection = Connection::open(&config.database_path)?;

    let Ok(raw_queries) = fs::read_to_string(&config.database_schema) else {
        eprintln!("no schema file found in {}", config.database_schema);
        eprintln!("The application will exit..");
        process::exit(1);
    };
    
    let queries = raw_queries.split(";");
    
    for query in queries {
        connection.execute(query, ())?;
    }

    Ok(())
}
