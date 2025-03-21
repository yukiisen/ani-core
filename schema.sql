CREATE TABLE IF NOT EXISTS animes (
    id INTEGER PRIMARY KEY,
    mal_id INTEGER UNIQUE,
    localName TEXT UNIQUE NOT NULL,
    title TEXT NOT NULL,
    title_english TEXT,
    title_japanese TEXT,
    type CHAR(20) NOT NULL,
    source CHAR(20)  NULL,
    episodes INTEGER NOT NULL,
    status CHAR(20) NOT NULL,
    aired_from DATETIME  NULL,
    aired_to DATETIME  NULL,
    duration VARCHAR(35)  NULL,
    rating VARCHAR(50)  NULL,
    score FLOAT  NULL,
    popularity INTEGER  NULL,
    `rank` INTEGER,
    background TEXT NULL,
    season CHAR(10)  NULL,
    year INTEGER NULL,
    broadcast_day CHAR(10) NULL,
    broadcast_time CHAR(6) NULL,
    studio VARCHAR(20) NULL
);

CREATE TABLE IF NOT EXISTS user_data (
    anime_id INTEGER NOT NULL UNIQUE,
    favorite BOOLEAN DEFAULT FALSE,
    started_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    finished_at DATETIME NULL,

    FOREIGN KEY (anime_id) REFERENCES animes(id)
);

CREATE TABLE IF NOT EXISTS synopsis (
    anime_id INTEGER NOT NULL UNIQUE,
    synopsis TEXT DEFAULT "",
    synopsis_ar TEXT DEFAULT "",
    FOREIGN KEY (anime_id) REFERENCES animes(id)
);

CREATE TABLE IF NOT EXISTS genres (
    anime_id INTEGER,
    genre CHAR(20) NOT NULL,
    FOREIGN KEY (anime_id) REFERENCES animes(id)
);

CREATE TABLE IF NOT EXISTS themes (
    anime_id INTEGER,
    theme CHAR(20) NOT NULL,
    FOREIGN KEY (anime_id) REFERENCES animes(id)
);

CREATE TABLE IF NOT EXISTS notes (
    id INTEGER PRIMARY KEY,
    anime_id INTEGER NOT NULL,
    `user` TEXT NOT NULL,
    `note` TEXT NOT NULL,
    FOREIGN KEY (anime_id) REFERENCES animes(id)
);

CREATE TABLE IF NOT EXISTS episodes (
    id INTEGER PRIMARY KEY,
    anime_id INTEGER NOT NULL,
    episode_number INTEGER NOT NULL,
    title TEXT,
    title_japanese TEXT,
    title_english TEXT,
    aired DATETIME,
    filler BOOLEAN DEFAULT FALSE,
    recap BOOLEAN DEFAULT FALSE,
    score FLOAT,
    file_name TEXT,
    watched BOOLEAN DEFAULT FALSE,
    user_score FLOAT,
    FOREIGN KEY (anime_id) REFERENCES animes(id),
    UNIQUE(anime_id, episode_number)
);

CREATE TABLE IF NOT EXISTS recommendations (
    id INTEGER PRIMARY KEY,
    anime_id INTEGER,
    recommended_mal_id INTEGER,
    recommended_image TEXT,
    recommended_title TEXT,
    votes INTEGER,
    
    FOREIGN KEY (anime_id) REFERENCES animes(id)
);

CREATE TABLE IF NOT EXISTS relations (
    id INTEGER PRIMARY KEY,
    anime_id INTEGER,
    related_id INTEGER NULL,
    related_mal_id INTEGER NOT NULL,
    relation TEXT,
    FOREIGN KEY (anime_id) REFERENCES animes(id)
);

CREATE TABLE IF NOT EXISTS user_lists (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    `default` BOOLEAN DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS list_entries (
    id INTEGER PRIMARY KEY,
    list_id INTEGER,
    anime_id INTEGER,
    added DATETIME DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY (list_id) REFERENCES user_lists(id),
    FOREIGN KEY (anime_id) REFERENCES animes(id)
);

CREATE INDEX idx_anime_mal_id ON animes(mal_id);
CREATE INDEX idx_episodes_anime_id ON episodes(anime_id);
CREATE INDEX idx_list_entries_list_id ON list_entries(list_id);
CREATE INDEX idx_list_entries_anime_id ON list_entries(anime_id);
CREATE INDEX idx_relations_anime_id ON relations(anime_id)
