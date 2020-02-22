use crate::db::connect::DBConfig;
use snafu::{ResultExt, Snafu};
use std::env;
use std::error::Error;
use std::fmt;

pub enum Platform {
    Local,
    Lambda,
}

#[derive(Snafu)]
enum PlatformError {
    #[snafu(display("{} must be set: {}", name, source))]
    ReadEnvVar {
        source: std::env::VarError,
        name: String,
    },
    #[snafu(display("Failed to parse port \"{}\" as a number: {}", port, source))]
    PortParseError {
        source: std::num::ParseIntError,
        port: String,
    },
}

// redirect to Display so main understands it
impl fmt::Debug for PlatformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Platform {
    pub fn get_database_config(&self) -> Result<DBConfig, Box<dyn Error>> {
        {
            let get_config_string = match self {
                Self::Local => |name| env::var(name).context(ReadEnvVar { name }),
                _ => unimplemented!(),
            };

            let user = get_config_string("POSTGRES_USER")?;
            let secret = get_config_string("POSTGRES_SECRET")?;
            let host = get_config_string("POSTGRES_HOST")?;
            let port = get_config_string("POSTGRES_PORT")?;
            let port = str::parse(&port).context(PortParseError { port })?;
            let database = get_config_string("POSTGRES_DATABASE")?;
            Ok(DBConfig {
                user,
                secret,
                host,
                port,
                database,
            })
        }
    }
}
