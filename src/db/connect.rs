use diesel::prelude::*;
use std::env;

// pub fn establish_connection() -> PgConnection {
//     let platform = Platform::Local;

//     let database_uri = platform.get_database_config().as_diesel_uri();
//     PgConnection::establish(&database_uri).expect(&format!("Error connecting to {}", database_uri))
// }

pub struct DBConfig {
    pub user: String,
    pub secret: String,
    pub database: String,
    pub host: String,
    pub port: i32,
}

impl DBConfig {
    pub fn as_diesel_uri(self) -> String {
        format!(
            "postgres://{user}:{secret}@{host}:{port}/{database}",
            user = self.user,
            secret = self.secret,
            host = self.host,
            port = self.port,
            database = self.database,
        )
    }
}
