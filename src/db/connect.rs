use diesel::prelude::*;
use std::env;

// pub fn establish_connection() -> PgConnection {
//     let platform = Platform::Local;

//     let database_uri = platform.get_database_config().as_diesel_uri();
//     PgConnection::establish(&database_uri).expect(&format!("Error connecting to {}", database_uri))
// }

pub struct DBConfig {
    pub user: String,
    pub password: String,
    pub database: String,
    pub host: String,
    pub port: i32,
}

impl DBConfig {
    pub fn as_uri(self) -> String {
        format!(
            "postgres://{user}:{password}@{host}:{port}/{database}",
            user = self.user,
            password = self.password,
            host = self.host,
            port = self.port,
            database = self.database,
        )
    }
}
